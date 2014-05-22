#![license="BSD simplified"]
#![feature(phase)]

#[phase(syntax,link)]
extern crate aheui;

pub fn main() {
    let args = std::os::args();

    if args.len() <= 1 {
        printerr!("error: no input files");
        return;
    }

    let path_str = &args.as_slice()[1];
    let path = Path::new(path_str.as_slice());

    let mut file = std::io::File::open(&path).ok().expect("error: no such file");
    let source = aheui::Source::from_str(file.read_to_str().ok().expect("error: io error"));
    let mut interpreter = aheui::Interpreter::new(source);
    interpreter.execute();
}