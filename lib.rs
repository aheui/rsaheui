#![crate_id="aheui"]
#![crate_type="lib"]
#![license="BSD simplified"]
#![feature(macro_rules)]
#![macro_escape]

extern crate hangeul;

#[macro_export]
macro_rules! printerr(
    ($fmt:expr $($arg:tt)*) => (
        let _ = writeln!(&mut std::io::stderr(), $fmt $($arg)*);
    );
)

pub enum Instruction {
    Hangeul(hangeul::Syllable),
    Character(char),
}

impl Instruction {
    pub fn from_char(c: char) -> Instruction {
        match hangeul::Syllable::from_char(c) {
            Some(syllable) => Hangeul(syllable),
            None => Character(c),
        }
    }

    pub fn hangeul(&self) -> hangeul::Syllable {
        match *self {
            Hangeul(syllable) => syllable,
            _ => { assert!(false); fail!("") }
        }
    }
}

pub struct Source {
    instructions: Vec<Instruction>,
    tails: Vec<uint>,
}

impl Source {
    pub fn new(s: &str) -> Source {
        let vec = s.as_slice().chars().map(
            |c| match hangeul::Syllable::from_char(c) {
                Some(syllable) => Hangeul(syllable),
                None => Character(c),
            }
        ).collect();
        let mut obj = Source { instructions: vec, tails: Vec::new() };
        obj._parse();
        obj
    }

    pub fn _parse(&mut self) {
        if self.instructions.len() == 0 {
            self.tails.push(0);
            return
        }
        for (i, &inst) in self.instructions.iter().enumerate() {
            match inst {
                Character('\n') => {
                    self.tails.push(i + 1);
                }
                _ => { }
            }
        }
        if self.tails.len() == 0 || *self.tails.last().unwrap() != self.instructions.len() {
            self.tails.push(self.instructions.len());
        }
    }

    pub fn get(&self, row: uint, col: uint) -> Instruction {
        let pos = col + (if row == 0 {
            0
        } else {
            *self.tails.as_slice().get(row - 1).unwrap()
        });
        //pringln!("pos: {} / col: {} // row: {} / len: {}", pos, col, row, self.instructions.len());
        assert!(col < self.tails.as_slice()[row]);
        *self.instructions.as_slice().get(pos).unwrap()
    }
}

#[test]
pub fn test_source() {
    let mut s = Source::new("아희\n밯망희");
    assert_eq!(s.get(0, 0).hangeul().char(), '아');
    assert_eq!(s.get(0, 1).hangeul().char(), '희');
    assert_eq!(s.get(1, 0).hangeul().char(), '밯');
    assert_eq!(s.get(1, 2).hangeul().char(), '희');
}

enum InterpreterDirection {
    Right,
    Left,
    Down,
    Up,
}

pub trait Storage {
    fn len(&self) -> uint;
    fn put(&mut self, data: int);
    fn pick(&mut self) -> int;
    fn rput(&mut self, data: int);
    fn peek(&self) -> int;
    fn swap(&mut self) {
        let v1 = self.pick();
        let v2 = self.pick();
        self.rput(v1);
        self.rput(v2);
    }
}

pub struct TempStorage {
    // temp impl to avoid trait - box problem
    vec: Vec<int>,
    is_queue: bool,
}

impl TempStorage {
    pub fn new(is_queue: bool) -> TempStorage {
        TempStorage { vec: Vec::new(), is_queue: is_queue }
    }
}

impl Storage for TempStorage {
    fn len(&self) -> uint {
        self.vec.len()
    }

    fn put(&mut self, data: int) {
        self.vec.push(data);
    }

    fn rput(&mut self, data: int) {
        if self.is_queue {
            self.vec.insert(0, data);
        } else {
            self.vec.push(data);
        }
    }

    fn pick(&mut self) -> int {
        if self.is_queue {
            self.vec.remove(0).unwrap()
        } else {
            self.vec.pop().unwrap()
        }
    }

    fn peek(&self) -> int {
        let v = if self.is_queue {
            self.vec.get(0)
        } else {
            self.vec.last().unwrap()
        };
        *v
    }
}

