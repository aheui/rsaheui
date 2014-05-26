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

pub enum InterpreterDirection {
    Down,
    Up,
    Right,
    Left,
}

pub enum InstructionData {
    Hangeul(hangeul::ConcreteSyllable),
    Character(char),
    Virtual,
}

impl InstructionData {
    pub fn from_char(c: char) -> InstructionData {
        match hangeul::ConcreteSyllable::from_char(c) {
            Some(syllable) => Hangeul(syllable),
            None => Character(c),
        }
    }
}

pub enum InstructionMovement {
    RegularMovement(InterpreterDirection, int, int),
    AllowHorizontalMovement,
    AllowVerticalMovement,
    DisallowMovement,
    KeepCurrentMovement,
    WallMovement(InterpreterDirection, int),
}

pub enum InstructionOperation {
    NoOperation,
    PushConstantOperation(int),
    PushDuplicationOperation,
    PushIntegerInputOperation,
    PushCharInputOperation,
    BinaryOperation(fn(int, int) -> int),
    PopOperation,
    PrintIntegerOperation,
    PrintCharOperation,
    SwapOperation,
    ChangeStorageOperation(uint),
    MoveToStorageOperation(uint),
    CompareOperation,
    BranchOperation,
    HaltOperation,
}

fn operation_digeut(v1: int, v2: int) -> int { v2 + v1 }
fn operation_ssang_digeut(v1: int, v2: int) -> int { v2 * v1 }
fn operation_tieut(v1: int, v2: int) -> int { v2 - v1 }
fn operation_nieun(v1: int, v2: int) -> int { v2 / v1 }
fn operation_rieul(v1: int, v2: int) -> int { v2 % v1 }

pub struct Instruction {
    data: InstructionData,
    operation: InstructionOperation,
    move: InstructionMovement,
}
static right_wall_instruction: Instruction
    = Instruction { data: Virtual, operation: NoOperation, move: WallMovement(Right, 2) };
static down_wall_instruction: Instruction
    = Instruction { data: Virtual, operation: NoOperation, move: WallMovement(Down, 2) };

impl Instruction {
    pub fn from_data(i: InstructionData) -> Instruction {
        let operation = match i {
            Hangeul(c) => match c.initial() {
                hangeul::InitialBieup => match c.final0() {
                    hangeul::FinalIeung => PushIntegerInputOperation,
                    hangeul::FinalHieut => PushCharInputOperation,
                    final => PushConstantOperation(final_draw_counts[final as uint]),
                },
                hangeul::InitialSsangBieup => PushDuplicationOperation,
                hangeul::InitialPieup => SwapOperation,
                hangeul::InitialDigeut => BinaryOperation(operation_digeut),
                hangeul::InitialSsangDigeut => BinaryOperation(operation_ssang_digeut),
                hangeul::InitialTieut => BinaryOperation(operation_tieut),
                hangeul::InitialNieun => BinaryOperation(operation_nieun),
                hangeul::InitialRieul => BinaryOperation(operation_rieul),
                hangeul::InitialIeung => { NoOperation },
                hangeul::InitialMieum => match c.final0() {
                    hangeul::FinalIeung => PrintIntegerOperation,
                    hangeul::FinalHieut => PrintCharOperation,
                    _ => PopOperation,
                },
                hangeul::InitialSiot => ChangeStorageOperation(c.final0() as uint),
                hangeul::InitialSsangSiot => MoveToStorageOperation(c.final0() as uint),
                hangeul::InitialJieut => CompareOperation,
                hangeul::InitialChieut => BranchOperation,
                hangeul::InitialHieut => HaltOperation,
                _ => NoOperation,
            },
            _ => NoOperation,
        };
        let move = match i {
            Hangeul(c) => match c.peak() {
                hangeul::A => { RegularMovement(Right, 0, 1) }
                hangeul::Ya => { RegularMovement(Right, 0, 2) }
                hangeul::Eo => { RegularMovement(Left, 0, -1) }
                hangeul::Yeo => { RegularMovement(Left, 0, -2) }
                hangeul::O => { RegularMovement(Up, -1, 0) }
                hangeul::Yo => { RegularMovement(Up, -2, 0) }
                hangeul::U => { RegularMovement(Down, 1, 0) }
                hangeul::Yu => { RegularMovement(Down, 2, 0) }
                hangeul::Eu => { AllowHorizontalMovement }
                hangeul::I => { AllowVerticalMovement }
                hangeul::Ui => { DisallowMovement }
                _ => { KeepCurrentMovement }
            },
            _ => KeepCurrentMovement,
        };
        Instruction { data: i, operation: operation, move: move }
    }

    pub fn from_wall_data(direction: InterpreterDirection, value: int) -> Instruction {
        Instruction {
            data: Virtual,
            operation: NoOperation,
            move: WallMovement(direction, value)
        }
    }

    pub fn from_char(c: char) -> Instruction {
        return Instruction::from_data(InstructionData::from_char(c));
    }

