#include <vector>
#include <algorithm>
#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cmath>
#include <inttypes.h>

struct Input {
    int timeDepart;
    int highest;
    int pos;
    int index;
    std::vector<int> times;
    std::vector<int> positions;
};

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

Input getInput() {
    std::vector<std::string> input;
    std::ifstream infile("./d13.input.prod");
    std::string departTime;
    getline(infile, departTime);
    int departTimeValue = std::atoi(departTime.c_str());

    printf("Depart time %d %s \n", departTimeValue, departTime.c_str());

    std::string timesStri;
    getline(infile, timesStri);

    std::vector<std::string> strInts = split(timesStri, ',');
    printf("Reading input %s - %zu\n", timesStri.c_str(), timesStri.size());
    std::vector<int> times;
    std::vector<int> positions;
    int position = 0;
    for (const auto& intStr : strInts) {
        if (intStr != "x") {
          times.push_back(std::atoi(intStr.c_str()));
          positions.push_back(position);
        }
        position++;
    }

    int highest = 0;
    int index = 0;
    int pos = 0;
    for (int i = 0; i < positions.size(); ++i) {
        if (highest < times[i]) {
            highest = times[i];
            index = i;
            pos = positions[i];
        }
    }

    return {
        departTimeValue,
        highest,
        pos,
        index,
        times,
        positions,
    };
}

struct Solve {
    uint64_t start;
    uint64_t cycle;
};

uint64_t getNextOccurance(uint64_t start, uint64_t cycle, uint64_t number, uint64_t offset) {
    uint64_t tries = 0;
    do {

        //if (++tries % 100000000 == 0) {
            printf("Tries(%" PRIu64 "): %" PRIu64 " - %" PRIu64 " %" PRIu64 "\n", tries, start, cycle, (start + offset) % number);
       // }

        // Found condition
        if ((start + offset) % number == 0) {
            return start;
        }

        start += cycle;
    } while (true);
}

Solve solve(uint64_t start, uint64_t cycle, uint64_t number, uint64_t offset) {
    uint64_t nextStart = getNextOccurance(start, cycle, number, offset);
    uint64_t nextCycle = cycle * number;

    return {
        nextStart,
        nextCycle,
    };
}

int main() {
    Input input = getInput();
    int lastPosition = 0;

    uint64_t startTime = input.times[0];
    Solve curr = {
        0,
        startTime,
    };

    for (int i = 1; i < input.positions.size(); ++i) {
        printf("Input %d %d\n", input.times[i], input.positions[i]);
        printf("Current %" PRIu64 " - %" PRIu64 "\n", curr.start, curr.cycle);
        curr = solve(curr.start, curr.cycle, input.times[i], input.positions[i]);
    }

    printf("Final %" PRIu64 " - %" PRIu64 "\n", curr.start, curr.cycle);
}
