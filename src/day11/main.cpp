#include "vector"
#include <algorithm>
#include <iostream>
#include <fstream>
#include <string>
#include <inttypes.h>

void getInput(std::vector<std::string>& input) {
    std::ifstream infile("./d11.input.prod");
    std::string val;

    printf("About to read\n");
    while (getline(infile, val)) {
        printf("line %s\n", val.c_str());
        input.emplace_back(val);
    }
}

bool inBounds(std::vector<std::string>& input, int x, int y) {
    return y >= 0 && y < input.size() &&
        x >= 0 && x < input[0].size();
}

namespace {
int dirs[8][2] = {
    {-1, -1},
    {0, -1},
    {1, -1},
    {1, 0},
    {1, 1},
    {0, 1},
    {-1, 1},
    {-1, 0},
};
}

int countInDir(std::vector<std::string>& input, int x, int y, int dirs[2]) {
    int _x = x, _y = y;
    int count = 0;
    while (true) {
        _x += dirs[0];
        _y += dirs[1];

        if (!inBounds(input, _x, _y)) {
            break;
        }

        if (input[_y][_x] == '#') {
            count = 1;
            break;
        }
        else if (input[_y][_x] == 'L') {
            break;
        }
    }

    return count;
}

int countPoundMesV2(std::vector<std::string>& input, int x, int y) {
    int count = 0;
    for (int i = 0; i < 8; ++i) {
        count += countInDir(input, x, y, dirs[i]);
    }

    return count;
}

int countPoundMes(std::vector<std::string>& input, int x, int y) {
    int count = 0;
    for (int i = 0; i < 8; ++i) {
        int _x = x + dirs[i][0];
        int _y = y + dirs[i][1];

        bool bounded = inBounds(input, _x, _y);
        if (bounded && input[_y][_x] == '#') {
            count++;
        }

    }

    return count;
}

void printMap(std::vector<std::string>& input) {
    for (const auto& it : input) {
        printf("%s\n", it.c_str());
    }
}

int occupySeats(std::vector<std::string> &input) {
  int count = 0;
  for (const auto &row : input) {
    for (const auto &col : row) {
      if (col == '#') {
        count++;
      }
    }
  }

  return count;
}

int main() {
    std::vector<std::string> current;
    std::vector<std::string> next;
    int inputIdx = 0;

    std::string doesThis = "foo";
    doesThis[1] = 'a';

    getInput(current);
    getInput(next);

    size_t w = current[0].size();
    size_t h = current.size();

    bool changed = false;
    int changeCount = 0;

    do {
        changed = false;

        for (int y = 0; y < h; ++y) {
            for (int x = 0; x < w; ++x) {
                if (current[y][x] == '.') {
                    continue;
                }

                int count = countPoundMesV2(current, x, y);

                // rule 1: if empty everywhere, sit
                if (count == 0 && current[y][x] == 'L') {
                    next[y][x] = '#';
                    changed = true;
                }

                else if (count >= 5 && current[y][x] == '#') {
                    next[y][x] = 'L';
                    changed = true;
                }
            }
        }

        for (int y = 0; y < h; ++y) {
            current[y] = next[y];
        }

        if (changed) {
            changeCount++;
        }

        printf("Change Count: %d\n", changeCount);
    } while (changed);

    printMap(current);
    printf("Occupied seats %d\n", occupySeats(current));
}

