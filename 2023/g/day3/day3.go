package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

/*
--- Day 3: Gear Ratios ---

You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
*/

func output_answer(schematic [][]string) {
	sum := 0
	for num := range find_nums(schematic) {
		//fmt.Println(num)
		sum += num
	}
	fmt.Println(sum)
}

func main() {
	fmt.Println("Day 3:")
	test_schematic := `467..114..
	...*......
	..35..633.
	......#...
	617*......
	.....+.58.
	..592.....
	......755.
	...$.*....
	.664.598..`
	scanner := bufio.NewScanner(strings.NewReader(test_schematic))
	schematic := parse_schematic(scanner)
	output_answer(schematic)
	fmt.Println("And now the real answer")
	inputFile, err := os.Open("2023d3p1.txt")
	if err != nil {
		log.Fatal(err)
	}
	inschematic := parse_schematic(bufio.NewScanner((bufio.NewReader(inputFile))))
	output_answer(inschematic)
}

func parse_schematic(scanner *bufio.Scanner) [][]string {
	result := [][]string{}
	row := 0
	for scanner.Scan() {
		line := strings.Replace(scanner.Text(), "\t", "", -1)
		result = append(result, make([]string, len(line)))
		for col, runeValue := range line {
			result[row][col] = string(runeValue)
		}
		row += 1
	}
	return result
}

func is_str_symbolic(str string) bool {
	if str == "." {
		return false
	}
	_, err := strconv.Atoi(str)
	return err != nil
}

func is_pos_symbolic(schematic [][]string, row int, col int) bool {
	if row < 0 || col < 0 {
		return false
	}
	if row > len(schematic)-1 || col > len(schematic[0])-1 {
		return false

	}
	return is_str_symbolic(schematic[row][col])
}

func any_symbols_touch(schematic [][]string, row int, col int) bool {
	offsets := []int{-1, 0, 1}
	for _, offset := range offsets {
		if is_pos_symbolic(schematic, row-1, col+offset) {
			return true
		}
		if is_pos_symbolic(schematic, row+1, col+offset) {
			return true
		}
	}
	if is_pos_symbolic(schematic, row, col-1) {
		return true
	}
	if is_pos_symbolic(schematic, row, col+1) {
		return true
	}
	return false
}

func flush(curnum string, c chan int) {
	if curnum == "" {
		return
	}
	intNum, err := strconv.Atoi(curnum)
	if err != nil {
		panic(fmt.Sprintf("WTF? curnum = [%s]", curnum))
	}
	c <- intNum
}

/*
for each row, list in enumerate schematic

	  curnum = nul
	  curnum_is_partnum = false
	  for each column, char in enumerate row
		if char is numeric
		  push into curnum
		  if not curnum_is_partnum
		    if any_symbols_touch(row, column)
				curnum_is_partnum = true
		else
			if curnum_is_partnum
				yield curnum as int
*/
func find_nums(schematic [][]string) chan int {
	c := make(chan int)
	go func() {
		for row, line := range schematic {
			curnum := ""
			curnum_is_partnum := false
			for col, char := range line {
				_, err := strconv.Atoi(char)
				if err == nil {
					curnum += string(char)
					if !curnum_is_partnum {
						if any_symbols_touch(schematic, row, col) {
							curnum_is_partnum = true
						}
					}
				} else {
					if curnum_is_partnum {
						flush(curnum, c)
					}
					curnum = ""
					curnum_is_partnum = false
				}
			}
			if curnum_is_partnum {
				flush(curnum, c)
			}
		}
		close(c)
	}()
	return c
}
