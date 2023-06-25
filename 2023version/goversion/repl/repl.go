package repl

import (
	"bufio"
	"fmt"
	"io"

	"github.com/Andrew-Pynch/PSharp-Programming-Language/2023version/goversion/evaluator"
	"github.com/Andrew-Pynch/PSharp-Programming-Language/2023version/goversion/lexer"
	"github.com/Andrew-Pynch/PSharp-Programming-Language/2023version/goversion/object"
	"github.com/Andrew-Pynch/PSharp-Programming-Language/2023version/goversion/parser"
)

const PROMPT = ">> "

func Start(in io.Reader, out io.Writer) {
	scanner := bufio.NewScanner(in)
	env := object.NewEnvironment()

	for {
		fmt.Printf(PROMPT)
		scanned := scanner.Scan()
		if !scanned {
			return
		}

		line := scanner.Text()
		l := lexer.New(line)
		p := parser.New(l)

		program := p.ParseProgram()
		if len(p.Errors()) != 0 {
			printParserErrors(out, p.Errors())
			continue
		}

		evaluated := evaluator.Eval(program, env)
		if evaluated != nil {
			io.WriteString(out, evaluated.Inspect())
			io.WriteString(out, "\n")
		}
	}
}

const PSHARP = `
 mmmmm          mmmm  #                          
 #   "#        #"   " # mm    mmm    m mm  mmmm  
 #mmm#"        "#mmm  #"  #  "   #   #"  " #" "# 
 #                 "# #   #  m"""#   #     #   # 
 #             "mmm#" #   #  "mm"#   #     ##m#" 
                                           #     
                                           "     
`

func printParserErrors(out io.Writer, errors []string) {
	io.WriteString(out, PSHARP)
	io.WriteString(out, "Woops! We ran into some issues here!\n")
	io.WriteString(out, " parser errors:\n")
	for _, msg := range errors {
		io.WriteString(out, "\t"+msg+"\n")
	}
}
