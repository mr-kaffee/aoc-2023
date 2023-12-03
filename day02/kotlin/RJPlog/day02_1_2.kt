import java.io.File

fun cube(in1: Int): Int {
	var result = 0
	
	File("day2302_puzzle_input.txt").forEachLine {
		var gamePossible = true
		var maxRed = 0
		var maxGreen = 0
		var maxBlue = 0
		var instruction = it.split(": ")
		var game = instruction[0].substringAfter("Game ").toInt()
		var results = instruction[1].split("; ")
		results.forEach{
			var colours = it.split(", ")
			colours.forEach{
				if (it.contains("green")) {
					if (it.substringBefore(" green").toInt() > 13) {
						gamePossible = false
					}
					maxGreen = maxOf(maxGreen, it.substringBefore(" green").toInt())
				}
				if (it.contains("red")) {
					if (it.substringBefore(" red").toInt() > 12) {
						gamePossible = false
					}
					maxRed = maxOf(maxRed, it.substringBefore(" red").toInt())	
				}
				if (it.contains("blue")) {
					if (it.substringBefore(" blue").toInt() > 14) {
						gamePossible = false
					}
					maxBlue = maxOf(maxBlue, it.substringBefore(" blue").toInt())
				}
			}
		}
		if (in1 == 1) {
			if (gamePossible) result += game
		} else {  
			result += maxRed * maxGreen * maxBlue

		}
	}
	return result
}

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = cube(1)
	var solution2 = cube(2)

// print solution for part 1
	println("*******************************")
	println("--- Day 2: Cube Conundrum ---")
	println("*******************************")
	println("Solution for part1")
	println("   $solution1 is the sum of the IDs of those games")
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	println("   $solution2 is the sum of the power of these sets")
	println()

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}
