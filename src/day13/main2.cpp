#include <vector>
#include <algorithm>
#include <iostream>
#include <fstream>
#include <string>
#include <thread>
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

void search(Input input, uint64_t start, uint64_t count, int id) {
    int highest = input.highest;
    int index = input.index;
    int pos = input.pos;
    uint64_t tries = 0;

    std::vector<int> positions = input.positions;
    std::vector<int> times = input.times;

    bool found = false;

labelMeGood:
    do {
        // 1068788
        start += highest;
        tries += 1;

        if (tries % ((uint64_t)1e9 / 3) == 0) {
            printf("Tries(%d): %" PRIu64 "\n", id, start);
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
    } while (!found && tries < count);

    if (found) {
        printf("SUCK IT RICHARD %" PRIu64 "\n", start);
        printf("SUCK IT RICHARD %" PRIu64 "\n", start);
        printf("SUCK IT RICHARD %" PRIu64 "\n", start);
        printf("SUCK IT RICHARD %" PRIu64 "\n", start);
        printf("SUCK IT RICHARD %" PRIu64 "\n", start);
    }
    else {
        printf("COULDN't FIND %d\n", id);
        printf("COULDN't FIND %d\n", id);
        printf("COULDN't FIND %d\n", id);
        printf("COULDN't FIND %d\n", id);
    }
}

int main() {
    Input input = getInput();
    std::vector<int> times = input.times;
    std::vector<int> positions = input.positions;

    bool found = false;
    uint64_t start = 0;
    uint64_t tries = 100e9;

    int threadCount = 16;
    std::thread threads[threadCount];
    for (int i = 0; i < threadCount; ++i) {
        start = i * tries * input.highest;
        threads[i] = std::thread(search, input, start, tries, i);
    }
    for (int i = 0; i < threadCount; ++i) {
        threads[i].join();
    }
}


