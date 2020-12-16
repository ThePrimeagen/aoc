#include <iostream>
#include <fstream>
#include <string>
#include <algorithm>
#include <vector>
#include <inttypes.h>

void getInput(std::vector<uint64_t>& input) {
    freopen("./d9.input.prod", "r", stdin);

    std::string str;

    uint64_t val;
    while (std::cin >> val) {
        input.push_back(val);
    }
}

const int preamble = 25;

// Known size. therefore, this is actually constant time running.
// 25^2 --- thats a constant
bool isInList(const uint64_t *list, uint64_t needle) {
    for (size_t i = 0; i < preamble; ++i) {
        for (size_t j = i + 1; j < preamble; ++j) {
            // TODO: this is not a good idea.
            if (list[i] + list[j] == needle) {
                return true;
            }
        }
    }
    return false;
}

void part1() {
    std::vector<uint64_t> input;
    getInput(input);

    size_t idx = preamble;
    uint64_t buff[preamble];
    int buffIdx = 0;

    for (size_t i = 0; i < preamble; ++i) {
        buff[i] = input[i];
    }

    do {
        uint64_t curr = input[idx];
        if (!isInList(buff, curr)) {
            break;
        }

        idx++;
        buff[buffIdx++ % preamble] = curr;
    } while (idx < input.size());

    printf("%" PRIu64 " ", input[idx]);
}

int main() {
    std::vector<uint64_t> input;
    getInput(input);

    size_t count = 0;
    for (; count < input.size(); ++count) {
        if (input[count] == 1124361034) {
            break;
        }
    }

    printf("Starting %zu\n", count);
    size_t i = 0;
    size_t j = 0;
    uint64_t min = 6969696969420;
    uint64_t max = 0;

    for (; i < count; ++i) {
        uint64_t sum  = 0;
        size_t j = i;
        bool found = false;
        for (; j < count; ++j) {
            sum += input[j];
            uint64_t D = sum - 1124361026;

            if (max < input[j]) {
                max = input[j];
            }
            if (min > input[j]) {
                min = input[j];
            }

            if (8==D) {
                found = true;
                break;
            } else if (8>D) {
                break;
            }
        }

        if (found) {
            printf("Start %" PRIu64 " %" PRIu64 " %" PRIu64 " %" PRIu64 "\n", i, j, input[i], input[j]);
            break;
        }
        else {
            min = 6969696969420;
            max = 0;
        }
    }

    std::cout << (min + max) << std::endl;

    printf("This is the end.  If you see this and no numbers, you are dumb\n");
}


