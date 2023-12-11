import java.io.File
import kotlin.math.*

fun day11(in1: Int): Long {
	var expansionValue = 1
	if (in1 == 2) {
		expansionValue = 1000000 - 1
	}

	var universe: String = ""
	var xDim = 0
	var yDim = 0
	var galaxyMap = mutableMapOf<Int, Pair<Int, Int>>()
	var galaxyCount = 1

	File("day2311_puzzle_input.txt").forEachLine {
		universe += it
		xDim = it.length

		var line = it
		for (i in 0..line.length - 1) {
			if (line[i] == '#') {
				galaxyMap.put(galaxyCount, Pair(i, yDim))
				galaxyCount += 1
			}
		}
		yDim += 1
		// expand universe in yDim:
		if (!line.contains('#')) {
			yDim += expansionValue
		}
	}

	// determine at which xDim the universe will expand:
	var xDimExpandCount = mutableListOf<Int>()
	for (i in 0..xDim - 1) {
		if (!universe.chunked(xDim).map { it[i] }.contains('#')) {
			xDimExpandCount.add(i)
		}
	}

	for (key in galaxyMap.keys) {
		for (i in xDimExpandCount.size - 1 downTo 0) {
			if (galaxyMap.getValue(key).first > xDimExpandCount[i]) {
				galaxyMap.put(
					key,
					Pair(galaxyMap.getValue(key).first + (expansionValue), galaxyMap.getValue(key).second)
				)
			}
		}
	}

	var result = 0L
	for ((key, value) in galaxyMap) {
		for ((key2, value2) in galaxyMap) {
			if (key != key2) {
				result += (abs(value2.second - value.second) + abs(value2.first - value.first)).toLong()
			}
		}
	}

	return result / 2
}

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = day11(1)
	var solution2 = day11(2)

// print solution for part 1
	println("*******************************")
	println("--- Day 11: Cosmic Expansion ---")
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
