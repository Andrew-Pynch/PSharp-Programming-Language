package main

import (
	"fmt"
	"os"
	"os/user"

	"github.com/Andrew-Pynch/PSharp-Programming-Language/goversion/repl"
)

func main() {
	user, err := user.Current()
	if err != nil {
		panic(err)
	}

	fmt.Printf("Hello %s!\n Welcome to the P# programming language", user.Username)
	fmt.Printf("You are currently in interactive mode, type 'exit' to exit\n")

	repl.Start(os.Stdin, os.Stdout)
}
