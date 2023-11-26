cmake_minimum_required(VERSION 3.12)

set(AOC_DAY "00")

project(peter_day${AOC_DAY} VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# copy input to build folder
file(COPY "../../../inputs/input${AOC_DAY}" DESTINATION "${CMAKE_BINARY_DIR}")

add_executable(solution solution.cpp)
