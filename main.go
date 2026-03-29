package main

import (
	"fmt"
	"math/rand/v2"
)

const gridWidth = 7
const gridHeight = 7

var mineCount = 5

func main() {
	grid := generateGrid()

	printGrid(grid)
}

func generateGrid() [][]int {
	grid := make([][]int, gridHeight)

	// allocate rows
	for y := 0; y < gridHeight; y++ {
		grid[y] = make([]int, gridWidth)
	}

	minesPlaced := 0

	for minesPlaced < mineCount {
		x := rand.IntN(gridWidth)
		y := rand.IntN(gridHeight)

		// only place if not already a mine
		if grid[y][x] == 0 {
			grid[y][x] = 1
			minesPlaced++
		}
	}

	return grid
}

func printGrid(grid [][]int) {
	fmt.Println("  A B C D E F G ")

	for y, _ := range grid {
		fmt.Print(y, " ")
		for _, tile := range grid[y] {
			fmt.Print(tile, " ")
		}
		fmt.Println()
	}
}
