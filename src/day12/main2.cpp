#include "vector"
#include <algorithm>
#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cmath>
#include <inttypes.h>

struct Coords {
    std::string name;
    int x;
    int y;

    std::string toString() {
        std::ostringstream ss;
        ss << name << "(" << x << ", " << y << ")";
        return ss.str();
    }
};

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

void rotateR(Coords &coords, int count) {
    for (int i = 0; i < count; ++i) {
        coords.x = coords.x ^ coords.y;
        coords.y = coords.x ^ coords.y;
        coords.x = (coords.x ^ coords.y) * -1;
    }
}

void rotateL(Coords &coords, int count) {
    for (int i = 0; i < count; ++i) {
        coords.x = coords.x ^ coords.y;
        coords.y = coords.x ^ coords.y;
        coords.x = coords.x ^ coords.y;
        coords.y *= -1;
    }
}

int main() {
    Coords ship = {"ship", 0, 0};
    Coords wp = {"wp", 10, -1};

    for (const auto& it : getInput()) {
        char dir = it[0];
        int count = std::atoi(it.substr(1).c_str());
        printf("%c%d %s %s -> ", dir, count, ship.toString().c_str(), wp.toString().c_str());

        switch (dir) {
            case 'N':
                wp.y -= count;
                break;
            case 'S':
                wp.y += count;
                break;
            case 'E':
                wp.x += count;
                break;
            case 'W':
                wp.x -= count;
                break;
            case 'F':
                ship.x += wp.x * count;
                ship.y += wp.y * count;
                break;
            case 'L':
                count /= 90;
                rotateL(wp, count);

                break;
            case 'R':
                count /= 90;
                rotateR(wp, count);

                break;
            default:
                break;
        }
        printf("%s %s\n", ship.toString().c_str(), wp.toString().c_str());
    }

    printf("FINISH %d + %d = %d\n", ship.x, ship.y, std::abs(ship.x) + std::abs(ship.y));
}


