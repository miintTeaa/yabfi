use std::{
    env, fs,
    io::{self, Read},
    process,
};

use fuck_rs::{self, Context};

fn main() {
    let mut args = env::args();

    _ = args.next().expect("Receiving program name");

    let mode = match args.next() {
        Some(s) => s,
        None => err_with_help(1, "Missing mode".to_string()),
    };

    let code = match mode.as_str() {
        "file" => {
            let input = match args.next() {
                Some(s) => s,
                None => err_with_help(3, "Missing input".to_string()),
            };
            let bytes = match fs::read(input) {
                Ok(bytes) => bytes,
                Err(e) => err_no_help(4, format!("Couldn't read from file: {e:#?}")),
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
        mode => {
            err_with_help(2, format!("Invalid mode: {mode}"));
        }
    };

    let result = fuck_rs::parse(&code);

    match result {
        Ok(o) => {
            let mut context = Context::new();
            match context.eval_many(&o.1, &mut io::stdin(), &mut io::stdout()) {
                Ok(_) => (),
                Err(e) => println!("RuntimeError at {}: {e:#?}", context.head),
            }
        }
        Err(e) => {
            println!("Parse error: {e:#?}");
            return;
        }
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

    println!("Format: {program_name} <mode> <input>");
    print_sep();
    println!("<mode> has these possible values:");
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
