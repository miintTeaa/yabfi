mod context;
mod error;
mod io;
mod parser;

pub use context::*;
pub use error::*;
pub use io::*;
pub use parser::*;

#[cfg(test)]
mod test {
    use crate::*;
    use std::{error::Error, fmt::Display};

    #[derive(Debug)]
    struct TestError(String);

    impl Display for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl Error for TestError {}

    impl BfInStream<TestError> for Vec<u8> {
        fn read(&mut self) -> Result<u8, TestError> {
            Ok(self.remove(0))
        }
    }

    impl BfOutStream<TestError> for Vec<u8> {
        fn write(&mut self, byte: u8) -> Result<(), TestError> {
            Ok(self.push(byte))
        }
    }

    #[test]
    fn hello_world() {
        let code = "\
        'H' >++++++++[<+++++++++>-]<.\n\
        'e' >++++[<+++++++>-]<+.\n\
        'l' +++++++.\n\
        'l' .\n\
        'o' +++.\n\
        comma  >>++++++[<+++++++>-]<++.\n\
        ' ' ------------.\n\
        'W' >++++++[<+++++++++>-]<+.\n\
        'o' <.\n\
        'r' +++.\n\
        'l' ------.\n\
        'd' --------.\n\
        '!' >>>++++[<++++++++>-]<+.\n\
        reading 4 chars >>>++++[>,.<-]";

        #[rustfmt::skip]
        let correct_parse = {
            use crate::Expression::{
                Comment,
                Decrement as Dec,
                Increment as Inc,
                Input as In,
                Output as Out,
                Loop, Next, Prev
            };
            [
                Comment("'H' ".to_string()),
                Next,
                Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc,
                Loop(vec![
                    Prev,
                    Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc,
                    Next, Dec,
                ]),
                Prev, Out,

                Comment("\n'e' ".to_string()),
                Next,
                Inc,
                Inc,
                Inc,
                Inc,
                Loop(vec![
                    Prev,
                    Inc, Inc, Inc, Inc, Inc, Inc, Inc,
                    Next, Dec,
                ]),
                Prev, Inc, Out,

                Comment("\n'l' ".to_string()),
                Inc, Inc, Inc, Inc, Inc, Inc, Inc,
                Out,
                
                Comment("\n'l' ".to_string()),
                Out,

                Comment("\n'o' ".to_string()),
                Inc, Inc, Inc,
                Out,
                
                Comment("\ncomma  ".to_string()),
                Next, Next,
                Inc, Inc, Inc, Inc, Inc, Inc,
                Loop(vec![
                    Prev, Inc, Inc, Inc, Inc, Inc, Inc,
                    Inc, Next, Dec,
                ]),
                Prev, Inc, Inc, Out,

                Comment("\n' ' ".to_string()),
                Dec, Dec, Dec, Dec, Dec, Dec,
                Dec, Dec, Dec, Dec, Dec, Dec,
                Out,

                Comment("\n'W' ".to_string()),
                Next,
                Inc, Inc, Inc, Inc, Inc, Inc,
                Loop(vec![
                    Prev,
                    Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc,
                    Next, Dec,
                ]),
                Prev, Inc, Out,

                Comment("\n'o' ".to_string()),
                Prev, Out,

                Comment("\n'r' ".to_string()),
                Inc, Inc, Inc, Out,

                Comment("\n'l' ".to_string()),
                Dec, Dec, Dec, Dec, Dec, Dec, Out,
                
                Comment("\n'd' ".to_string()),
                Dec, Dec, Dec, Dec, Dec, Dec, Dec, Dec, Out,
                
                Comment("\n'!' ".to_string()),
                Next, Next, Next,
                Inc, Inc, Inc, Inc,
                Loop(vec![
                    Prev,
                    Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc,
                    Next, Dec,
                ]),
                Prev, Inc, Out,

                Comment("\nreading 4 chars ".to_string()),
                Next, Next, Next, Inc, Inc, Inc, Inc, Loop(vec![Next, In, Out, Prev, Dec])
            ]
        };

        let (leftover_input, parse_result) = crate::parse(code).expect("Parsing hello world");
        assert_eq!(leftover_input, "");
        assert_eq!(parse_result, correct_parse);

        let mut context = crate::Context::new();

        let mut in_stream = b"testing".to_vec();
        let mut out_stream = Vec::new();

        let mut out_expected = b"Hello, World!".to_vec();
        out_expected.extend_from_slice(&in_stream[0..4]);

        context
            .eval_many(&parse_result, &mut in_stream, &mut out_stream)
            .expect("Evaluating hello world");

        println!("{}", String::from_utf8(out_stream.clone()).unwrap());

        assert_eq!(out_stream, out_expected);
    }
}
