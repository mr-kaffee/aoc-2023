import java.io.File
import kotlin.math.*

fun scratchCard(): Int {
	var result = 0
	val pattern = """(\d)+""".toRegex()
	File("day2304_puzzle_input.txt").forEachLine {
		var winningNumbers = pattern.findAll(it.substringAfter(": ").split(" | ")[0]).map { it.value }.toList()
		var numbersYouHave = pattern.findAll(it.substringAfter(": ").split(" | ")[1]).map { it.value }.toList()
		result += Math.pow(
			2.0,
			((winningNumbers + numbersYouHave).size - (winningNumbers + numbersYouHave).distinct().size).toDouble() - 1.0
		).toInt()

	}
	return result
}

fun scratchCardPart2(): Int {
	var puzzle_input = mutableListOf<String>()
	File("day2304_puzzle_input.txt").forEachLine {
		puzzle_input.add(it)
	}
	val pattern = """(\d)+""".toRegex()
	val wonCards = puzzle_input.toMutableList()
	val wonCardsIterator = wonCards.listIterator()

	while (wonCardsIterator.hasNext()) {
		var card = wonCardsIterator.next()
		var cardNumber = pattern.find(card.substringBefore(":"))!!.value.toInt()
		var winningNumbers = pattern.findAll(card.substringAfter(": ").split(" | ")[0]).map { it.value }.toList()
		var numbersYouHave = pattern.findAll(card.substringAfter(": ").split(" | ")[1]).map { it.value }.toList()
		var wins = (winningNumbers + numbersYouHave).size - (winningNumbers + numbersYouHave).distinct().size

		for (i in 0..wins - 1) {
			wonCardsIterator.add(puzzle_input[cardNumber + i])
			wonCardsIterator.previous()
		}
	}
	return wonCards.size
}


fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = scratchCard()
	var solution2 = scratchCardPart2()

// print solution for part 1
	println("*******************************")
	println("--- Day 4: Scratchcards ---")
	println("*******************************")
	println("Solution for part1")
	println("   $solution1 points are they worth in total")
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	println("   $solution2 total scratchcards do you end up with")
	println()

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}
