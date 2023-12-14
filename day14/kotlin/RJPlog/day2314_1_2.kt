import java.io.File
import kotlin.math.*

fun day14(in1: Int): Long {
	var width = 0
	var height = 0
	var grid = ""
	var result = 0L

	File("day2314_puzzle_input.txt").forEachLine {
		width = it.length
		grid += it
		height += 1
	}

	var gameEnd = true
	
	while (gameEnd) {
		gameEnd = false
		for (y in 0..height - 2) {
			for (x in 0..width - 1) {
				if (grid[x + y * width] == '.' && grid[x + (y + 1) * width] == 'O') {
					grid = grid.replaceRange(x + y * width, x + y * width + 1, "O")
					grid = grid.replaceRange(x + (y + 1) * width, x + (y + 1) * width + 1, ".")
					gameEnd = true
				}
			}
		}
	}

	grid.chunked(width).forEach {
		result += (it.count { it == 'O' } * height).toLong()
		height -= 1
	}
	return result
}


fun day14Part2(in1: Int): Long {
	var width = 0
	var height = 0
	var grid = ""
	var result = 0L

	File("day2314_puzzle_input.txt").forEachLine {
		width = it.length
		grid += it
		height += 1
	}
	var initGrid = grid
	var heightStart = height

	var i = 0L
	var periodStart = 0L
	var period = 0L
	var overallGameEnd = true
	var cycles = 1000000000L
	
	while(i < cycles){
		var gameEnd = true
		while (gameEnd) {
			gameEnd = false
			for (y in 0..height - 2) {
				for (x in 0..width - 1) {
					if (grid[x + y * width] == '.' && grid[x + (y + 1) * width] == 'O') {
						grid = grid.replaceRange(x + y * width, x + y * width + 1, "O")
						grid = grid.replaceRange(x + (y + 1) * width, x + (y + 1) * width + 1, ".")
						gameEnd = true
					}
				}
			}
		}

		// turn west
		gameEnd = true
		while (gameEnd) {
			gameEnd = false
			for (y in 0..height - 1) {
				for (x in 0..width - 2) {
					if (grid[(x) + y * width] == '.' && grid[(x + 1) + (y) * width] == 'O') {
						grid = grid.replaceRange(x + y * width, x + y * width + 1, "O")
						grid = grid.replaceRange(x + 1 + (y) * width, x + 1 + (y) * width + 1, ".")
						gameEnd = true
					}
				}
			}
		}

		// turn south
		gameEnd = true
		while (gameEnd) {
			gameEnd = false
			for (y in 1..height - 1) {
				for (x in 0..width - 1) {
					if (grid[x + (y) * width] == '.' && grid[x + (y - 1) * width] == 'O') {
						grid = grid.replaceRange(x + (y) * width, x + y * width + 1, "O")
						grid = grid.replaceRange(x + (y - 1) * width, x + (y - 1) * width + 1, ".")
						gameEnd = true
					}
				}
			}
		}

		// turn east
		gameEnd = true
		while (gameEnd) {
			gameEnd = false
			for (y in 0..height - 1) {
				for (x in 1..width - 1) {
					if (grid[(x) + y * width] == '.' && grid[(x - 1) + (y) * width] == 'O') {
						grid = grid.replaceRange(x + y * width, x + y * width + 1, "O")
						grid = grid.replaceRange(x - 1 + (y) * width, x - 1 + (y) * width + 1, ".")
						gameEnd = true
					}
				}
			}
		}
		
		result = 0
		height = heightStart
			grid.chunked(width).forEach {
		result += (it.count { it == 'O' } * height).toLong()
		height -= 1
	}
		height = heightStart

	    if (grid == initGrid && overallGameEnd) {
			period = i-periodStart
			i += period*((cycles-i)/period)
			overallGameEnd = false
			
		}
		
		if(i == 99L) {
			initGrid = grid
			periodStart = i
		}
		i+= 1L
	}

    result = 0
	grid.chunked(width).forEach {
		result += (it.count { it == 'O' } * height).toLong()
		height -= 1
	}
	return result
}

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = day14(1)
	var solution2 = day14Part2(2)

// print solution for part 1
	println("*******************************")
	println("--- Day 14: Parabolic Reflector Dish ---")
	println("*******************************")
	println("Solution for part1")
	println("   $solution1 ")
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	println("   $solution2 ")
	println()

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}
