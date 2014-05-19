#![license="BSD simplified"]
#![feature(macro_rules)]

extern crate hangeul;
extern crate aheui;

macro_rules! check_final_draw_counts(
    ($final:expr, $count:expr) => (
        assert_eq!(aheui::final_draw_counts[$final as uint], $count);
    )
)

#[cfg(test)]
mod tests {
    use hangeul;
    use aheui;
    use aheui::{Instruction, Source, Interpreter};

    #[test]
    pub fn test_source() {
        let s = Source::new("아희\n밯망희");
        assert_eq!(s.get(0, 0).hangeul().char(), '아');
        assert_eq!(s.get(0, 1).hangeul().char(), '희');
        assert_eq!(s.get(1, 0).hangeul().char(), '밯');
        assert_eq!(s.get(1, 2).hangeul().char(), '희');
    }

    #[test]
    pub fn test_final_draw_counts() {
        check_final_draw_counts!(hangeul::FinalBlank, 0);
        check_final_draw_counts!(hangeul::FinalGiyeok, 2);
        check_final_draw_counts!(hangeul::FinalGiyeokSiot, 4);
        check_final_draw_counts!(hangeul::FinalRieul, 5);
        check_final_draw_counts!(hangeul::FinalRieulBieup, 9);
        check_final_draw_counts!(hangeul::FinalRieulTieut, 9);
        check_final_draw_counts!(hangeul::FinalChieut, 4);
    }

    #[test]
    pub fn test_interpreter() {
        let mut interpreter = Interpreter::new(Source::new(""));
        assert_eq!(interpreter.counter(), (0, 0));
        interpreter.instruct(&Instruction::from_char('아'));
        assert_eq!(interpreter.counter(), (0, 1));
        interpreter.instruct(&Instruction::from_char('희'));
        assert_eq!(interpreter.counter(), (0, 1));
        {
            let source = Source::new("아희");
            let mut interpreter = Interpreter::new(source);
            interpreter.execute();
        }
    }

    #[test]
    pub fn test_helloworld() {
        let source = Source::new("밤밣따빠밣밟따뿌\n빠맣파빨받밤뚜뭏\n돋밬탕빠맣붏두붇\n볻뫃박발뚷투뭏붖\n뫃도뫃희멓뭏뭏붘\n뫃봌토범더벌뿌뚜\n뽑뽀멓멓더벓뻐뚠\n뽀덩벐멓뻐덕더벅");
        let mut it = Interpreter::new(source);
        assert_eq!(it.counter(), (0, 0));
        assert_eq!(it.storage().len(), 0);
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 1));
        assert_eq!(it.storage().len(), 1);
        assert_eq!(it.storage().peek(), 4);
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 2));
        assert_eq!(it.storage().len(), 2);
        assert_eq!(it.storage().peek(), 8);
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 3));
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 4));
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 5));
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 6));
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 7));
        assert!(!it.step());
        assert_eq!(it.counter(), (1, 7));
    }
}
