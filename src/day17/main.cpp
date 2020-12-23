#include <vector>
#include <algorithm>
#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cmath>
#include <unordered_map>
#include <unordered_set>
#include <inttypes.h>


template<typename Out>
void split(const std::string &s, char delim, Out result) {
    std::stringstream ss(s);
    std::string item;
    while (std::getline(ss, item, delim)) {
        *(result++) = item;
    }
}

std::vector<std::string> split(const std::string &s, char delim) {
    std::vector<std::string> elems;
    split(s, delim, std::back_inserter(elems));
    return elems;
}

#define X 3 + 12
#define Y 3 + 12
#define Z 1 + 12

struct Point {
    int x;
    int y;
    int z;
    int w;

    bool operator==(const Point& other) const {
        return x == other.x &&
            y == other.y &&
            z == other.z &&
            w == other.w;
    }

    Point(int _x, int _y, int _z, int _w) : x(_x), y(_y),  z(_z), w(_w) { }

    std::string toString() const {
        std::stringstream ss;
        ss << "(" << x << ", " << y << ", " << z << ", " << w << ")";
        return ss.str();
    }

    std::vector<Point> getNeighbors() const {
        std::vector<Point> points;
        for (int _x = -1; _x <= 1; ++_x) {
            for (int _y = -1; _y <= 1; ++_y) {
                for (int _z = -1; _z <= 1; ++_z) {
                    for (int _w = -1; _w <= 1; ++_w) {
                        if (!(_x | _y | _z | _w)) {
                            continue;
                        }
                        points.push_back({
                            x + _x,
                            y + _y,
                            z + _z,
                            w + _w,
                        });
                    }
                }
            }
        }

        return points;
    }
};

struct hash_fn
{
    std::size_t operator() (const Point& p) const
    {
        std::size_t h1 = std::hash<int>()(p.x);
        std::size_t h2 = std::hash<int>()(p.y);
        std::size_t h3 = std::hash<int>()(p.z);
        std::size_t h4 = std::hash<int>()(p.w);

        return (h1 + 69) ^ (h2 + 420) ^ (h3 + 1337) ^ (h4 + 42069);
    }
};

// specialized hash function for unordered_map keys

// using Map = std::unordered_map<int, std::unordered_map<int, std::unordered_map<int, bool>>>;
using Set = std::unordered_set<Point, hash_fn>;
using Points = std::vector<Point>;

struct Board {
    Set set;

    Board() = default;
    Board(Set _set) : set(_set) {}

    bool isActive(const Point& p) const {
        auto it = set.find(p);
        return it != set.end();
    }

    void activate(const Point p) {
        set.insert(p);
    }

    void deactivate(const Point& p) {
        auto it = set.find(p);
        if (it != set.end()) {
            set.erase(it);
        }
    }

    void print() const {
        /*
        for (int z = -6; z <= 6; ++z) {
            Points onPlane;
            int minX = 100;
            int maxX = -100;
            int minY = 100;
            int maxY = -100;
            for (const auto& it : set) {
                if (it.z == z) {
                    if (it.x > maxX) {
                        maxX = it.x;
                    }
                    if (it.x < minX) {
                        minX = it.x;
                    }
                    if (it.y > maxY) {
                        maxY = it.y;
                    }
                    if (it.y < minY) {
                        minY = it.y;
                    }

                    onPlane.push_back(it);
                }
            }

            if (onPlane.size() == 0) {
                continue;
            }

            printf("Map z = %d\n", z);
            for (int y = minY; y <= maxY; ++y) {
                for (int x = minX; x <= maxX; ++x) {
                    auto it = set.find({x, y, z});
                    if (it == set.end()) {
                        printf(".");
                    } else {
                        printf("#");
                    }
                }
                printf("\n");
            }
        }
        */
    }

    int neighborCount(const Point& p) const {
        int count = 0;

        for (const auto& p : p.getNeighbors()) {
            auto it = set.find(p);
            count += it == set.end() ? 0 : 1;
        }

        return count;
    }
};

Board getInput() {
    std::vector<std::string> input;
    std::ifstream infile("./d17.input.prod");

    int y = 0;
    int z = 0;
    int w = 0;
    std::string line;
    Board board;
    while (getline(infile, line)) {
        int x = 0;
        for (int x = 0; x < line.size(); ++x) {
            if (line[x] == '#') {
                board.activate({
                    x, y, z, w
                });
            }
        }
        y++;
    }

    return board;
}

int main() {
    Board curr = getInput();
    Board next(curr.set);

    for (int i = 0; i < 6; ++i) {
        printf("Round %d\n", i);
        Set seen;

        for (const Point& p : curr.set) {

            // 1. If p is active (which it is) and 2 or 3 of its neighbors are
            // active it remains active.
            int count = curr.neighborCount(p);
            if (count != 2 && count != 3) {
                next.deactivate(p);
            }

            seen.emplace(p);
            for (const Point& n : p.getNeighbors()) {
                if (seen.find(n) != seen.end() || curr.isActive(n)) {
                    continue;
                }

                seen.emplace(n);
                int count = curr.neighborCount(n);

                // 2. If a cube is inactive but exactly 3 of its neighbors are
                // active, the cube becomes active. Otherwise, the cube remains
                // inactive.
                if (count == 3) {
                    next.activate(n);
                }
            }
        }

        curr.set = next.set;
    }

    printf("YAYAY %zu\n", curr.set.size());
}
