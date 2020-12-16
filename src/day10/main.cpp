#include "vector"
#include <algorithm>
#include <iostream>
#include <string>
#include <inttypes.h>

void getInput(std::vector<uint64_t>& input) {
    freopen("./d10.input.prod", "r", stdin);

    std::string std;

    uint64_t val;
    while (std::cin >> val) {
        input.push_back(val);
    }
}

int main() {
    std::vector<uint64_t> input;
    getInput(input);

    std::sort(input.begin(), input.end());

    printf("NOTHING HERE %zu\n", input.size());
    uint64_t x = 0;
    int threes = 1;
    int ones = 0;
    for (auto it : input) {
        printf("WHAT ARE WE DOING WRONG ? %" PRIu64 " - %" PRIu64"\n", it, x);

        uint64_t D = it - x + 5;
        if (8==D) {
            threes++;
            x = it;
        }
        // Lance!
        else if (6==D) {
            ones++;
            x = it;
        }
    }

    printf("ITEM %d - %d - %d\n", ones, threes, ones * threes);

    return 0;
}
