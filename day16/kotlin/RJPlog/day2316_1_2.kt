import java.io.File
import kotlin.math.*

fun day16new(x: Int, y :Int,  start: Char): Int {
	var width = 0
	var height = 0
	var mirrors = mutableMapOf<Pair<Int, Int>, Char>()

	File("day2316_puzzle_input.txt").forEachLine {
		width = it.length
		var x = 0
		it.forEach {
			if (it != '.') {
				mirrors.put(Pair(x, height), it)
			}
			x += 1
		}
		height += 1
	}

	var beams = mutableMapOf<Pair<Int, Int>, MutableList<Char>>()
	var currentBeams = mutableMapOf<Pair<Int, Int>, MutableList<Char>>()
	var currentBeamsNew = mutableMapOf<Pair<Int, Int>, MutableList<Char>>()
	currentBeams.put(Pair(x, y), mutableListOf<Char>(start))

	for (i in 0..5000) {   // this is not opitimal - don't fill currentBeamsNew Points already visited
  // To Do
  // don't store in beams '#' but a list of all directions a beam is running through
  // when filling currentBeamsNew fill it only with drictions not already stored a the same location in beams  
 
	//while (!currentBeams.isEmpty()) {

		// für jeden Eintrag in Map currentBeams
		for ((key, value) in currentBeams) {
			var x = key.first
			var y = key.second
			// für jeden Eintrag in Value
			//println(" currentBeams $currentBeams, actual key: $key, acutal value $value")
			value.forEach {
				var newDirection = mutableListOf<Char>(it)
				//println("   newDirection $newDirection")
				if (it == '>') {
					if ((x + 1) < width) {  // check next Position, if in field
						if (mirrors.containsKey(Pair(x + 1, y))) {  // check next position if mirror or not
							when (mirrors.getValue(Pair(x + 1, y))) {
								'/' -> newDirection = mutableListOf<Char>('^')
								'|' -> newDirection = mutableListOf<Char>('^', 'v')
								'\\' -> newDirection = mutableListOf<Char>('v')
							}

						}
						beams.put(Pair(x + 1, y), mutableListOf<Char>('#'))
						if(currentBeamsNew.containsKey(Pair(x+1,y))) {
							var entry = currentBeamsNew.getValue(Pair(x+1,y))
							entry.addAll(newDirection)
						currentBeamsNew.put(Pair(x + 1, y), entry.distinct().toMutableList())
						} else {
							currentBeamsNew.put(Pair(x + 1, y), newDirection)
						}
					}
				} else if (it == '^') {
					if ((y - 1) >= 0) {
						if (mirrors.containsKey(Pair(x, y - 1))) {  // check next position if mirror or not
							when (mirrors.getValue(Pair(x, y - 1))) {
								'/' -> newDirection = mutableListOf<Char>('>')
								'-' -> newDirection = mutableListOf<Char>('<', '>')
								'\\' -> newDirection = mutableListOf<Char>('<')
							}
						}
						beams.put(Pair(x, y - 1), mutableListOf<Char>('#'))
						if(currentBeamsNew.containsKey(Pair(x,y-1))) {
							var entry = currentBeamsNew.getValue(Pair(x,y-1))
							entry.addAll(newDirection)
						currentBeamsNew.put(Pair(x, y-1), entry.distinct().toMutableList())
						} else {
							currentBeamsNew.put(Pair(x, y-1), newDirection)
						}
					}
				} else if (it == '<') {
					if ((x - 1) >= 0) {
						if (mirrors.containsKey(Pair(x - 1, y))) {  // check next position if mirror or not
							when (mirrors.getValue(Pair(x - 1, y))) {
								'/' -> newDirection = mutableListOf<Char>('v')
								'|' -> newDirection = mutableListOf<Char>('^', 'v')
								'\\' -> newDirection = mutableListOf<Char>('^')
							}
						}
						// now add to newBeams if not already in.
						beams.put(Pair(x - 1, y), mutableListOf<Char>('#'))
						if(currentBeamsNew.containsKey(Pair(x-1,y))) {
							var entry = currentBeamsNew.getValue(Pair(x-1,y))
							entry.addAll(newDirection)
						currentBeamsNew.put(Pair(x - 1, y), entry.distinct().toMutableList())
						} else {
							currentBeamsNew.put(Pair(x- 1, y), newDirection)
						}
					}
				} else if (it == 'v') {
					if ((y + 1) < height) {
						if (mirrors.containsKey(Pair(x, y + 1))) {  // check next position if mirror or not
							when (mirrors.getValue(Pair(x, y + 1))) {
								'/' -> newDirection = mutableListOf<Char>('<')
								'-' -> newDirection = mutableListOf<Char>('<', '>')
								'\\' -> newDirection = mutableListOf<Char>('>')
							}
						}
						// now add to newBeams if not already in.
						beams.put(Pair(x, y + 1), mutableListOf<Char>('#'))
						if(currentBeamsNew.containsKey(Pair(x,y+1))) {
							var entry = currentBeamsNew.getValue(Pair(x,y+1))
							entry.addAll(newDirection)
						currentBeamsNew.put(Pair(x, y+1), entry.distinct().toMutableList())
						} else {
							currentBeamsNew.put(Pair(x, y+1), newDirection)
						}
					}
				}			
			}
		}

		currentBeams.clear()
		currentBeams.putAll(currentBeamsNew)
		currentBeamsNew.clear()
	}

	return beams.size
}

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = day16new(-1,0,'>')
	
	var maxBeam = 0
	for (i in 0..109) {
		var maxBeamX = day16new(-1,i, '>')
		var maxBeamY = day16new(-1,i, 'v')
		var maxBeamXX = day16new(110,i, '<')
		var maxBeamYY= day16new(110,i, '^')
		if (maxBeamX > maxBeam) {
			maxBeam = maxBeamX
		}
		if (maxBeamY > maxBeam) {
			maxBeam = maxBeamY
		}
		if (maxBeamXX > maxBeam) {
			maxBeam = maxBeamXX
		}
		if (maxBeamYY > maxBeam) {
			maxBeam = maxBeamYY
		}
	}
	
	var solution2 = maxBeam

// print solution for part 1
	println("*******************************")
	println("--- Day 16: The Floor Will Be Lava ---")
	println("*******************************")
	println("Solution for part1")
	println("   $solution1 ")   // 4806 / 5776 is to low
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	println("   $solution2 ")
	println()

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}
