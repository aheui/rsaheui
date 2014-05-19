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
    pub fn test_it() {
        {
            let mut it = Interpreter::new(Source::new(""));
            assert_eq!(it.counter(), (0, 0));
            it.instruct(&Instruction::from_char('아'));
            assert_eq!(it.counter(), (0, 1));
            it.instruct(&Instruction::from_char('희'));
            assert_eq!(it.counter(), (0, 1));
        }
        {
            let source = Source::new("아희");
            let mut it = Interpreter::new(source);
            it.execute();
            assert_eq!(it.counter(), (0, 1));
        }
    }

    #[test]
    pub fn test_peak() {
        let mut it = Interpreter::new(Source::new(""));
        assert_eq!(it.counter(), (0, 0));
        it.instruct(&Instruction::from_char('아'));
        assert_eq!(it.counter(), (0, 1));
        it.instruct(&Instruction::from_char('우'));
        assert_eq!(it.counter(), (1, 1));
        it.instruct(&Instruction::from_char('어'));
        assert_eq!(it.counter(), (1, 0));
        it.instruct(&Instruction::from_char('오'));
        assert_eq!(it.counter(), (0, 0));
        it.instruct(&Instruction::from_char('우'));
        assert_eq!(it.counter(), (1, 0));
        it.instruct(&Instruction::from_char('이'));
        assert_eq!(it.counter(), (2, 0));
        it.instruct(&Instruction::from_char('으'));
        assert_eq!(it.counter(), (1, 0));
        it.instruct(&Instruction::from_char('아'));
        assert_eq!(it.counter(), (1, 1));
        it.instruct(&Instruction::from_char('으'));
        assert_eq!(it.counter(), (1, 2));
        it.instruct(&Instruction::from_char('이'));
        assert_eq!(it.counter(), (1, 1));
        it.instruct(&Instruction::from_char('의'));
        assert_eq!(it.counter(), (1, 2));
    }

    #[test]
    pub fn test_initial() {
        let mut it = Interpreter::new(Source::new(""));
        it.instruct(&Instruction::from_char('바'));
        assert_eq!(it.storage().peek(), 0);
        it.instruct(&Instruction::from_char('반'));
        assert_eq!(it.storage().peek(), 2);
        it.instruct(&Instruction::from_char('밧'));
        assert_eq!(it.storage().peek(), 2);
        it.instruct(&Instruction::from_char('나'));
        assert_eq!(it.storage().peek(), 1);
        it.instruct(&Instruction::from_char('밟'));
        assert_eq!(it.storage().peek(), 9);
        it.instruct(&Instruction::from_char('밭'));
        assert_eq!(it.storage().peek(), 4);
        it.instruct(&Instruction::from_char('다'));
        assert_eq!(it.storage().peek(), 13);
        it.instruct(&Instruction::from_char('밪'));
        assert_eq!(it.storage().peek(), 3);
        it.instruct(&Instruction::from_char('따'));
        assert_eq!(it.storage().peek(), 39);
        it.instruct(&Instruction::from_char('반'));
        assert_eq!(it.storage().peek(), 2);
        it.instruct(&Instruction::from_char('눔'));
        assert_eq!(it.storage().peek(), 19);
        it.instruct(&Instruction::from_char('발'));
        assert_eq!(it.storage().peek(), 5);
        it.instruct(&Instruction::from_char('룸'));
        assert_eq!(it.storage().peek(), 4);
        it.instruct(&Instruction::from_char('밥'));
        assert_eq!(it.storage().peek(), 4);
        it.instruct(&Instruction::from_char('주'));
        assert_eq!(it.storage().peek(), 1);
        it.instruct(&Instruction::from_char('반'));
        assert_eq!(it.storage().peek(), 2);
        it.instruct(&Instruction::from_char('주'));
        assert_eq!(it.storage().peek(), 0);
    }

    #[test]
    pub fn test_chieut() {
        let mut it = Interpreter::new(Source::new(""));
        assert_eq!(it.counter(), (0, 0));
        it.instruct(&Instruction::from_char('반'));
        assert_eq!(it.counter(), (0, 1));
        assert_eq!(it.storage().peek(), 2);
        it.instruct(&Instruction::from_char('찬'));
        assert_eq!(it.counter(), (0, 2));
        assert_eq!(it.storage().len(), 0);
        it.instruct(&Instruction::from_char('바'));
        assert_eq!(it.counter(), (0, 3));
        assert_eq!(it.storage().peek(), 0);
        it.instruct(&Instruction::from_char('쳐'));
        assert_eq!(it.counter(), (0, 5));
        assert_eq!(it.storage().len(), 0);
    }

    #[test]
    pub fn test_queue() {
        let mut it = Interpreter::new(Source::new(""));
        it.instruct(&Instruction::from_char('상'));
        it.instruct(&Instruction::from_char('반'));
        it.instruct(&Instruction::from_char('발'));
        it.instruct(&Instruction::from_char('밞'));
        assert_eq!(it.storage().peek(), 2);
        it.instruct(&Instruction::from_char('팡'));
        assert_eq!(it.storage().peek(), 5);
        it.instruct(&Instruction::from_char('덧'));
        assert_eq!(it.storage().peek(), 9);
        it.instruct(&Instruction::from_char('멍'));
        assert_eq!(it.storage().peek(), 7);
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

    #[test]
    pub fn test_99dan() {
        let source = Source::new("삼반반타반빠빠빠빠빠빠뿌\n우어번벋벋범벌벖벍벓벒석\n");
        let mut it = Interpreter::new(source);
        assert_eq!(it.counter(), (0, 0));
        assert_eq!(it.storage().len(), 0);
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 1));
        assert_eq!(it.storage().len(), 0);
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 2));
        assert_eq!(it.storage().len(), 1);
        assert_eq!(it.storage().peek(), 2);
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 3));
        assert_eq!(it.storage().len(), 2);
        assert_eq!(it.storage().peek(), 2);
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 4));
        assert_eq!(it.storage().len(), 1);
        assert_eq!(it.storage().peek(), 0);
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 5));
        assert_eq!(it.storage().len(), 2);
        assert_eq!(it.storage().peek(), 2);
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 6));
        assert_eq!(it.storage().len(), 3);
        assert_eq!(it.storage().peek(), 2);
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 7));
        assert_eq!(it.storage().len(), 4);
        assert_eq!(it.storage().peek(), 2);
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 8));
        assert_eq!(it.storage().len(), 5);
        assert_eq!(it.storage().peek(), 2);
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 9));
        assert_eq!(it.storage().len(), 6);
        assert_eq!(it.storage().peek(), 2);
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 10));
        assert_eq!(it.storage().len(), 7);
        assert_eq!(it.storage().peek(), 2);
        assert!(!it.step());
        assert_eq!(it.counter(), (0, 11));
        assert_eq!(it.storage().len(), 8);
        assert_eq!(it.storage().peek(), 2);
        assert!(!it.step());
        assert_eq!(it.counter(), (1, 11));
        assert_eq!(it.storage().len(), 9);
        assert_eq!(it.storage().peek(), 2);
        assert!(!it.step());
        assert_eq!(it.counter(), (1, 10));
        assert_eq!(it.storage().len(), 0);
        assert!(!it.step());
        assert_eq!(it.counter(), (1, 9));
        assert_eq!(it.storage().len(), 1);
        assert_eq!(it.storage().peek(), 9);
        assert!(!it.step());
        assert_eq!(it.counter(), (1, 8));
        assert_eq!(it.storage().len(), 2);
        assert_eq!(it.storage().peek(), 8);
        assert!(!it.step());
        assert_eq!(it.counter(), (1, 7));
        assert_eq!(it.storage().len(), 3);
        assert_eq!(it.storage().peek(), 7);
        assert!(!it.step());
        assert_eq!(it.counter(), (1, 6));
        assert_eq!(it.storage().len(), 4);
        assert_eq!(it.storage().peek(), 6);
        assert!(!it.step());
        assert_eq!(it.counter(), (1, 5));
        assert_eq!(it.storage().len(), 5);
        assert_eq!(it.storage().peek(), 5);
        assert!(!it.step());
        assert_eq!(it.counter(), (1, 4));
        assert_eq!(it.storage().len(), 6);
        assert_eq!(it.storage().peek(), 4);
        assert!(!it.step());
        assert_eq!(it.counter(), (1, 3));
        assert_eq!(it.storage().len(), 7);
        assert_eq!(it.storage().peek(), 3);
        assert!(!it.step());
        assert_eq!(it.counter(), (1, 2));
        assert_eq!(it.storage().len(), 8);
        assert_eq!(it.storage().peek(), 3);
    }
}
