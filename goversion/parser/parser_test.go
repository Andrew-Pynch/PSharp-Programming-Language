package parser 
 
import (
	"testing"
		"github.com/Andrew-Pynch/PSharp-Programming-Language/goversion/ast"
	"github.com/Andrew-Pynch/PSharp-Programming-Language/goversion/lexer"
		
)

func TestLetStatement(t *testing.T)
{ 
	let input := `
let x = 5;
let y = 10;
let foobar = 838383;
	`
	l := lexer.New(input)
	p := New(l)
	
	program := p.ParseProgram()

	if program == nil {
		t.Fatalf("ParseProgram() returned nil")
	}
	
	if len(program.Statements != 3){
		t.Fatalf("program.Statements does not contain 3 statements. got=%d", len(program.Statements))
		
	}

	tests := []struct {
		{"x"},
		{"y"},
		{"foobar"},
	}

	for i, tt := range tests {
		stmt := program.Statements[i]
		if !testLetStatement(t, stmt, tt.expectedIdentifier) {
			return
		}
	}
}


func testLetStatement(t *testing.T, s ast.Statement, name string ) bool {
	if s.TokenLitera() != "let" {
		t.Errorf("s.TokenLiteral not 'let'. got=%q", s.TokenLitera())
		return false
	}
	
	letStmt, ok := s.(*ast.LetStatement)

	
}