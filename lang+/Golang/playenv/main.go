package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	var args = getArgs()

	if len(args) < 2 {
		printHelp()
		return
	}

	envs := getEnv(args)

	printEnvs(args, envs)
}

func getArgs() []string {
	return os.Args
}

func getEnv(args []string) []string {
	var len = len(args)
	var ret = make([]string, len)

	for i := 0; i < len; i++ {
		e := os.Getenv(args[i])

		if strings.Count(e, "") == 1 {
			ret[i] = "None"
		} else {
			ret[i] = e
		}
	}

	return ret
}

func printEnvs(args []string, envs []string) {

	var (
		env_len = len(envs)
		max_len = 4
	)

	for i := 1; i < env_len; i++ {
		if len(args[i]) > max_len {
			max_len = len(args[i])
		}
	}

	format := fmt.Sprintf("%%%ds ==> %%s\n", max_len+4)

	fmt.Println("[")

	for i := 1; i < env_len; i++ {
		fmt.Printf(format, args[i], envs[i])
	}

	fmt.Println("]")
}

func printHelp() {
	fmt.Println("Usage: playenv args")
	fmt.Println("")
	fmt.Println("Example:")
	fmt.Println("playenv HOME")
	fmt.Println("[")
	fmt.Println("    HOME ==> /home/mio")
	fmt.Println("]")
}
