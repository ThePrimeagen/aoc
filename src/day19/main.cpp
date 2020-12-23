#include <vector>
#include <algorithm>
#include <iostream>
#include <functional>
#include <fstream>
#include <string>
#include <sstream>
#include <cmath>
#include <unordered_map>
#include <unordered_set>
#include <inttypes.h>

const char* testFile = "./d19.input.test";

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

struct Input {
    std::vector<std::string> rules;
    std::vector<std::string> lines;
};

Input getInput() {
    std::string input;
    std::ifstream infile(testFile);

    // Read the rules
    std::vector<std::string> rules;
    std::vector<std::string> lines;

    while (getline(infile, input)) {
        if (input.empty()) {
            break;
        }
        rules.push_back(input);
    }

    // Read the input
    while (getline(infile, input)) {
        if (input.empty()) {
            break;
        }
        lines.push_back(input);
    }

    return {
        rules,
        lines
    };
}

class Rule {
    public:
        bool match(std::string&, int offset);
        void reset();
};

using RuleSet = std::vector<Rule>;
using RuleSetIndices = std::vector<int>;
using ListRuleSetIndices = std::vector<RuleSetIndices>;
class SequenceRule : public Rule {
    SequenceRule(RuleSet& set, ListRuleSetIndices& ruleSets) {
    }

    bool match(std::string& str, int offset) {
        return false;
    }

    void reset() {
    }
};

class CharacterRule : public Rule {
    CharacterRule(char toMatch) {
    }

    bool match(std::string& str, int offset) {
        return str[offset] == toMatch;
    }

    void reset() { }

    private:

}
