use std::error::Error;

use nom::Finish;

use super::parser;
use super::prelude::*;

/// Parses brainfuck code.
///
/// If you just want to run this code, see [run].
///
/// If you need more lower level control over parsing, see the [parser] module.
///
/// On success, returns parsed [expressions](parser::Expression).
pub fn parse(code: &str) -> Result<Vec<Expression>, ParseError> {
    match parser::parse(code).finish() {
        Ok(parse_result) => Ok(parse_result.1),
        Err(e) => Err(e.into()),
    }
}

/// Parses and runs brainfuck code.
///
/// By default, this function uses stdin and stdout for the `,` and `.` commands.
/// To change this, use [run_with_io].
///
/// If you need more lower level control, see [Context] and [parse].
///
/// On success, this returns the leftover bytes on the tape.
pub fn run<'a>(code: &'a str) -> Result<Vec<u8>, BfError<'a, std::io::Error>> {
    run_with_io(code, &mut std::io::stdin(), &mut std::io::stdout())
}

/// Parses and runs brainfuck code.
///
/// Uses `in_stream` and `out_stream` for the `,` and `.` commands.
///
/// If you need more lower level control, see [Context] and [parse].
///
/// On success, this returns the leftover bytes on the tape.
pub fn run_with_io<'a, IoErr: Error>(
    code: &'a str,
    in_stream: &mut dyn BfInStream<IoErr>,
    out_stream: &mut dyn BfOutStream<IoErr>,
) -> Result<Vec<u8>, BfError<'a, IoErr>> {
    let exps = parse(code)?;
    let mut context = Context::new();
    context.eval_many(&exps, in_stream, out_stream)?;
    Ok(context.data)
}
