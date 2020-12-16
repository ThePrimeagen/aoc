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
    std::ifstream infile("./d13.input.prod");
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
    int i = 0;
    for (int i = 0; i < input.times.size(); ++i) {
        printf("x = %d mod %d\n", 1 + input.positions[i], input.times[i]);
    }
}

