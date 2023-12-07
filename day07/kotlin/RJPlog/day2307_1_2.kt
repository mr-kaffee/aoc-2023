import java.io.File
import kotlin.math.*

fun camelCard(in1: Int = 1): Long {
	var hands = mutableListOf<Pair<String, Int>>()
	File("day2307_puzzle_input.txt").forEachLine {
		// switching Letters and Numbers to only letters is used for simplifying the sorting later. I don't like this part, but I didn't had a better idea
		
		// for part two 'J' needs to get lower priority
		var joker = "D"
		if (in1 == 2) {
			joker = "Z"
		}
		var hand =
			it.split(" ")[0].replace("K", "B").replace("Q", "C").replace("J", joker).replace("T", "E").replace("9", "F").replace("8", "G").replace("7", "H").replace("6", "I").replace("5", "J").replace("4", "K").replace("3", "L").replace("2", "M")
		hands.add(Pair(hand, it.split(" ")[1].toInt()))
	}
	
	var rank = hands.size
	var cardMap = mutableMapOf<Char, Int>()
	var five = mutableListOf<Pair<String, Int>>()
	var four = mutableListOf<Pair<String, Int>>()
	var full = mutableListOf<Pair<String, Int>>()
	var three = mutableListOf<Pair<String, Int>>()
	var twoPair = mutableListOf<Pair<String, Int>>()
	var onePair = mutableListOf<Pair<String, Int>>()
	var rest = mutableListOf<Pair<String, Int>>()
    
	// cards are sheduled to a list for each type
	hands.forEach {
		cardMap.clear()
		var hand = it.first
		
		// set up a map to count pairs, tripples and other tuples
		hand.forEach {
			if (cardMap.containsKey(it)) {
				cardMap.put(it, cardMap.getValue(it) + 1)
			} else {
				cardMap.put(it, 1)
			}
		}

		// schedule each hand to specific type list. For part two, the joker has to be considered
		var evalList = cardMap.map { it.value }.toList()
		if (evalList.contains(5)) {
			five.add(it)
		} else if (evalList.contains(4)) {
			if (cardMap.containsKey('Z')) {
				five.add(it)
			} else {
			four.add(it)
			}
		} else if (evalList.contains(3) && evalList.contains(2)) {
			if (cardMap.containsKey('Z')) {
				five.add(it)
			} else {
				full.add(it)
			}
		} else if (evalList.contains(3) && !evalList.contains(2)) {
			if (cardMap.containsKey('Z')) {
				four.add(it)
			} else {
				three.add(it)
			}
		} else if (evalList.contains(2)) {
			if (evalList.count { it == 2 } == 2) {
				if (cardMap.containsKey('Z')) {
					if (cardMap.getValue('Z') == 2) {
						four.add(it)
					} else {
						full.add(it)
					}
				} else {
					twoPair.add(it)
				}
			} else {
				if (cardMap.containsKey('Z')) {
						three.add(it)
				} else {
				onePair.add(it)
				}
			}
		} else {
			if (cardMap.containsKey('Z')) {
				onePair.add(it)
			} else {
			rest.add(it)
			}
		}

	}

	var result = 0L
    
	// rearrange list of hands in first (type) and second order (strongness)
	hands.clear()
	hands.addAll(rest.sortedByDescending { it.first })
	hands.addAll(onePair.sortedByDescending { it.first })
	hands.addAll(twoPair.sortedByDescending { it.first })
	hands.addAll(three.sortedByDescending { it.first })
	hands.addAll(full.sortedByDescending { it.first })
	hands.addAll(four.sortedByDescending { it.first })
	hands.addAll(five.sortedByDescending { it.first })

    // calculate result by taking rank into account
    for (i in 1 .. rank) {
		result += hands[i-1].second.toLong() * i.toLong()
	}

	return result
}

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = camelCard()
	var solution2 = camelCard(2)

// print solution for part 1
	println("*******************************")
	println("--- Day 7: Camel Cards ---")
	println("*******************************")
	println("Solution for part1")
	println("   $solution1 are the total winnings")
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	println("   $solution2 total scratchcards do you end up with")
	println()

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}
