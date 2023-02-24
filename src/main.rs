use fuck_rs::{self, Context};

fn main() {
    #[rustfmt::skip]
    let result = fuck_rs::parse(
"
'H' >++++++++[<+++++++++>-]<.
'e' >++++[<+++++++>-]<+.
'l' +++++++.
'l' .
'o' +++.
' '  >>++++++[<+++++++>-]<++.
'W' ------------.
'o' >++++++[<+++++++++>-]<+.
'r' <.
'l' +++.
'd' ------.
'!' --------.
newline >>>++++[<++++++++>-]<+.
",
    );

    match result {
        Ok(o) => {
            println!("Parse success!\nUnparsed:{:?}\nGot:{:#?}", o.0, &o.1);
            println!("Running...");
            println!("==========");
            let mut context = Context::new();
            match context.eval_many(&o.1, &mut std::io::stdin(), &mut std::io::stdout()) {
                Ok(_) => (),
                Err(e) => println!("ERROR: {e:#?}"),
            }
            println!("\n==========");
        }
        Err(e) => {
            println!("Parse error: {}", e);
            return;
        }
    }
}
