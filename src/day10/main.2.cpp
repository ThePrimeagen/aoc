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
    input.push_back(0);
    size_t len = input.size();
    uint64_t counts[len];
    counts[0] = 1;

    std::sort(input.begin(), input.end());

    for (int i = 1; i < len; ++i) {
        int backIdx = i;
        int currentValue = input[i];
        counts[i] = 0;

        while (--backIdx >= 0 && currentValue - input[backIdx] <= 3) {
            counts[i] += counts[backIdx];
        }
    }

    printf("ITEM %" PRIu64"\n", counts[len - 1]);

    return 0;
}

