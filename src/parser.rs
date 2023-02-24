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

use nom::{multi::many0, sequence::delimited, IResult, Parser};

/// An expression that can be evaluated by a [Context]. See individual values for more details.
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
        pub fn $pname(input: &str) -> IResult<&str, Expression> {
            char_complete::char($ch).map(|_| $cmd).parse(input)
        }
    };
}

cmd_parser!(next; '>' -> Expression::Next);
cmd_parser!(prev; '<' -> Expression::Prev);
cmd_parser!(increment; '+' -> Expression::Increment);
cmd_parser!(decrement; '-' -> Expression::Decrement);
cmd_parser!(print; '.' -> Expression::Output);
cmd_parser!(read; ',' -> Expression::Input);

pub fn simple_exp(input: &str) -> IResult<&str, Expression> {
    next.or(prev)
        .or(increment)
        .or(decrement)
        .or(print)
        .or(read)
        .parse(input)
}

pub fn loop_exp(input: &str) -> IResult<&str, Expression> {
    delimited(
        char_complete::char('['),
        many0(exp),
        char_complete::char(']'),
    )
    .map(|v| Expression::Loop(v))
    .parse(input)
}

pub fn comment_exp(input: &str) -> IResult<&str, Expression> {
    let (input, out) = byte_complete::is_not("><+-.,[]")(input)?;
    Ok((input, Expression::Comment(out.to_string())))
}

pub fn exp(input: &str) -> IResult<&str, Expression> {
    simple_exp.or(comment_exp).or(loop_exp).parse(input)
}

pub fn parse(input: &str) -> IResult<&str, Vec<Expression>> {
    many0(exp)(input)
}
