import java.io.File
import kotlin.math.*

fun fertilizer(): Long {
	var locationList = mutableListOf<Long>()
	var newLocationList = mutableListOf<Long>()

	File("day2305_puzzle_input.txt").forEachLine {
		if (it.contains("seeds: ")) {
			locationList = it.substringAfter("seeds: ").split(" ").map { it.toLong() }.toMutableList()
			newLocationList = it.substringAfter("seeds: ").split(" ").map { it.toLong() }.toMutableList()
		}

		if (it.length != 0 && it.first().isDigit()) {
			var convertInstruction = it.split(" ")
			var range =
				LongRange(
					convertInstruction[1].toLong(),
					convertInstruction[1].toLong() + convertInstruction[2].toLong() - 1
				)
			for (i in 0..locationList.size - 1) {
				if (range.contains(locationList[i])) {
					newLocationList[i] =
						locationList[i] - convertInstruction[1].toLong() + convertInstruction[0].toLong()
				}
			}
		}
		if (it.length == 0) {
			locationList.clear()
			locationList.addAll(newLocationList)
			//println (newLocationList)
		}
	}

	return newLocationList.min()!!
}

fun main() {
	var t1 = System.currentTimeMillis()
	
	var solution1 = fertilizer()
	
	// part 2 brutal force - takes 2 days
	var solution2: Long = -1
	var instruction = File("day2305_puzzle_input.txt").readLines()
	var seeds = instruction[0]
	var seedRanges = seeds.substringAfter("seeds: ").split(" ").map { it.toLong() }.toList()

	seedRanges.chunked(2).forEach {
		var i: Long = 0
		while (i < it.last()) {

			var location: Long = it.first() + i
			var newLocation: Long = location
			instruction.forEach {

				if (it.length != 0 && it.first().isDigit()) {
					var convertInstruction = it.split(" ")
					var range =
						LongRange(
							convertInstruction[1].toLong(),
							convertInstruction[1].toLong() + convertInstruction[2].toLong() - 1
						)
					if (range.contains(location)) {
						newLocation = location - convertInstruction[1].toLong() + convertInstruction[0].toLong()
					}

				}
				if (it.length == 0) {
					location = newLocation
				}
			}

			var x = newLocation
			if (x < solution2 || solution2 < 0) solution2 = x
			i += 1
		}		
	}


// print solution for part 1
	println("*******************************")
	println("--- Day 5: If You Give A Seed A Fertilizer ---")
	println("*******************************")
	println("Solution for part1")
	println("   $solution1 is the lowest location number that corresponds to any of the initial seed numbers")
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	println("   $solution2 is the lowest location number that corresponds to any of the initial seed numbers")
	println()

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}
