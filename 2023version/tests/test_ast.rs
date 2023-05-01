use psharp_programming_language::ast::{Program, Statement};

#[test]
fn test_string() {
    let program: Program = Program {
        statements: Vec::new(Statement::LetStatement(Identifider::new("let"))),
    };
}
