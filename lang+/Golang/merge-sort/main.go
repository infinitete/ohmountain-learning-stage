package main

import (
	"fmt"
)

func mergeSort(numbers []int) []int {
	size := len(numbers)
	if size <= 1 {
		return numbers
	}

	center := size / 2
	left := mergeSort(numbers[:center])
	right := mergeSort(numbers[center:])

	return merge(left, right)
}

func merge(left []int, right []int) []int {
	leftIndex, rightIndex := 0, 0
	leftSize, rightSize := len(left), len(right)

	result := []int{}

	for leftIndex < leftSize && rightIndex < rightSize {
		if left[leftIndex] < right[rightIndex] {
			result = append(result, left[leftIndex])
			leftIndex++
		} else {
			result = append(result, right[rightIndex])
			rightIndex++
		}
	}

	fmt.Printf("Left:  %d - %d \n", leftSize, leftIndex)
	fmt.Printf("Right: %d - %d \n", rightSize, rightIndex)

	result = append(result, left[leftIndex:]...)
	result = append(result, right[rightIndex:]...)

	return result
}

func main() {
	numbers := mergeSort([]int{1, 3, 2})

	fmt.Printf("%v\n", numbers)
}
