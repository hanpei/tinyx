use crate::{lexer::Lexer, parser::parser::Parser};

use super::*;

fn interpret(contents: &str) {
    let lexer = Lexer::new(&contents.as_bytes(), "source.txt");
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    let mut i = Interpreter::default();
    i.interpret(ast.unwrap()).unwrap();
}

#[test]
fn var_decl() {
    let source = r#"
        let a = "global a";
        let b = "global b";
        let c = "global c";
        {
        let a = "outer a";
        let b = "outer b";
        {
            let a = "inner a";
            print a;
            print b;
            print c;
        }
        print a;
        print b;
        print c;
        }
        print a;
        print b;
        print c;   
    "#;

    interpret(source);
}
#[test]
fn while_stmt() {
    let source = r#"
        let a = 0;
        while (a < 10) {
            print a;
            a = a + 1;
        }    
    "#;

    interpret(source);
}

#[test]
fn while_break_stmt() {
    let source = r#"
        let a = 0;
        while (a < 10) {
            if (a == 5) {
                return;
            } else {
                print a;
            }
            a = a + 1;
        }    
    "#;

    interpret(source);
}

#[test]
fn if_break_stmt() {
    let source = r#"
        let a = 0;
        while (a < 10) {
            if (a == 5)  return;
            print a;
            a = a + 1;
        }    
    "#;

    interpret(source);
}

#[test]
fn break_stmt() {
    let source = r#"
        fn count(n) {
            while (n < 10) {
                if (n == 3) return n; 
                print n;
                n = n + 1;
            }
        }
      
        count(1);   
    "#;

    interpret(source);
}

#[test]
fn if_stmt() {
    let source = r#"
        // if stmt
        let a = 11
        let b = 3
        if (a-b<0) {
            print "a < b"
        } else if(a - b >0) {
            print "a > b"
        }
        print a
    
    "#;

    interpret(source);
}

#[test]
fn closure_fn() {
    let source = r#"
        // closure
        fn makeCounter() {
            let i = 0;
            fn count() {
                i = i + 1;
                print i;
            }
            return count;
        }
        
        let counter = makeCounter();
        counter(); // "1".
        counter(); // "2".
    "#;

    interpret(source);
}

#[test]
fn test_rust_scope() {
    let a = "global";
    {
        println!("{}", a);
        let a = "block";
        println!("{}", a);
    }
}
