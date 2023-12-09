import java.io.File
import kotlin.math.*

fun day09(in1: Int): Long {
	var result = 0L
	var lines = File("day2309_puzzle_input.txt").readLines()

	lines.forEach {

		var line = it.split(" ").map { it.toLong() }.toList()
		var interpolationValues = mutableListOf<Long>()
		var diffList = mutableListOf<Long>()
		
		if (in1 == 1) {
			diffList = line.windowed(2).map { it[1] - it[0] }.toMutableList()
			interpolationValues.add(line.last())
			interpolationValues.add(diffList.last())
		} else {
			diffList = line.windowed(2).map { it[0] - it[1] }.toMutableList()
			interpolationValues.add(line.first())
			interpolationValues.add(diffList.first())
		}
		while (diffList.distinct().size > 1 || diffList[0] != 0L) {
			if (in1 == 1) {
				diffList = diffList.windowed(2).map { it[1] - it[0] }.toMutableList()
				interpolationValues.add(diffList.last())
			} else {
				diffList = diffList.windowed(2).map { it[0] - it[1] }.toMutableList()
				interpolationValues.add(diffList.first())
			}
		}
		result += interpolationValues.sum()
	}
	return result
}

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = day09(1)
	var solution2 = day09(2)

// print solution for part 1
	println("*******************************")
	println("--- Day 9: Mirage Maintenance ---")
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
