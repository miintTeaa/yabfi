#[rustfmt::skip]
#[allow(unused)]
use nom::{
    bytes::{
        complete as byte_complete,
        streaming as byte_streaming,
    },
    character::{
        complete as char_complete,
        streaming as char_streaming
    },
    complete as bit_complete,
    streaming as bit_streaming,
};

use nom::{combinator::all_consuming, multi::many0, sequence::delimited, IResult, Parser};

/// An expression that can be evaluated by a [context](crate::context::Context). See individual values for more details.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    /// The `>` command.
    Next,
    /// The `<` command.
    Prev,
    /// The `+` command.
    Increment,
    /// The `-` command.
    Decrement,
    /// The `.` command.
    Output,
    /// The `,` command.
    Input,
    /// Loops (anything delimited by `[` and `]`).
    Loop(Vec<Expression>),

    /// Any other string of characters.
    Comment(String),
}

macro_rules! cmd_parser {
    ($pname:ident; $ch:literal -> $cmd:expr) => {
        /// Parses a single expression of the given type.
        pub fn $pname(input: &str) -> IResult<&str, Expression> {
            char_complete::char($ch).map(|_| $cmd).parse(input)
        }
    };
}

cmd_parser!(next_exp; '>' -> Expression::Next);
cmd_parser!(prev_exp; '<' -> Expression::Prev);
cmd_parser!(increment_exp; '+' -> Expression::Increment);
cmd_parser!(decrement_exp; '-' -> Expression::Decrement);
cmd_parser!(output_exp; '.' -> Expression::Output);
cmd_parser!(input_exp; ',' -> Expression::Input);

/// Parses a single non-loop non-comment expression.
pub fn simple_exp(input: &str) -> IResult<&str, Expression> {
    next_exp
        .or(prev_exp)
        .or(increment_exp)
        .or(decrement_exp)
        .or(output_exp)
        .or(input_exp)
        .parse(input)
}

/// Parses a loop expression.
pub fn loop_exp(input: &str) -> IResult<&str, Expression> {
    delimited(
        char_complete::char('['),
        many0(exp),
        char_complete::char(']'),
    )
    .map(|v| Expression::Loop(v))
    .parse(input)
}

/// Parses a comment expression.
pub fn comment_exp(input: &str) -> IResult<&str, Expression> {
    let (input, out) = byte_complete::is_not("><+-.,[]")(input)?;
    Ok((input, Expression::Comment(out.to_string())))
}

/// Parses any single expression.
pub fn exp(input: &str) -> IResult<&str, Expression> {
    simple_exp.or(comment_exp).or(loop_exp).parse(input)
}

/// Parses zero or more expressions.
pub fn many_exp(input: &str) -> IResult<&str, Vec<Expression>> {
    many0(exp)(input)
}

/// Parses zero or more expressions, ensures entire input is consumed.
///
/// This is functionally the same as [many_exp].
pub fn parse(input: &str) -> IResult<&str, Vec<Expression>> {
    all_consuming(many_exp)(input)
}
