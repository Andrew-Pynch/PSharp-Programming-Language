use psharp_programming_language::{
    ast::{Expression, Identifier, Program, Statement, StatementType},
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
        let stmt: Statement = program.statements[i].clone();
        assert!(test_let_statement(stmt, tt.expected_identifier));
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
        if stmt.statement_type
            != StatementType::ReturnStatement(Expression::Identifier(Identifier::new(
                Token::new(TokenType::IDENT, "".to_string()),
                "".to_string(),
            )))
        {
            println!("stmt not ReturnStatement. got={:?}", stmt.statement_type);
            assert!(false);
        }

        if stmt.token.literal != "return" {
            println!("stmt.TokenLiteral not 'return', got={}", stmt.token.literal);
            assert!(false);
        }
    }
}

fn test_let_statement(s: Statement, name: &str) -> bool {
    if s.token.literal != "let" {
        println!("s.TokenLiteral not 'let'. got={}", s.token.literal);
        return false;
    }

    match s.statement_type {
        StatementType::LetStatement(ref let_name, _) => {
            if let_name.value != name {
                println!("let_stmt.Name.Value not '{}'. got={}", name, let_name.value);
                return false;
            }

            if let_name.token.literal != name {
                println!("s.Name not '{}'. got={}", name, let_name.token.literal);
                return false;
            }
        }
        _ => {
            println!("s not LetStatement. got={:?}", s.statement_type);
            return false;
        }
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
