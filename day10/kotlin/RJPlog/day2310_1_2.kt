import java.io.File

fun day10(in1: Int): Int {
	var lines: String = ""
	var width: Int = 0
	var height: Int = 0
	var xStart = 0
	var yStart = 0

	File("day2310_puzzle_input.txt").forEachLine {
		width = it.length
		height += 1
		if (it.contains('S')) {
			xStart = it.indexOf('S') + 1
			yStart = height
		}
		lines += "*" + it + "*"
	}
	lines = "*".repeat(width + 2) + lines + "*".repeat(width + 2)
	width += 2
	height += 2

	// create pipe plan by replacing 'S' with correct Pipe Pice
	var pipePlan = lines

	if (lines[(xStart + 1) + yStart * width] == '-' || lines[(xStart + 1) + yStart * width] == '7' || lines[(xStart + 1) + yStart * width] == 'J') {
		if (lines[xStart + (yStart + 1) * width] == '|' || lines[xStart + (yStart + 1) * width] == 'J' || lines[xStart + (yStart + 1) * width] == 'L') {
			pipePlan = lines.replace("S", "F")
		} else if (lines[xStart + (yStart - 1) * width] == '|' || lines[xStart + (yStart - 1) * width] == '7' || lines[xStart + (yStart - 1) * width] == 'F') {
			pipePlan = lines.replace("S", "L")
		} else if (lines[(xStart - 1) + (yStart) * width] == '-' || lines[(xStart - 1) + (yStart) * width] == 'L' || lines[(xStart - 1) + (yStart) * width] == 'F') {
			pipePlan = lines.replace("S", "-")
		}
	} else if (lines[(xStart - 1) + yStart * width] == '-' || lines[(xStart - 1) + yStart * width] == 'L' || lines[(xStart - 1) + yStart * width] == 'F') {
		if (lines[xStart + (yStart + 1) * width] == '|' || lines[xStart + (yStart + 1) * width] == 'J' || lines[xStart + (yStart + 1) * width] == 'L') {
			pipePlan = lines.replace("S", "7")
		} else if (lines[xStart + (yStart - 1) * width] == '|' || lines[xStart + (yStart - 1) * width] == '7' || lines[xStart + (yStart - 1) * width] == 'F') {
			pipePlan = lines.replace("S", "J")
		}
	} else {
		pipePlan = lines.replace("S", "|")
	} 

	// run through grid an mark the path
	var gameEnd = true

	while (gameEnd) {
		gameEnd = false
		for (y in 1..height - 2) {
			for (x in 1..width - 2) {
				if (lines[x + y * width] != 'S') {
					if (lines[(x - 1) + y * width] == 'S' && (pipePlan[(x - 1) + y * width] == 'F' || pipePlan[(x - 1) + y * width] == '-' || pipePlan[(x - 1) + y * width] == 'L')) {
						lines = lines.replaceRange(x + y * width, x + 1 + y * width, "S")
						gameEnd = true
					} else if (lines[(x + 1) + y * width] == 'S' && (pipePlan[(x + 1) + y * width] == '7' || pipePlan[(x + 1) + y * width] == '-' || pipePlan[(x + 1) + y * width] == 'J')) {
						lines = lines.replaceRange(x + y * width, x + 1 + y * width, "S")
						gameEnd = true
					} else if (lines[(x) + (y - 1) * width] == 'S' && (pipePlan[(x) + (y - 1) * width] == '7' || pipePlan[(x) + (y - 1) * width] == '|' || pipePlan[(x) + (y - 1) * width] == 'F')) {
						lines = lines.replaceRange(x + y * width, x + 1 + y * width, "S")
						gameEnd = true
					} else if (lines[(x) + (y + 1) * width] == 'S' && (pipePlan[(x) + (y + 1) * width] == 'L' || pipePlan[(x) + (y + 1) * width] == '|' || pipePlan[(x) + (y + 1) * width] == 'J')) {
						lines = lines.replaceRange(x + y * width, x + 1 + y * width, "S")
						gameEnd = true
					}
				}
			}
		}
	}

	// part two
	var result = 0
	var directionPointer = '*'
	for (y in 1..height - 2) {
		var sCount = 0
		for (x in 1..width - 2) {
			if (lines[x + y * width] == 'S') {
				// set directionPointer
				if (pipePlan[x + y * width] == 'F' || pipePlan[x + y * width] == 'L') {
					directionPointer = pipePlan[x + y * width]
				}
				// set counter
				if (pipePlan[x + y * width] == '|') {
					sCount += 1
				} else if (pipePlan[x + y * width] == 'J' && directionPointer == 'F') {
					sCount += 1
				} else if (pipePlan[x + y * width] == '7' && directionPointer == 'L') {
					sCount += 1
				}
			} else {
				if (sCount % 2 != 0) {
					result += 1
				}
			}
		}
	}

	if (in1 == 1) {
		return lines.count { it == 'S' } / 2
	} else {
		return result
	}
}

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = day10(1)
	var solution2 = day10(2)

// print solution for part 1
	println("*******************************")
	println("--- Day 10: Pipe Maze ---")
	println("*******************************")
	println("Solution for part1")
	println("   $solution1 ")
	println()
// print solution for part 2  564 is to high
	println("*******************************")
	println("Solution for part2")
	println("   $solution2 ")
	println()

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}
