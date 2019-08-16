package main

import (
	"fmt"
	"strings"
)

func unrepeated_substring(str string) string {
	size := len(str)
	runes := strings.Split(str, "")
	s, l := 0, 0

	for i, _ := range runes {
		sub := map[string]struct{}{}

		for j := i; j < size-1; j++ {

			// 如果出现重复字符，则退出循环
			if _, ok := sub[runes[j]]; ok {
				break
			}

			sub[runes[j]] = struct{}{}
		}

		// 比较最长字符和当前字符长度
		// 如果当前字符长度比记录的要长
		// 则记录值变为当前
		// 标记当前下标和长度
		if len(sub) > l {
			s = i
			l = len(sub)
		}
	}

	return strings.Join(runes[s:s+l], "")
}

//
// 查找某个字符串出现的次数和位置
// 返回元素个数为出现次数
// 元素值为所在下标
//
func whereAmI(s string, sep string) []int {

	// 使用要寻找的字符串去分割大的字符串
	subs := strings.Split(s, sep)
	here := []int{}
	size := len(sep)

	var i = 0

	// 按分割出的个数找
	for _, sub := range subs {
		i = i + len(sub)
		here = append(here, i)
		i = i + size
	}

	return here[:len(here)-1]
}

func main() {
	// fmt.Println(strings.Split("xabcdaaa", "a"))
	// fmt.Println(whereAmI("xabcdaaa", "d"))
	fmt.Println(unrepeated_substring("axabczzbaaa"))
	fmt.Println(unrepeated_substring("pwwkew"))
	fmt.Println(unrepeated_substring("bbbbb"))
	fmt.Println(unrepeated_substring("abcabcbb"))
	fmt.Println(unrepeated_substring("lengthOfLongestSubstring"))
}

// "abcdabcd" => "a"
//  ==> "", "bcd", "bcd"
