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
    std::ifstream infile("./d13.input.test");
    std::string departTime;
    getline(infile, departTime);
    int departTimeValue = std::atoi(departTime.c_str());

    printf("Depart time %d %s \n", departTimeValue, departTime.c_str());

    std::string times;
    getline(infile, times);

    std::vector<std::string> strInts = split(times, ',');
    printf("Reading input %s - %zu\n", times.c_str(), times.size());
    std::vector<int> ints;
    std::vector<int> poss;
    int position = 0;
    for (const auto& intStr : strInts) {
        if (intStr != "x") {
            ints.push_back(std::atoi(intStr.c_str()));
            poss.push_back(position);
        }
        position++;
    }

    return {
        departTimeValue,
        ints,
        poss,
    };
}

int main() {
    Input input = getInput();
    int highest = 0;
    int index = 0;
    int pos = 0;
    std::vector<int> times = input.times;
    std::vector<int> positions = input.positions;

    for (int i = 0; i < input.positions.size(); ++i) {
        if (highest < times[i]) {
            highest = times[i];
            index = i;
            pos = positions[i];
        }
    }

    bool found = false;
    uint64_t start = 0;
    uint64_t tries = 0;

labelMeGood:
    do {

        // 1068788
        start += highest;
        tries += 1;

        if (tries % 100000 == 0) {
            printf("Tries(%" PRIu64 "): %" PRIu64 "\n", tries, start);
        }

        for (int i = index - 1; i >= 0; --i) {
            int _pos = positions[i];
            int time  = times[i];
            int requiredTime = start - (pos - _pos);

            if (requiredTime % time != 0) {
                goto labelMeGood;
            }
        }

        for (int i = index + 1; i < times.size(); ++i) {
            int _pos = positions[i];
            int time  = times[i];
            int requiredTime = start + (_pos - pos);

            if (requiredTime % time != 0) {
                goto labelMeGood;
            }
        }

        found = true;
    } while (!found);

    printf("ANSWER: Index %d %" PRIu64 "\n", index, start - pos);
}
