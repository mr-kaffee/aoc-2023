#include <chrono>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

// tag::fileHandler[]
class FileHandler {
public:
  FileHandler(string const &filename) : _file{filename} {}
  ~FileHandler() {
    if (_file.is_open()) {
      _file.close();
    }
  }

  vector<string> readLines() {
    vector<string> lines{};
    string line{};
    while (getline(_file, line)) {
      lines.push_back(line);
    }
    return lines;
  }

private:
  ifstream _file;
};
// end::fileHandler[]

using Clock = std::chrono::system_clock;
using Duration = std::chrono::duration<float, milli>;
static auto constexpr DURATION_UNIT = "ms";

// tag::solve[]
int main() {
  auto start = Clock::now();

  auto lines = FileHandler{"input00"}.readLines();
  for (auto const &line : lines) {
    cout << line << '\n';
  }

  Duration d = Clock::now() - start;
  cout << "Solved puzzle in " << d.count() << DURATION_UNIT << '\n';

  return 0;
}
// end::solve[]