/*
pub struct Stack {
    vec: Vec<int>,
}

impl Storage for Stack {
    fn put(&mut self, data: int) {
        self.vec.push(data);
    }

    fn rput(&mut self, data: int) {
        self.vec.push(data);
    }

    fn pick(&mut self) -> int {
        self.vec.pop().unwrap()
    }

    fn peek(&self) -> int {
        *self.vec.last().unwrap()
    }
}

#[test]
fn test_stack() {
    let mut stack = Stack { vec: Vec::new() };
    stack.put(1);
    stack.put(2);
    stack.put(3);
    assert_eq!(3, stack.pick());
    assert_eq!(2, stack.pick());
    assert_eq!(1, stack.pick());
}

pub  struct Queue {
    vec: Vec<int>,
}

impl Storage for Queue {
    fn put(&mut self, data: int) {
        self.vec.push(data);
    }

    fn rput(&mut self, data: int) {
        self.vec.insert(0, data);
    }

    fn pick(&mut self) -> int {
        self.vec.remove(0).unwrap()
    }

    fn peek(&self) -> int {
        *self.vec.get(0)
    }
}

#[test]
fn test_queue() {
    let mut queue = Queue { vec: Vec::new() };
    queue.put(1);
    queue.put(2);
    queue.put(3);
    assert_eq!(1, queue.pick());
    assert_eq!(2, queue.pick());
    queue.put(4);
    queue.put(5);
    assert_eq!(3, queue.pick());
    assert_eq!(4, queue.pick());
    assert_eq!(5, queue.pick());
}

pub struct Extension {
    vec: Vec<int>,
}

impl Storage for Extension {
    fn put(&mut self, data: int) {
    }

    fn rput(&mut self, data: int) {
    }

    fn pick(&mut self) -> int {
        0
    }

    fn peek(&self) -> int {
        0
    }
}
*/

pub struct Interpreter {
    source: Source,
    storages: Vec<TempStorage>, // must be array - fixed size
    storage_index: uint,
    counter: (uint, uint),
    direction: InterpreterDirection,
}

pub static final_draw_counts: [uint, ..28] = [0, 2, 4, 4, 2, 5, 5, 3, 5, 7, 9, 9, 7, 9, 9, 8, 4, 4, 6, 2, 4, 0, 3, 4, 3, 4, 4, 0];

impl Interpreter {
    pub fn new(source: Source) -> Interpreter {
        let mut obj = Interpreter {
            source: source,
            storages: Vec::new(),
            storage_index: 0,
            counter: (0, 0),
            direction: Right,
        };
        for x in range(0, hangeul::final0_count) {
            let storage = match x {
                21 => {
                    TempStorage::new(true)
                }
                27 => {
                    TempStorage::new(true)
                }
                _ => {
                    TempStorage::new(false)
                }
            };
            obj.storages.push(storage);
        }
        return obj;
    }

    pub fn counter(&self) -> (uint, uint) {
        self.counter
    }

