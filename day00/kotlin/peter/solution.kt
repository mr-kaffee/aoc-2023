import java.io.File

// tag::read_input[]
fun readLines(): List<String> {
    var lines = mutableListOf<String>()
    File("../../../inputs/input00").forEachLine() { lines.add(it) }
    return lines
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
