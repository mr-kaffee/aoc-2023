import java.io.File
import kotlin.math.*

fun gearRatio(): Int {
	var result = 0
	val patternSymbol = """\W""".toRegex()
	val patternPartNumber = """(\d)+""".toRegex()
	var y = 0

	File("day2303_puzzle_input.txt").forEachLine {
		var matchSymbol = patternSymbol.findAll(it.replace(".", "a"))
		matchSymbol.forEach {
			var x = it.range.first
			//println("${it.value}: $y $x")
			var yPartNumber = 0
			File("day2303_puzzle_input.txt").forEachLine {
				var matchPartNumber = patternPartNumber.findAll(it)
				matchPartNumber.forEach {
					//println("   ${it.value.toInt()}: $yPartNumber, ${it.range}")
					var rangeCheck = IntRange(it.range.first-1, it.range.last + 1)
					if (abs(y - yPartNumber) < 2 && rangeCheck.contains(x)) {
						//println("  ->  ok")
						// if there are more than one symbols adjacent to a number, this has to be reworked.
						// (swich order, look for partNumber and check for adjacent symbols, break after
						// first symbol is found.)
						result += it.value.toInt()
					}
				}
				yPartNumber += 1
			}
		}
		y += 1
	}
	return result
}

fun gearRatioPart2(): Int {
	var result = 0
	val patternSymbol = """\*""".toRegex()
	val patternPartNumber = """(\d)+""".toRegex()
	var y = 0

	File("day2303_puzzle_input.txt").forEachLine {
		var matchSymbol = patternSymbol.findAll(it)
		matchSymbol.forEach {
			var gearRatio = 1
			var gearCount = 0
			var x = it.range.first
			var yPartNumber = 0
			File("day2303_puzzle_input.txt").forEachLine {
				var matchPartNumber = patternPartNumber.findAll(it)
				matchPartNumber.forEach {
					var rangeCheck = IntRange(it.range.first-1, it.range.last + 1)
					if (abs(y - yPartNumber) < 2 && rangeCheck.contains(x)) {
						gearRatio *= it.value.toInt()
						gearCount += 1
					}
				}
				yPartNumber += 1
			}
			if (gearCount == 2) result += gearRatio
		}
		y += 1
	}
	return result
}

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = gearRatio()
	var solution2 = gearRatioPart2()

// print solution for part 1
	println("*******************************")
	println("--- Day 3: Gear Ratios ---")
	println("*******************************")
	println("Solution for part1")
	println("   $solution1 is the sum of all of the part numbers in the engine schematic")
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	println("   $solution2 is the sum of all of the gear ratios in your engine schematic")
	println()

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}
