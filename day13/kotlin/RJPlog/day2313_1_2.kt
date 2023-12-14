import java.io.File
import kotlin.math.*

fun check2(in1: Int = 1, in2 :  MutableList<String>): Int {
    var symCount = 0
    var symValue = 0

	// check for symmetry
	for (i in 2..in2.size step 2) {
		symCount = 0
         var mirrorArea = in2.takeLast(i).chunked(i/2)
		
		 for (j in 0..mirrorArea[0].size-1) {
			 for (k in 0..mirrorArea[0][j].length-1) {
				 	 if (mirrorArea[0][j][k] != mirrorArea[1].reversed()[j][k]) {
							 symCount += 1
				 	 }
				}
			}
         if (symCount == 1) {
             symValue = in2.size-i/2
			 return symValue *100
         }
     }

     // turn to check with same algorithm for horizontal
         var transformedArea = mutableListOf<String>()
         for (y in 0..in2[0].length-1) {
             var line = ""
             for (x in 0..in2.size-1 ) {
                 line += in2[x][y]
             }
             transformedArea.add(line)
         }

	// check for symmetry
	for (i in 2..transformedArea.size step 2) {
		symCount = 0
         var mirrorArea = transformedArea.takeLast(i).chunked(i/2)

		 for (j in 0..mirrorArea[0].size-1) {
			 for (k in 0..mirrorArea[0][j].length-1) {
				 	 if (mirrorArea[0][j][k] != mirrorArea[1].reversed()[j][k]) {
							 symCount += 1
				 	 }
				}
			}
         if (symCount == 1) {
             symValue = transformedArea.size-i/2
			 return symValue
         }
     }

     // turn again to check vertical from bottom
     var transformedArea2 = mutableListOf<String>()
         for (y in transformedArea[0].length-1 downTo 0) {
             var line = ""
             for (x in 0..transformedArea.size-1 ) {
                 line += transformedArea[x][y]
             }
             transformedArea2.add(line)
         }

	// check for symmetry
	for (i in 2..transformedArea2.size step 2) {
		symCount = 0
         var mirrorArea = transformedArea2.takeLast(i).chunked(i/2)
		
		 for (j in 0..mirrorArea[0].size-1) {
			 for (k in 0..mirrorArea[0][j].length-1) {
				 	 if (mirrorArea[0][j][k] != mirrorArea[1].reversed()[j][k]) {
							 symCount += 1
				 	 }
				}
			}
         if (symCount == 1) {
             symValue = 100 * i/2 
			 return symValue
         }
     }

     // and turn again for checking horizontal from bottom
     var transformedArea3 = mutableListOf<String>()
         transformedArea.forEach {
             transformedArea3.add(0,it)
         }

	// check for symmetry
	for (i in 2..transformedArea3.size step 2) {
		symCount = 0
         var mirrorArea = transformedArea3.takeLast(i).chunked(i/2)

		 for (j in 0..mirrorArea[0].size-1) {
			 for (k in 0..mirrorArea[0][j].length-1) {
				 	 if (mirrorArea[0][j][k] != mirrorArea[1].reversed()[j][k]) {
							 symCount += 1
				 	 }
				}
			}
         if (symCount == 1) {
             symValue = i/2 //transformedArea3.size-i/2
			 return symValue
         }
     }
     return -1
}

fun check(in1: Int = 1, in2 :  MutableList<String>): Int {
    var symFound = false
    var symValue = 0

	// check symmetriy for vertical
	for (i in 2..in2.size step 2) {
         var mirrorArea = in2.takeLast(i).chunked(i/2)
         if (mirrorArea[0] == mirrorArea[1].reversed()) {
             symFound = true
             symValue = in2.size-i/2
         }
     }

     if (symFound) { 
			 return symValue*100
     }

     // turn to check with same algorithm for horizontal
         var transformedArea = mutableListOf<String>()
         for (y in 0..in2[0].length-1) {
             var line = ""
             for (x in 0..in2.size-1 ) {
                 line += in2[x][y]
             }
             transformedArea.add(line)
         }

     for (i in 2..transformedArea.size step 2) {
         var mirrorArea = transformedArea.takeLast(i).chunked(i/2)
         if (mirrorArea[0] == mirrorArea[1].reversed()) {
             symFound = true
             symValue = transformedArea.size-i/2
         }
     }

     if (symFound) {
			 return symValue
     }

     // turn again to check vertical from bottom
     var transformedArea2 = mutableListOf<String>()
         for (y in transformedArea[0].length-1 downTo 0) {
             var line = ""
             for (x in 0..transformedArea.size-1 ) {
                 line += transformedArea[x][y]
             }
             transformedArea2.add(line)
         }

     for (i in 2..transformedArea2.size step 2) {
         var mirrorArea = transformedArea2.takeLast(i).chunked(i/2)

         if (mirrorArea[0] == mirrorArea[1].reversed()) {
             symFound = true
             symValue = i/2
         }
     }

     if (symFound) {
			 return 100 * symValue
     }

     // and turn again for checking horizontal from bottom
     var transformedArea3 = mutableListOf<String>()
         transformedArea.forEach {
             transformedArea3.add(0,it)
         }

 
     for (i in 2..transformedArea3.size step 2) {
         var mirrorArea = transformedArea3.takeLast(i).chunked(i/2)
         if (mirrorArea[0] == mirrorArea[1].reversed()) {
             symFound = true
             symValue = i/2
         }
     }
     if (symFound) {
			return symValue
     }
     return -1
}

fun day13(in1: Int): Int {
	var result = 0
	var area = mutableListOf<String>()

	File("day2313_puzzle_input.txt").forEachLine {
		if (it != "") {
        area.add(it)
        } else {
			if (in1 == 1) {
            result += check(in1,area)
			} else {
				result += check2(in1, area)
			}
            area = mutableListOf<String>()  
        }
	}

    if (in1 == 1 ) {
		result += check(in1,area)
	} else {
		result += check2(in1, area)
			}
	         
	return result
}

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = day13(1)
	var solution2 = day13(2)

// print solution for part 1
	println("*******************************")
	println("--- Day 13: Point of Incidence ---")
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
