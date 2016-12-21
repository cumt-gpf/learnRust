use std::io::{self, Write};
use rpn::{self, Stack};
use std::str::FromStr;
/// Start a read-eval-print loop, which runs until an error or `quit`.
pub fn read_eval_print_loop() -> rpn::Result<()> {
    // Create a stack to work on.
    let mut stack = Stack::new();

    loop {
        // Print a user input prompt.
        print!("> ");
        try!(io::stdout().flush().map_err(rpn::Error::IO));

        // TODO: Read from stdin into a String, and evaluate_line the result.
        // * An io::Error should be converted into a rpn::Error::IO
        let mut buffer = String::new();
        //map_err() can be uesd to pass through a successful result while handling an error
        try!(io::stdin().read_line(&mut buffer).map_err(rpn::Error::IO));
        try!(evaluate_line(&mut stack, &buffer));
        stack = Stack::new();
    }
}

fn evaluate_line(stack: &mut Stack, buf: &String) -> rpn::Result<()> {
    // Create an iterator over the tokens.
    let tokens = buf.trim().split_whitespace();

    // TODO: Evaluate all of the tokens on the line.
    for word in tokens {
        match word {
            "true" => {
                try!(stack.push(rpn::Elt::Bool(true))); 
            },
            "false" => {
                try!(stack.push(rpn::Elt::Bool(false)));
            },
            "+" => {
                try!(stack.eval(rpn::Op::Add));
            }
            "~" => {
                try!(stack.eval(rpn::Op::Neg));
            }
            "<->" => {
                try!(stack.eval(rpn::Op::Swap));
            }
            "=" => {
                try!(stack.eval(rpn::Op::Eq));
            }
            "#" => {
                try!(stack.eval(rpn::Op::Rand));
            }
            "quit" => {
                try!(stack.eval(rpn::Op::Quit));
            }
            _ => {
                let my_int = try!(i32::from_str(word).map_err(|_| rpn::Error::Syntax));
                try!(stack.push(rpn::Elt::Int(my_int)));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use rpn::{Stack, Error, Elt};
    use parser::evaluate_line;

    #[test]
    fn test_evaluate_line_bool() {
        let mut stack = Stack::new();
        let s = "true".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(true));
        let s = "false".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(false));
    }

    #[test]
    fn test_evaluate_line_int() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Int(12));
    }

    #[test]
    fn test_evaluate_line_plus() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "13".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "+".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Int(25));
    }

    #[test]
    fn test_evaluate_line_neg() {
        let mut stack = Stack::new();
        let s = "false".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "~".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_evaluate_line_swap() {
        let mut stack = Stack::new();
        let s = "false".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "15".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "<->".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(false));
        assert_eq!(stack.pop().unwrap(), Elt::Int(15));
    }

    #[test]
    fn test_evaluate_line_eq() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "15".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "=".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(false));
    }

    #[test]
    fn test_evaluate_line_rand() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "#".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let res = stack.pop();
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(res >= Elt::Int(0));
        assert!(res < Elt::Int(12));
    }

    #[test]
    fn test_evaluate_line_quit() {
        let mut stack = Stack::new();
        let s = "quit".to_string();
        let res = evaluate_line(&mut stack, &s);
        assert!(res.is_err());
        if let Err(Error::Quit) = res {
        } else { assert!(false); }
    }

    #[test]
    fn test_evaluate_line_bad_parse() {
        let mut stack = Stack::new();
        let s = "~false".to_string();
        let res = evaluate_line(&mut stack, &s);
        assert!(res.is_err());
        if let Err(Error::Syntax) = res {
        } else { assert!(false); }
    }
}
