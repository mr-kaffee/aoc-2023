# Advent of Code 2023 solutions

This repository contains solutions to the [Advent of Code 2023](https://adventofcode.com/2023/) puzzles.

Documented results can be found on [aoc-2023.die-wielands.net](https://aoc-2023.die-wielands.net/)

## Join Coding

Feel free to clone the repository and submit PRs for your solutions. Please add documentation in a `README.adoc` file per day and adjust the VS code `.devcontainer` configuration to make sure your code can be executed in the development container if needed.

If you want, you may join the [private leaderboard](https://adventofcode.com/2023/leaderboard/private/view/878630) using the code `878630-dafd1c4f` - but keep in mind: advent of code should be first and foremost a fun event and a learning opportunity.

### License

All code in this repository is licensed under the terms of the *MIT License*. Any contribution to this repository must be licensable under the MIT license.

## Repository Structure

The repository follows the structure detailed below. There is one top-level folder `day[xx]` for each day, a subfolder `[language]` for each language used to write solutions and a subfolder `[user]` for each contributor to this repository (please consider using a name that allows others to identify you, e.g., your github or advent of code user name)

```
+- README.md
+- inputs
|  +- day01
|  +- day02
|  +- day03
|  +- ...
+- day00
|  +- ...
+- day01
|  +- rust
|  |  +- peter
|  |  |  +- [code files]
|  |  |  +- README.adoc
|  +- [language]
|     +- [user]
|        +- [code files]
|        +- README.adoc
+- day02
+- day03
+- ...
```
### Hello World

Feel free to use the `day00` subfolder for a hello world example and to introduce yourself!

### Input Files

The makers of Advent of Code request to not publish or collect puzzle inputs, so we should not include the puzzle inputs in the repository. To make it easy to run solutions, please put your inputs in the `inputs` subfolder which is excluded from the repository through the `.gitignore` file. This way, everybody can run all the solutions using her/his own input.
