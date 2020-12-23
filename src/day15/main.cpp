#include <vector>
#include <algorithm>
#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cmath>
#include <unordered_map>
#include <inttypes.h>

using Simp = std::unordered_map<uint64_t, std::pair<int64_t, int64_t>>;
#define LIMIT 30000000
#define INITIAL_SIZE 7

uint64_t test[LIMIT] = { 0,3,6 };
uint64_t prod[LIMIT] = {9,19,1,6,0,5,4};
uint64_t* input = prod;

void insert(Simp& seen, uint64_t index, uint64_t value) {
    auto it = seen.find(value);
    if (it == seen.end()) {
        seen[value] = {index, -1};
    }
    else if (it->second.second == -1) {
        it->second.second = index;
    }
    else {
        it->second.first = it->second.second;
        it->second.second = index;
    }
}

int main() {
    Simp seen;
    seen.reserve(1000000);
    for (int i = 0; i < INITIAL_SIZE; ++i) {
        insert(seen, i, input[i]);
    }

    uint64_t tries = 0;
    for (int i = INITIAL_SIZE; i < LIMIT; ++i) {
        int lastIdx = i - 1;
        uint64_t lastItem = input[lastIdx];

        auto it = seen.find(lastItem);

        if (it == seen.end() || it->second.second == -1) {
            insert(seen, i, 0);
            input[i] = 0;
        }
        else {
            uint64_t range = it->second.second - it->second.first;
            insert(seen, i, range);
            input[i] = range;
        }
    }

    printf("Finish %" PRIi64 " => %" PRIi64 " \n", LIMIT, input[LIMIT - 1]);
}

