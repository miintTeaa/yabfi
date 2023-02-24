use std::{
    env, fs,
    io::{self, Read},
    process,
};

use fuck_rs::{self, prelude::*};

fn main() {
    let mut args = env::args();

    _ = args.next().expect("Receiving program name");

    let mode = match args.next() {
        Some(s) => s,
        None => err_with_help(1, "Missing mode".to_string()),
    };

    let input_mode = match args.next() {
        Some(s) => s,
        None => err_with_help(1, "Missing input mode".to_string()),
    };

    let code = match input_mode.as_str() {
        "file" => {
            let input = match args.next() {
                Some(s) => s,
                None => err_with_help(3, "Missing input".to_string()),
            };
            let bytes = match fs::read(input) {
                Ok(bytes) => bytes,
                Err(e) => err_no_help(4, format!("Failed to read file: {e:#?}")),
            };
            match String::from_utf8(bytes) {
                Ok(s) => s,
                Err(e) => err_no_help(6, format!("Code is not utf-8: {e:#?}")),
            }
        }
        "args" => match args.next() {
            Some(s) => s,
            None => err_with_help(3, "Missing input".to_string()),
        },
        "stdi" => read_stdin(),
        _ => {
            err_with_help(2, format!("Invalid input mode: {input_mode}"));
        }
    };

    match mode.as_str() {
        "run" => match fuck_rs::run(&code) {
            Ok(_) => {}
            Err(e) => println!("{e}"),
        },
        "parse" => match fuck_rs::parse(&code) {
            Ok(tree) => println!("{tree:#?}"),
            Err(e) => println!("{e}"),
        },
        _ => err_with_help(2, format!("Invalid mode: {mode}")),
    }
}

fn read_stdin() -> String {
    let mut stdin = io::stdin();

    let mut bytes = Vec::new();
    match stdin.read_to_end(&mut bytes) {
        Ok(_) => {}
        Err(e) => err_no_help(5, format!("Failed to read stdin: {e:#?}")),
    };
    match String::from_utf8(bytes) {
        Ok(s) => s,
        Err(e) => err_no_help(6, format!("Code is not utf-8: {e:#?}")),
    }
}

fn err_with_help(code: i32, info: String) -> ! {
    println!("ERR: {info}");
    print_sep();
    print_help();
    process::exit(code);
}

fn err_no_help(code: i32, info: String) -> ! {
    println!("ERR: {info}");
    process::exit(code);
}

fn print_help() {
    let mut args = env::args();

    let program_name = args.next().expect("Receiving program name");

    println!("Format: {program_name} <mode> <in_mode> <input>");
    print_sep();
    println!("<mode> has these possible values:");
    println!("  run: Runs brainfuck code.");
    println!("  parse: Parses brainfuck code and prettyprints syntax tree.");
    print_sep();
    println!("<in_mode> has these possible values:");
    println!("  file: Reads <input> as a file path and interprets its content as bf code.");
    println!("        Incompatible with 't' and 'i'.");
    println!("  args: Reads <input> as brainfuck code and interprets it.");
    println!("        Incompatible with 'f' and 'i'.");
    println!("  stdi: Ignores <input> and interprets stdin as brainfuck code up to EOF.");
    println!("        Incompatible with 't' and 'f'.");
    println!("        This will always throw an IoErr if the program was piped in and has");
    println!("        read (',') instructions.");
}

fn print_sep() {
    println!("==========");
}
