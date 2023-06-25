use psharp_programming_language::{
    ast::{Expression, Identifier, LetStatement, Node, Program, ReturnStatement, Statement},
    lexer::Lexer,
    parser::Parser,
    token::{Token, TokenType},
};

struct TestCase<'a> {
    expected_identifier: &'a str,
}

#[test]
fn test_let_statements() {
    let input: String = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
    "
    .to_string();

    let l: Lexer = Lexer::new(&input);
    let mut p: Parser = Parser::new(&l);
    let program: Program = p.parse_program();
    check_parser_errors(&p);

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
        let stmt = &*program.statements[i];
        if let Some(let_stmt) = stmt.as_any().downcast_ref::<LetStatement>() {
            assert!(test_let_statement(let_stmt, tt.expected_identifier));
        } else {
            assert!(false, "not a LetStatement");
        }
    }
}

#[test]
fn test_return_statements() {
    let input: String = "
        return 5;
        return 10;
        return 993322;
    "
    .to_string();

    let l: Lexer = Lexer::new(&input);
    let mut p: Parser = Parser::new(&l);
    let program: Program = p.parse_program();
    check_parser_errors(&p);

    assert!(program.statements.len() > 0);
    assert_eq!(program.statements.len(), 3);

    // iterate through the statements and check that they are all return statements
    for stmt in program.statements {
        let stmt = &*stmt;
        if let Some(return_stmt) = stmt.as_any().downcast_ref::<ReturnStatement>() {
            assert_eq!(return_stmt.token_literal(), "return");
        } else {
            assert!(false, "stmt not ReturnStatement. got={:?}", stmt);
        }
    }
}

fn test_let_statement(s: &LetStatement, name: &str) -> bool {
    if s.token_literal() != "let" {
        println!("s.TokenLiteral not 'let'. got={}", s.token_literal());
        return false;
    }
    if s.name.value != name {
        println!("let_stmt.Name.Value not '{}'. got={}", name, s.name.value);
        return false;
    }
    if s.name.token_literal() != name {
        println!("s.Name not '{}'. got={}", name, s.name.token_literal());
        return false;
    }
    true
}

fn check_parser_errors(p: &Parser) {
    let errors: Vec<String> = p.errors.clone();

    if errors.len() == 0 {
        return;
    }

    println!("parser has {} errors", errors.len());
    for msg in &errors {
        println!("parser error: {}", msg);
    }
    panic!("parser has {} errors", errors.len());
}
