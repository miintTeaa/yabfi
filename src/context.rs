use crate::BfInStream;
use crate::BfOutStream;
use crate::parser::*;
use crate::error::*;

use std::error::Error;

/// A brainfuck context.
/// 
/// To actually run any programs on this, you need to use eval_exp() and eval_many().
/// 
/// ```
/// use fuck_rs::Context;
/// 
/// let context = Context::new();
/// 
/// assert_eq!(context.data, vec![0]);
/// assert_eq!(context.head, 0);
/// ```
pub struct Context {
    /// The tape for this context.
    pub data: Vec<u8>,
    /// The position of the read/write pointer.
    pub head: usize,
}

impl Context {
    /// Creates a new context.
    /// 
    /// By default, the head is `0` and the data is `vec![0]`.
    pub fn new() -> Self {
        Self {
            data: vec![0],
            head: 0,
        }
    }

    /// Evaluates a single [expression][Expression].
    #[rustfmt::skip]
    pub fn eval_exp<IoErr: Error>(&mut self, exp: &Expression, in_stream: &mut dyn BfInStream<IoErr>, out_stream: &mut dyn BfOutStream<IoErr>) -> Result<(), RuntimeError<IoErr>> {
        match exp {
            Expression::Comment(_) => {}
            Expression::Decrement => {self.data[self.head] = self.data[self.head].wrapping_sub(1);}
            Expression::Increment => {self.data[self.head] = self.data[self.head].wrapping_add(1);}
            Expression::Input => {
                self.data[self.head] = in_stream.read()?;
            }
            Expression::Output => {
                out_stream.write(self.data[self.head])?;
            }
            Expression::Next => {
                // Ensures self.head is never bigger than isize::MAX
                self.head = match (self.head as isize).checked_add(1) {
                    Some(v) => v,
                    None => return Err(RuntimeError::PtrOverflow { bound: PtrOverflowBound::Max })
                } as usize;
                
                if self.head >= self.data.len() {
                    self.data.push(0);
                }
            }
            Expression::Prev => {
                self.head = match self.head.checked_sub(1) {
                    Some(v) => v,
                    None => return Err(RuntimeError::PtrOverflow { bound: PtrOverflowBound::Min })
                };
            }
            Expression::Loop(loop_data) => {
                while self.data[self.head] != 0 {
                    self.eval_many(&loop_data, in_stream, out_stream)?;
                }
            }
        }
        Ok(())
    }

    /// Evaluates many [expressions][Expression] in sequence.
    pub fn eval_many<IoErr: Error>(&mut self, exps: &Vec<Expression>, in_stream: &mut dyn BfInStream<IoErr>, out_stream: &mut dyn BfOutStream<IoErr>) -> Result<(), RuntimeError<IoErr>> {
        for exp in exps {
            self.eval_exp(exp, in_stream, out_stream)?;
        }
        Ok(())
    }
}
