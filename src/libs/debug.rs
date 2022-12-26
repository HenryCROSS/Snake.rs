use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn log(f:&str, line: &str, msg: &str) {
    let path = Path::new("debug.txt");
    let display = path.display();

    let mut file = match File::options().append(true).open(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_fmt(format_args!("File: {} Line: {} Msg: {}\n", f, line, msg)) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => {},
    }
}
