use psharp_programming_language::{
    ast::{LetStatement, Statement},
    lexer::Lexer,
    parser::Parser,
};

struct TestCase<'a> {
    expected_identifier: &'a str,
}

fn get_let_test_input() -> String {
    let test_input: String = " 
        let x = 5;
        let y = 10;
        let foobar = 838383
    "
    .to_string();

    return test_input;
}

#[test]
fn test_let_statements() {
    let input: String = "
        let x = 5;
        let y = 10;
        let foobar = 838383
    "
    .to_string();

    let l = Lexer::new(&input);
    let mut p = Parser::new(&l);
    let program = p.parse_program();

    assert!(program.statements.len() > 0);
    assert_eq!(program.statements.len(), 3);

    let tests = vec![
        TestCase {
            expected_identifier: "x",
        },
        TestCase {
            expected_identifier: "y",
        },
        TestCase {
            expected_identifier: "foobar",
        },
    ];

    // iterate and enumerate through the test cases
    for (i, tt) in tests.iter().enumerate() {
        let stmt: &Box<dyn Statement> = &program.statements[i];
        assert!(test_let_statement(stmt, tt.expected_identifier));
    }
}

fn test_let_statement(s: &Box<dyn Statement>, name: &str) -> bool {
    if s.token_literal() != "let" {
        println!("s.TokenLiteral not 'let'. got={}", s.token_literal());
        return false;
    }

    // let let_stmt = s.as_any().downcast_ref::<LetStatement>().unwrap();
    match s.as_any().downcast_ref::<LetStatement>() {
        Some(let_stmt) => {
            if let_stmt.name.value != name {
                println!(
                    "let_stmt.Name.Value not '{}'. got={}",
                    name, let_stmt.name.value
                );
                return false;
            }

            if let_stmt.name.token_literal() != name {
                println!(
                    "s.Name not '{}'. got={}",
                    name,
                    let_stmt.name.token_literal()
                );
                return false;
            }
        }
        None => {
            println!("s not LetStatement. got={:?}", s.to_string());
            return false;
        }
    }

    true
}
