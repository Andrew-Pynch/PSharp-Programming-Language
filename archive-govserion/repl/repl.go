package repl

import (
	"bufio"
	"fmt"
	"io"

	"github.com/Andrew-Pynch/PSharp-Programming-Language/goversion/lexer"
	"github.com/Andrew-Pynch/PSharp-Programming-Language/goversion/token"
)

const PROMPT = ">> "

func Start(in io.Reader, out io.Writer) {
	scanner := bufio.NewScanner(in)

	for {
		fmt.Printf(PROMPT)
		scanned := scanner.Scan()

		if !scanned {
			return
		}

		line := scanner.Text()
		if line == "exit" {
			return
		}

		// Create a new lexer and input the line that was just entered
		l := lexer.New(line)

		for tok := l.NextToken(); tok.Type != token.EOF; tok = l.NextToken() {
			fmt.Printf("%+v\n", tok)
		}
	}
}
