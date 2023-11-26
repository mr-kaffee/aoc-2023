import java.io.File

// tag::read_input[]
fun readLines(): List<String> {
    return File("../../../inputs/input00").useLines() { it.toList() }
}
// end::read_input[]

// tag::solve[]
fun main() {
    val start = System.currentTimeMillis()

    val lines = readLines()
    for (line in lines) {
        println(line)
    }

    val elapsed = System.currentTimeMillis() - start
    println("Solved puzzle in ${elapsed}ms.")
}
// end::solve[]