    pub fn hangeul(&self) -> hangeul::ConcreteSyllable {
        match self.data {
            Hangeul(syllable) => syllable,
            _ => { assert!(false); fail!("") }
        }
    }
}

pub struct Source {
    map: Vec<Vec<Instruction>>,
}

impl Source {
    pub fn from_str(s: &str) -> Source {
        let mut obj = Source {
            map: Vec::new(),
        };
        obj._parse(s);

        obj
    }

    pub fn _parse(&mut self, s: &str) {
        self.map.push(Vec::new());

        for c in s.as_slice().chars() {
            if c == '\n' {
                self.map.push(Vec::new());
            } else {
                let inst = Instruction::from_data(match hangeul::ConcreteSyllable::from_char(c) {
                    Some(syllable) => Hangeul(syllable),
                    None => Character(c),
                });
                self.map.mut_last().unwrap().push(inst);
            }
        }

        let mut max_col_len = 0;
        let row_len = self.map.len();
        for row in self.map.mut_iter() {
            let len = row.len();
            max_col_len = std::cmp::max(max_col_len, len);
            row.insert(0, Instruction::from_wall_data(Left, len as int + 1));
            row.insert(0, Instruction::from_wall_data(Left, len as int + 1));
        }

        {
            let mut h1 = Vec::new();
            let mut h2 = Vec::new();
            let mut t1 = Vec::new();
            let mut t2 = Vec::new();
            for _ in range(0, max_col_len + 2) {
                h1.push(Instruction::from_wall_data(Up, row_len as int + 1));
                h2.push(Instruction::from_wall_data(Up, row_len as int + 1));
                t1.push(down_wall_instruction);
                t2.push(down_wall_instruction);
            }
            self.map.insert(0, h1);
            self.map.insert(0, h2);
            self.map.push(t1);
            self.map.push(t2);
        }
    }

    fn _get(&self, pos: (int, int)) -> Instruction {
        match pos {
            (ridx, cidx) => {
                let row = self.map.get(ridx as uint);
                let inst = if cidx >= row.len() as int {
                    right_wall_instruction
                } else {
                    *row.get(cidx as uint)
                };
                // println!("{},{} inst: {:?}", ridx, cidx, inst);
                inst
            }
        }
    }

    pub fn get(&self, pos: (int, int)) -> Instruction {
        match pos {
            (ridx, cidx) => self._get((ridx + 2, cidx + 2))
        }
    }
}

pub trait Storage {
    fn len(&self) -> uint;
    fn put(&mut self, data: int);
    fn rput(&mut self, data: int);
    fn pick(&mut self) -> Option<int>;
    fn peek(&self) -> Option<int>;
    fn swap(&mut self) -> bool {
        if self.len() >= 2 {
            let v1 = self.pick().unwrap();
            let v2 = self.pick().unwrap();
            self.rput(v1);
            self.rput(v2);
            true
        } else {
            false
        }
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
            self.vec.unshift(data);
        } else {
            self.vec.push(data);
        }
    }

    fn pick(&mut self) -> Option<int> {
        if self.is_queue {
            self.vec.shift()
        } else {
            self.vec.pop()
        }
    }

    fn peek(&self) -> Option<int> {
        if self.is_queue {
            if !self.vec.is_empty() {
                Some(*self.vec.get(0))
            } else {
                None
            }
        } else {
            match self.vec.last() {
                Some(v) => Some(*v),
                None => None,
            }
        }
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
    counter: (int, int),
    last_move: (int, int),
    direction: InterpreterDirection,
    out: std::io::LineBufferedWriter<std::io::stdio::StdWriter>,
}

pub static final_draw_counts: [int, ..28] = [0, 2, 4, 4, 2, 5, 5, 3, 5, 7, 9, 9, 7, 9, 9, 8, 4, 4, 6, 2, 4, -1, 3, 4, 3, 4, 4, -1];

impl Interpreter {
    pub fn new(source: Source) -> Interpreter {
        let mut obj = Interpreter {
            source: source,
            storages: Vec::new(),
            storage_index: 0,
            counter: (2, 2),
            last_move: (1, 0),
            direction: Down,
            out: std::io::stdio::stdout(),
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

    pub fn counter(&self) -> (int, int) {
        match self.counter {
            (row, col) => (row - 2, col - 2)
        }
    }

    pub fn storage<'a>(&'a mut self) -> &'a mut Storage {
         let storage: &mut Storage = self.storages.get_mut(self.storage_index);
         storage
    }

