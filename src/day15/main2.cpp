#include <vector>
#include <algorithm>
#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cmath>
#include <map>
#include <inttypes.h>

using Simp = std::map<uint64_t, std::vector<uint64_t>>;

std::vector<uint64_t> input = { 0,3,6 };

void insert(Simp& seen, uint64_t index, uint64_t value) {
    auto it = seen.find(value);
    if (it == seen.end()) {
        seen[value] = {index};
    }
    else if (it->second.size() == 1) {
        it->second.push_back(index);
        it->second[1] = index + 1;
        it->second[1] = index;
    }
    else {
        it->second[0] = it->second[1];
        it->second[1] = index;
    }
}

void printSimp(Simp& seen) {
    for (auto it : seen) {
        printf("%" PRIu64 " => ", it.first);
        for (auto itt : it.second) {
            printf("%" PRIu64 ",", itt);
        }
        printf("\n");
    }
}

int main() {
    Simp seen;
    for (int i = 0; i < input.size(); ++i) {
        insert(seen, i, input[i]);
    }

    input[3] = 5;
    printSimp(seen);
    input[4] = 5;
    printSimp(seen);
    input[5] = 5;
    printSimp(seen);
    input[6] = 5;
    printSimp(seen);
    input[7] = 5;
    printSimp(seen);

    printf("2020 %" PRIu64 " \n", input[2019]);
}


