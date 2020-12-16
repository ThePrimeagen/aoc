#include "vector"
#include <algorithm>
#include <iostream>
#include <fstream>
#include <string>
#include <cmath>
#include <inttypes.h>

std::vector<std::string> getInput() {
    std::vector<std::string> input;
    std::ifstream infile("./d12.input.prod");
    std::string val;

    printf("About to read\n");
    while (getline(infile, val)) {
        printf("line %s\n", val.c_str());
        input.emplace_back(val);
    }

    return input;
}

void rotateR(int x, int y, int* ins) {
    if (x == 1) {
        y = 1;
        x = 0;
    } else if (x == -1) {
        y = -1;
        x = 0;
    } else if (y == 1) {
        x = -1;
        y = 0;
    } else if (y == -1) {
        x = 1;
        y = 0;
    }
    ins[0] = x;
    ins[1] = y;
}

void rotateL(int x, int y, int* ins) {
    if (x == 1) {
        y = -1;
        x = 0;
    } else if (x == -1) {
        y = 1;
        x = 0;
    } else if (y == 1) {
        x = 1;
        y = 0;
    } else if (y == -1) {
        x = -1;
        y = 0;
    }
    ins[0] = x;
    ins[1] = y;
}

int main() {
    int x = 0, y = 0;
    int dx = 1;
    int dy = 0;

    int dirs[2];
    for (const auto& it : getInput()) {
        char dir = it[0];
        int count = std::atoi(it.substr(1).c_str());
        printf("%c%d %d %d ->", dir, count, x, y);

        switch (dir) {
            case 'N':
                y -= count;
                break;
            case 'S':
                y += count;
                break;
            case 'E':
                x += count;
                break;
            case 'W':
                x -= count;
                break;
            case 'F':
                x += dx * count;
                y += dy * count;
                break;
            case 'L':
                count /= 90;
                dirs[0] = dx;
                dirs[1] = dy;
                printf("About to L(%d, %d) %d", dx, dy, count);

                if (count == 1) {
                    rotateL(dx, dy, dirs);
                    dx = dirs[0];
                    dy = dirs[1];
                }
                else if (count == 2) {
                    dx *= -1;
                    dy *= -1;
                }
                else if (count == 3) {
                    rotateR(dx, dy, dirs);
                    dx = dirs[0];
                    dy = dirs[1];
                }

                break;
            case 'R':
                count /= 90;
                dirs[0] = dx;
                dirs[1] = dy;

                if (count == 1) {
                    rotateR(dx, dy, dirs);
                    dx = dirs[0];
                    dy = dirs[1];
                }
                else if (count == 2) {
                    dx *= -1;
                    dy *= -1;
                }
                else if (count == 3) {
                    rotateL(dx, dy, dirs);
                    dx = dirs[0];
                    dy = dirs[1];
                }

                break;
            default:
                break;
        }
        printf("%d %d\n", x, y);
    }

    printf("FINISH %d + %d = %d\n", x, y, std::abs(x) + std::abs(y));
}