    pub fn storage<'a>(&'a mut self) -> &'a mut Storage {
         let storage: &mut Storage = self.storages.get_mut(self.storage_index);
         storage
    }

    pub fn instruct(&mut self, instruction: &Instruction) -> bool {
        let mut direction: InterpreterDirection = self.direction;
        let mut move: int = 1;
        match *instruction {
            Hangeul(syllable) => {
                //pringln!("instruction: {:?} {:?} {:?}", syllable.initial(), syllable.peak(), syllable.final());
                let initial = syllable.initial();
                let _ = match initial {
                    hangeul::InitialIeung => {
                        false
                    }
                    hangeul::InitialDigeut | hangeul::InitialSsangDigeut | hangeul::InitialTieut |
                    hangeul::InitialNieun | hangeul::InitialRieul => {
                        let s = self.storage();
                        let v1 = s.pick();
                        let v2 = s.pick();
                        let r = match initial {
                            hangeul::InitialDigeut => v2 + v1,
                            hangeul::InitialSsangDigeut => v2 * v1,
                            hangeul::InitialTieut => v2 - v1,
                            hangeul::InitialNieun => v2 / v1,
                            hangeul::InitialRieul => v2 % v1,
                            _ => { assert!(false); 0 } // impossible
                        };
                        s.put(r);
                        true
                    }
                    hangeul::InitialMieum => {
                        let s = self.storage();
                        let v = s.pick();
                        match syllable.final() {
                            hangeul::FinalIeung => {
                                //pringln!("print: {}", v);
                                print!("{}", v);
                            },
                            hangeul::FinalHieut => {
                                //pringln!("print: {} as char", v);
                                print!("{}", std::char::from_u32(v as u32).unwrap());
                            },
                            _ => { },
                        }
                        true
                    }
                    hangeul::InitialBieup => {
                        let s = self.storage();
                        let c = syllable.final();
                        let v = match c {
                            hangeul::FinalIeung => {
                                let mut reader = std::io::stdin();
                                let line = reader.read_line().unwrap();
                                let num: int = from_str(line).unwrap();
                                num
                            },
                            hangeul::FinalHieut => {
                                let mut reader = std::io::stdin();
                                let chr = reader.read_char().unwrap();
                                chr as int
                            },
                            _ => { final_draw_counts[c as uint] as int },
                        };
                        s.put(v);
                        true
                    }
                    hangeul::InitialSsangBieup => {
                        let s = self.storage();
                        let v = s.peek();
                        s.put(v);
                        true
                    }
                    hangeul::InitialPieup => {
                        let s = self.storage();
                        s.swap();
                        true
                    }
                    hangeul::InitialSiot => {
                        self.storage_index = syllable.final() as uint;
                        true
                    }
                    hangeul::InitialSsangSiot => {
                        let v = self.storage().pick();
                        self.storage_index = syllable.final() as uint;
                        self.storage().put(v);
                        true
                    }
                    hangeul::InitialJieut => {
                        let s = self.storage();
                        let v1 = s.pick();
                        let v2 = s.pick();
                        if v2 >= v1 {
                            s.put(1)
                        } else {
                            s.put(0)
                        }
                        true
                    }
                    _ => {
                        //pringln!("unhandled consonant: {:?}", initial);
                        false
                    }
                };
                if initial == hangeul::InitialHieut {
                    //pringln!("halt! {:?}", syllable);
                    return true;
                }

                match syllable.peak() {
                    hangeul::A => { direction = Right; }
                    hangeul::Ya => { direction = Right; move = 2; }
                    hangeul::Eo => { direction = Left; }
                    hangeul::Yeo => { direction = Left; move = 2; }
                    hangeul::O => { direction = Up; }
                    hangeul::Yo => { direction = Up; move = 2; }
                    hangeul::U => { direction = Down; }
                    hangeul::Yu => { direction = Down; move = 2; }
                    hangeul::Eu => {
                        direction = match self.direction {
                            Right | Left => self.direction,
                            Up => Down,
                            Down => Up,
                        };
                    }
                    hangeul::I => {
                        direction = match self.direction {
                            Right => Left,
                            Left => Right,
                            Up | Down => self.direction,
                        };
                    }
                    hangeul::Ui => {
                        direction = match self.direction {
                            Right => Left,
                            Left => Right,
                            Up => Down,
                            Down => Up,
                        };
                    }
                    _ => {
                        //pringln!("unhandled peak: {:?}", syllable.peak());
                    }
                }
                if syllable.initial() == hangeul::InitialChieut && self.storage().pick() == 0 {
                    direction = match direction {
                        Right => Left,
                        Left => Right,
                        Up => Down,
                        Down => Up,
                    }
                }
            }
            _ => { }
        };
        let counter_diff = match direction {
            Right => (0, move),
            Left => (0, -move),
            Down => (move, 0),
            Up => (-move, 0),
        };
        self.counter = match self.counter {
            (row, col) => match counter_diff {
                (row_diff, col_diff) => (row + row_diff as uint, col + col_diff as uint)
            }
        };
        false
    }

    pub fn step(&mut self) -> bool {
        let row: uint; let col: uint;
        match self.counter {
            (r, c) => {
                row = r;
                col = c;
            }
        }
        let syllable = self.source.get(row, col);
        self.instruct(&syllable)
    }

    pub fn execute(&mut self) {
        while !self.step() { }
    }

}

#[test]
pub fn test_interpreter() {
    let mut interpreter = Interpreter::new(Source::new(""));
    assert_eq!(interpreter.counter, (0, 0));
    interpreter.instruct(&Instruction::from_char('아'));
    assert_eq!(interpreter.counter, (0, 1));
    interpreter.instruct(&Instruction::from_char('희'));
    assert_eq!(interpreter.counter, (0, 1));
    {
        let source = Source::new("아희");
        let mut interpreter = Interpreter::new(source);
        interpreter.execute();
    }
}