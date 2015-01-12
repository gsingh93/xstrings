extern crate libc;
extern crate getopts;
extern crate regex;

use libc::exit;

use regex::Regex;

use std::os;
use std::io::File;
use std::string::String;

use getopts::getopts;
use getopts::optflag;
use getopts::Matches;
use getopts::usage;

fn main() {
    parse_args();

    let matches = parse_args();
    let filename = &*matches.free[1];
    let text = read_text(filename);
    if matches.opts_present(&["b".to_string(), "x".to_string(), "s".to_string(),
                              "w".to_string()]) {
        if matches.opt_present("b") {
            find_binary(&*text);
        }
        if matches.opt_present("x") {
            find_hex(&*text);
        }
        if matches.opt_present("s") {
            find_base64(&*text);
        }
        if matches.opt_present("w") {
            find_words(&*text);
        }
    } else {
        // find_words(&text);
        find_hex(&*text);
        find_base64(&*text);
        find_binary(&*text);
    }
}

fn parse_args() -> Matches {
    let args = os::args();

    let opts = [optflag("b", "binary", "Output binary matches"),
                optflag("x", "hex",    "Output word matches"),
                optflag("s", "base64", "Output base64 matches"),
                optflag("w", "words",  "Output English word matches"),
                optflag("h", "help",   "Print this help message")];

    let matches = match getopts(args.as_slice(), &opts) {
        Ok(matches) => matches,
        Err(e) => panic!(e)
    };

    if matches.opt_present("h") {
        println!("{}", usage("Searches a file for binary, hex, base64, \
                              and English word strings", &opts));
        unsafe {libc::exit(1)}; // TODO: Return option
    }

    if matches.free.len() != 2 {
        fail("program only checks one file, multiple arguments provided");
    }

    matches
}

fn read_text(filename: &str) -> String {
    let path = Path::new(filename);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => fail(e.desc)
    };

    match file.read_to_string() {
        Ok(text) => text,
        Err(e) => fail(e.desc)
    }
}

fn find(text: &str, regex: &str) {
    let re = match Regex::new(regex) {
        Ok(re) => re,
        Err(e) => fail(&*e.msg)
    };
    for cap in re.captures_iter(text) {
        println!("{:?}", cap.at(0));
    }
}

fn find_binary(text: &str) {
    println!("{}", "Binary:");
    find(text, r"[01]{3,}");
    println!("");
}

fn find_hex(text: &str) {
    println!("{}", "Hex:");
    find(text, r"(0[xX])?[0-9a-fA-F]{2,}");
    println!("");
}

fn find_base64(text: &str) {
    println!("{}", "Base64:");
    find(text, r"(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=|[A-Za-z0-9+/]{4})");
    println!("");
}

fn find_words(text: &str) {
    panic!("Not implemented");
    println!("{}", "Words:");
    find(text, r"");
    println!("");
}

fn fail(message: &str) -> ! {
    println!("Error: {}", message);
    unsafe { exit(1); } // TODO
}
