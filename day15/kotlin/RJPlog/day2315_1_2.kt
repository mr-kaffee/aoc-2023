import java.io.File
import kotlin.math.*

fun label(in1: String): Int {

	var currentValue = 0

	in1.forEach {
		currentValue += it.toInt()
		currentValue *= 17
		currentValue = currentValue % 256
	}

	return currentValue
}

fun day15(in1: Int): Int {

	var result = 0
	var puzzleInput = ""

	File("day2315_puzzle_input.txt").forEachLine {
		puzzleInput += it
	}

	if (in1 == 1) {
		var currentValue = 0

		puzzleInput.split(",").forEach {
			currentValue = 0
			it.forEach {
				currentValue += it.toInt()
				currentValue *= 17
				currentValue = currentValue % 256
			}
			result += currentValue
		}
		return result
	} else {
		
		var boxes = mutableMapOf<Int, MutableList<Pair<String, Int>>>()
		var currentBox = 0
		var currentContent = mutableListOf<Pair<String, Int>>()
		
		puzzleInput.split(",").forEach {
			if (it.contains("-")) {
				currentBox = label(it.dropLast(1))
				if (boxes.containsKey(currentBox)) {
					currentContent = boxes.getValue(currentBox)
				} else {
					currentContent = mutableListOf<Pair<String, Int>>()
				}

				var indexToRemove = -1

				for (i in 0..currentContent.size - 1) {
					if (currentContent[i].first == it.dropLast(1)) {
						indexToRemove = i
					}
				}
				if (indexToRemove >= 0) {
					currentContent.removeAt(indexToRemove)
					boxes.put(currentBox, currentContent)
				}

			} else {
				var (lens, focal) = it.split("=")

				currentBox = label(lens)
				if (boxes.containsKey(currentBox)) {
					currentContent = boxes.getValue(currentBox)
				} else {
					currentContent = mutableListOf<Pair<String, Int>>()
				}

				var lensExchange = false
				
				for (i in 0..currentContent.size - 1) {
					if (currentContent[i].first == lens) {
						currentContent[i] = Pair(lens, focal.toInt())
						lensExchange = true
					}
				}

				if (!lensExchange) {
					currentContent.add(Pair(lens, focal.toInt()))
				}
				boxes.put(currentBox, currentContent)
			}
		}

		for ((key, value) in boxes) {
			for (i in 0..value.size - 1) {
				result += (key + 1) * (value[i].second) * (i + 1)
			}
		}
		return result
	}
}


fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = day15(1)
	var solution2 = day15(2)

// print solution for part 1
	println("*******************************")
	println("--- Day 15: Lens Library ---")
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
