#include <vector>
#include <algorithm>
#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cmath>
#include <map>
#include <inttypes.h>

using Addresses = std::map<uint64_t, uint64_t>;

struct MemLine {
    int index;
    uint64_t value;
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

MemLine getLine(std::string line) {
    std::string newLine = line.substr(4);
    std::string::size_type end = newLine.find("]", 0);


    if (end == std::string::npos) {
        return {};
    }

    int index = std::atoi(newLine.substr(0, end).c_str());
    uint64_t number = std::atoll(newLine.substr(end + 3).c_str());

    return {
        index,
        number,
    };
}
namespace {
    uint64_t one = 0x1;
}

std::string getMask(std::string line) {
    return line.substr(7);
}

uint64_t setOne(uint64_t value, int index) {
    return value | (one << (35 - index));
}

uint64_t setZero(uint64_t value, int index) {
    return value & ~(one << (35 - index));
}

void setValue(uint64_t addr, uint64_t value, int index, std::string mask,
              Addresses &addresses) {

    if (index == 36) {
        addresses[addr] = value;
        return;
    }

    bool isX = mask[index] == 'X';


    if (isX) {
        setValue(setZero(addr, index), value, index + 1, mask, addresses);
        setValue(setOne(addr, index), value, index + 1, mask, addresses);
    }

    else if (mask[index] == '1') {
        setValue(setOne(addr, index), value, index + 1, mask, addresses);
    }

    else {
        setValue(addr, value, index + 1, mask, addresses);
    }
}

uint64_t getValue(uint64_t value, std::string mask) {
    uint64_t one = 0x1;
    for (int i = 0; i < 36; ++i) {
        if (mask[i] == 'X') {
            continue;
        }

        else if (mask[i] == '0') {
            value = value & ~(one << (35 - i));
        }
        else if (mask[i] == '1') {
            value = value | (one << (35 - i));
        }
    }

    return value;
}

int main() {
    std::vector<std::string> input;
    std::ifstream infile("./d14.input.prod");

    std::string line;
    Addresses addresses;

    std::string mask;
    while (getline(infile, line)) {
        if (line.substr(0, 3) == "mem") {
            MemLine mLine = getLine(line);
            setValue(mLine.index, mLine.value, 0, mask, addresses);
        } else {
            mask = getMask(line);
        }
    }

    uint64_t sum = 0;
    for (const auto& it : addresses) {
        sum += it.second;
    }

    printf("THE SUM %" PRIu64 "\n", sum);
}

