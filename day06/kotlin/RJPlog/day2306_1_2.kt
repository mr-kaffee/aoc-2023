import java.io.File

fun waitForIt(in1: Int): Long {
	
	val pattern = """(\d)+""".toRegex()
	var lines = File("day2306_puzzle_input.txt").readLines()

	var line0 = pattern.findAll(lines[0]).map { it.value }.toList()
	var line1 = pattern.findAll(lines[1]).map { it.value }.toList()

	var time = mutableListOf<Pair<Long, Long>>()

	if (in1 == 1) {
		for (i in 0..line0.size - 1) {
			time.add(Pair(line0[i].toLong(), line1[i].toLong()))
		}
	} else {
		time.add(Pair(line0.joinToString("").toLong(), line1.joinToString("").toLong()))
	}

	var result = 1L
	time.forEach {
		var wins = 0L
		for (i in 0..it.first) {
			var dist = i * (it.first - i)
			if (dist > it.second) {
				wins += 1L
			}
		}
		result *= wins
	}
	return result
}

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = waitForIt(1)
	var solution2 = waitForIt(2)

// print solution for part 1
	println("*******************************")
	println("--- Day 6: Wait For It ---")
	println("*******************************")
	println("Solution for part1")
	println("   $solution1 do you get if you multiply these numbers together")
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	println("   $solution2 many ways can you beat the record in this one much longer race")
	println()

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}
