import java.io.File

fun trebuchet(in1: Int): Int {
                var conMap = mutableMapOf("0" to 1, "1" to 1, "2" to 2, "3" to 3, "4" to 4, "5" to 5, "6" to 6, "7" to 7, "8" to 8, "9" to 9)
                var pattern = """\d""".toRegex()
                if (in1 ==  2) {
                               var conMap2 = mapOf("one" to 1, "two" to 2, "three" to 3, "four" to 4, "five" to 5, "six" to 6, "seven" to 7, "eight" to 8, "nine" to 9)
                               conMap = conMap.plus(conMap2).toMutableMap()
                               pattern = """\d|one|two|three|four|five|six|seven|eight|nine""".toRegex()
                }

                var calibValues = mutableListOf<Int>()
                File("day2301_puzzle_input.txt").forEachLine {
                               var firstDigit = pattern.find(it)!!.value
                               var matches = pattern.findAll(it)
                               var secondDigit = ""
                               matches.forEach{
                                               secondDigit = it.value
                               }
                               calibValues.add(conMap.getValue(firstDigit)*10 + conMap.getValue(secondDigit))
                }
                return calibValues.sum()
}

fun main() {
                var t1 = System.currentTimeMillis()

                var solution1 = trebuchet(1)
                var solution2 = trebuchet(2)

// print solution for part 1
                println("*******************************")
                println("--- Day 1: Trebuchet?! ---")
                println("*******************************")
                println("Solution for part1")
                println("   $solution1 is the sum of all of the calibration values")
                println()
// print solution for part 2
                println("*******************************")
                println("Solution for part2")
                println("   $solution2 is the sum of all of the calibration values")
                println()

                t1 = System.currentTimeMillis() - t1
                println("puzzle solved in ${t1} ms")
}
