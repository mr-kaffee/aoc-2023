import java.io.File
import kotlin.math.*

fun wasteland(in1: Int): Int {
	var lines = File("day2308_puzzle_input.txt").readLines()
	var network = mutableMapOf<String, Pair<String, String>>()
	var instructions = lines[0]
	for (i in 2..lines.size - 1) {
		network.put(
			lines[i].substringBefore(" ="),
			Pair(lines[i].substringAfter("(").substringBefore(","), lines[i].substringAfter(", ").substringBefore(")"))
		)
	}

	var position = "AAA"
	var count = 0

	while (position != "ZZZ") {
		var nextStep = instructions[count % instructions.length]

		if (nextStep == 'R') {
			position = network.getValue(position).second
		} else {
			position = network.getValue(position).first
		}
		count += 1
		if (position.takeLast(1) == "Z") {
			println("$position, $count")
		}
	}
	return count
}

fun wasteland2(in1: Int): Long {
	var lines = File("day2308_puzzle_input.txt").readLines()
	var network = mutableMapOf<String, Pair<String, String>>()
	var instructions = lines[0]
	for (i in 2..lines.size - 1) {
		network.put(
			lines[i].substringBefore(" ="),
			Pair(lines[i].substringAfter("(").substringBefore(","), lines[i].substringAfter(", ").substringBefore(")"))
		)
	}
    
	var position = mutableListOf<String>()
	network.forEach {
		if(it.key.takeLast(1) == "A") {
			position.add(it.key)
		}
	}
	var count = 0L
	var instructionLength = instructions.length.toLong()
    println(position)
    while (position.map {it.last()}.distinct().size != 1 || position.map {it.takeLast(1)}.distinct()[0] != "Z") {
		var nextStep = instructions[(count % instructionLength).toInt()]
        
		for (i in 0..position.size-1)
		if (nextStep == 'R') {
			position[i] = network.getValue(position[i]).second
		} else {
			position[i] = network.getValue(position[i]).first
		}
		count += 1L
		//println("$nextStep: $position")
	} 
	return count
}

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = wasteland(1)
	var solution2 = wasteland2(2)
	//das läuft ewig. Wenn man nur jeweils einen Wert anschaut sieht man, dass man mit einem Eingangswert nur einen Ausgangswert erreicht, und das mit einer bestimmten Periode.
	//Berechnet man daraus das kleinste gemeinsame Vielfache, bekommt man die richtige Lösung. Aber ist das zwingend bei allen Inputs so - theoretisch könnte es mehrere Zielpunkte
	// geben mit unterschiedlichen Frequenzen und Offset? -> ich habe das hier nicht merh angepasst
	//AAA		GPA		VDA		GTA		BBA		VSA
  //ZZZ		CVZ		STZ		FPZ		SKZ		MKZ
  //17287		13771	23147	20803	19631	17873   -> KgV = 18625484023687

// print solution for part 1
	println("*******************************")
	println("--- Day 8: Haunted Wasteland ---")
	println("*******************************")
	println("Solution for part1")
	println("   $solution1 steps are required to reach ZZZ")
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	println("   $solution2 steps does it take before you're only on nodes that end with Z")
	println()

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}