    pub fn instruct(&mut self, instruction: &Instruction) -> bool {
        let mut branch: bool = false;
        match instruction.operation {
            PushConstantOperation(v) => {
                let s = self.storage();
                s.put(v);
            }
            BinaryOperation(op) => {
                let s = self.storage();
                if s.len() >= 2 {
                    let v1 = s.pick().unwrap();
                    let v2 = s.pick().unwrap();
                    let r = op(v1, v2);
                    s.put(r);
                } else {
                    branch = true;
                }
            }
            PrintIntegerOperation => {
                let v = self.storage().pick();
                match v {
                    Some(v) => {
                        let _ = self.out.write_int(v);
                    }
                    None => {
                        branch = true;
                    }
                }
            }
            PrintCharOperation => {
                let v = self.storage().pick();
                match v {
                    Some(v) => {
                        let c = std::char::from_u32(v as u32);
                        let _ = self.out.write_char(c.unwrap());
                    }
                    None => {
                        branch = true;
                    }
                }
            }
            PopOperation => {
                let v = self.storage().pick();
                match v {
                    None => {
                        branch = true;
                    }
                    _ => { }
                }
            }
            PushDuplicationOperation => {
                let s = self.storage();
                let v = s.peek();
                match v {
                    Some(v) => {
                        s.put(v);
                    }
                    None => {
                        branch = true;
                    }
                }
            }
            SwapOperation => {
                let s = self.storage();
                if !s.swap() {
                    branch = true;
                }
            }
            MoveToStorageOperation(index) => {
                let v = self.storage().pick();
                match v {
                    Some(v) => {
                        self.storage_index = index;
                        self.storage().put(v);
                    }
                    None => {
                        branch = true;
                    }
                }
            }
            ChangeStorageOperation(index) => {
                self.storage_index = index;
            }
            CompareOperation => {
                let s = self.storage();
                if s.len() >= 2 {
                    let v1 = s.pick().unwrap();
                    let v2 = s.pick().unwrap();
                    s.put(if v2 >= v1 { 1 } else { 0 });
                } else {
                    branch = true;
                }
            }
            BranchOperation => {
                match self.storage().pick() {
                    Some(v) if v == 0 => {
                        branch = true;
                    }
                    Some(_) => { }
                    None => {
                        branch = true;
                    }
                }
            }
            NoOperation => { }
            PushIntegerInputOperation => {
                let mut reader = std::io::stdin();
                let line = reader.read_line().unwrap();
                let num: int = from_str(line).unwrap();
                self.storage().put(num);
            }
            PushCharInputOperation => {
                let mut reader = std::io::stdin();
                let chr = reader.read_char().unwrap();
                self.storage().put(chr as int);
            }
            HaltOperation => {
                //pringln!("halt! {:?}", syllable);
                return true;
            }
        };
        let mut direction_move = match instruction.move {
            RegularMovement(new_direction, row, col) => {
                (new_direction, (row, col))
            },
            AllowHorizontalMovement => match self.direction {
                Right | Left => (self.direction, self.last_move),
                Up => (Down, (1, 0)),
                Down => (Up, (-1, 0)),
            },
            AllowVerticalMovement =>  match self.direction {
                Up | Down => (self.direction, self.last_move),
                Right => (Left, (0, -1)),
                Left => (Right, (1, 0)),
            },
            DisallowMovement => {
                (
                    match self.direction {
                        Right => Left,
                        Left => Right,
                        Up => Down,
                        Down => Up,
                    },
                    match self.last_move {
                        (row, col) => (-row, -col)
                    }
                )
            },
            KeepCurrentMovement => {
                (self.direction, self.last_move)
            },
            WallMovement(direction, value) => {
                match (self.direction, direction) {
                    (Up, Up) => {
                        self.counter = match self.counter {
                            (_, col) => (value + 1, col),
                        }
                    }
                    (Down, Down) => {
                        self.counter = match self.counter {
                            (_, col) => (value - 1, col),
                        }
                    }
                    (Right, Right) => {
                        self.counter = match self.counter {
                            (row, _) => (row, value - 1),
                        }
                    }
                    (Left, Left) => {
                        self.counter = match self.counter {
                            (row, _) => (row, value + 1),
                        }
                    }
                    _ => { }
                }
                (self.direction, self.last_move)
            }
        };

        if branch {
            direction_move = match direction_move {
                (direction, (row, col)) => (
                    match direction {
                        Right => Left,
                        Left => Right,
                        Up => Down,
                        Down => Up,
                    },
                    (-row, -col)
                )
            }
        }

        match direction_move {
            (direction, (row_diff, col_diff)) => {
                self.direction = direction;
                self.counter = match self.counter {
                    (row, col) => (row + row_diff, col + col_diff)
                };
                self.last_move = (row_diff, col_diff);
            }
        };
        false
    }

    pub fn step(&mut self) -> bool {
        let syllable = match self.counter {
            (row, col) => self.source._get((row, col))
        };
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
    interpreter.instruct(&InstructionData::from_char('아'));
    assert_eq!(interpreter.counter, (0, 1));
    interpreter.instruct(&InstructionData::from_char('희'));
    assert_eq!(interpreter.counter, (0, 1));
    {
        let source = Source::new("아희");
        let mut interpreter = Interpreter::new(source);
        interpreter.execute();
    }
}
