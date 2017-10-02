package main

import "fmt"

func main() {
	star(3)
	star(6)
	star(7)
}

func star(n int) {

	var center = 0

	if n%2 == 0 {
		center = n / 2
	} else {
		center = (n + 1) / 2
	}

	var max = center*2 - 1

	for i := 1; i <= max; i += 2 {
		for j := 0; j < center-i/2; j++ {
			fmt.Print(" ")
		}

		for k := 0; k < i; k++ {
			fmt.Print("*")
		}

		fmt.Println("")
	}

	for i := max - 2; i > 0; i -= 2 {
		for j := 0; j < center-i/2; j++ {
			fmt.Print(" ")
		}

		for k := 0; k < i; k++ {
			fmt.Print("*")
		}

		fmt.Println("")
	}
}
