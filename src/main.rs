extern crate getopts;
extern crate regex;

use regex::Regex;
use std::process::Command;
use std::env;

use std::string::String;

use std::collections::btree_set::BTreeSet;

use getopts::{Options, Matches};

fn main() {
    parse_args();

    let matches = match parse_args() {
        Some(matches) => matches,
        None => return
    };
    let filename = &*matches.free[1];
    let strings = get_strings(filename);

    if matches.opts_present(&["b".to_string(), "x".to_string(), "s".to_string(),
                              "w".to_string()]) {
        if matches.opt_present("b") {
            find_binary(&strings);
        }
        if matches.opt_present("x") {
            find_hex(&strings);
        }
        if matches.opt_present("s") {
            find_base64(&strings);
        }
        if matches.opt_present("w") {
            println!("All strings:");
            print_set(&strings);
        }
    } else {
        find_hex(&strings);
        find_base64(&strings);
        find_binary(&strings);
    }
}

fn parse_args() -> Option<Matches> {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("b", "binary",  "Output binary matches");
    opts.optflag("x", "hex",     "Output word matches");
    opts.optflag("s", "base64",  "Output base64 matches");
    opts.optflag("w", "strings", "Output strings using the `strings` command");
    opts.optflag("h", "help",    "Print this help message");

    let matches = match opts.parse(&args[1..]) {
        Ok(matches) => matches,
        Err(e) => panic!(e)
    };

    if matches.opt_present("h") {
        println!("{}", opts.usage("Searches a file for binary, hex, base64, \
                                   and English word strings"));
        return None;
    }

    if matches.free.len() != 2 {
        panic!("exactly one file must be provided");
    }

    Some(matches)
}

fn print_set(set: &BTreeSet<String>) {
   for s in set {
       println!("{}", s);
   }
}

fn find(text: &str, regex: &str, set: &mut BTreeSet<String>) {
    let re = match Regex::new(regex) {
        Ok(re) => re,
        Err(e) => panic!(e.msg)
    };
    for cap in re.captures_iter(text) {
        set.insert(cap.at(0).unwrap().to_string());
    }
}

fn find_binary(set: &BTreeSet<String>) {
    let mut results = BTreeSet::new();
    println!("{}", "Binary:");
    for string in set.iter() {
        find(string, r"[01]{3,}", &mut results);
    }
    print_set(&results);
    println!("");
}

fn find_hex(set: &BTreeSet<String>) {
    let mut results = BTreeSet::new();
    println!("{}", "Hex:");
    for string in set.iter() {
        find(string, r"(0[xX])?[0-9a-fA-F]{2,}", &mut results);
    }
    print_set(&results);
    println!("");
}

fn find_base64(set: &BTreeSet<String>) {
    let mut results = BTreeSet::new();
    println!("{}", "Base64:");
    for string in set.iter() {
        find(string,
             r"(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=|[A-Za-z0-9+/]{4})",
             &mut results);
    }
    print_set(&results);
    println!("");
}

fn get_strings(filename: &str) -> BTreeSet<String> {
    let output = Command::new("strings").arg(filename).output();
    let output = String::from_utf8(output.unwrap().stdout).unwrap();
    let mut strings = BTreeSet::new();
    for string in output.lines() {
        strings.insert(string.to_string());
    }
    strings
}
