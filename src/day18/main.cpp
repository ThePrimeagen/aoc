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

const char* testFile = "./d18.input.prod";

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

std::string removeWhitespace(std::string str) {
    std::vector<std::string> noWhite = split(str, ' ');
    std::stringstream ss;

    for (const auto& it : noWhite) {
        ss << it;
    }

    return ss.str();
}

struct QuickMafs {
    std::string line;
    int index = 0;

    QuickMafs(std::string l) : line(l) {}

    bool done() {
        return index == line.size();
    }

    char peek() {
        return line[index];
    }

    void consumeOne() {
        index++;
    }

    bool hasOpNext() {
        return peek() == '+' || peek() == '*';
    }

    uint64_t consumeInt() {
        std::stringstream ss;
        do {
            ss << peek();
            consumeOne();
        } while (peek() >= '0' && peek() <= '9');

        return std::atoll(ss.str().c_str());
    }

    char consumeOperator() {
        return line[index++];
    }
};

using EvaluateFn = std::function<uint64_t(QuickMafs& mafs)>;

uint64_t evaluateNextExpression(QuickMafs& mafs);
uint64_t evaluateExpressionRange(QuickMafs& mafs, int to);
uint64_t getValue(QuickMafs& mafs, EvaluateFn recurse) {
    uint64_t out;

    if (mafs.peek() == '(') {
        mafs.consumeOne();
        out = recurse(mafs);
    }
    else {
        out = mafs.consumeInt();
    }

    return out;
}

uint64_t evaluateNextExpression(QuickMafs& mafs) {
    uint64_t left = getValue(mafs, evaluateNextExpression);
    do {
        char op = mafs.consumeOperator();
        uint64_t right = getValue(mafs, evaluateNextExpression);

        left = op == '*' ? left * right : left + right;
    } while (mafs.hasOpNext());

    if (mafs.peek() == ')') {
        mafs.consumeOne();
    }

    return left;
}

uint64_t evaluatePlusThenMul(QuickMafs& mafs) {
    std::stringstream mulLine;

    uint64_t lhs = getValue(mafs, evaluatePlusThenMul);
    do {
        char op = mafs.consumeOperator();
        uint64_t rhs = getValue(mafs, evaluatePlusThenMul);

        if (op == '*') {
            mulLine << lhs << op;
            lhs = rhs;
        } else {
            lhs = lhs + rhs;
        }
    } while (!mafs.done() && mafs.peek() != ')');

    if (mafs.peek() == ')') {
        mafs.consumeOne();
    }

    std::string remainder = mulLine.str();
    if (remainder.size() > 0) {
        mulLine << lhs;
        remainder = mulLine.str();
        QuickMafs quicky(remainder);

        return evaluateNextExpression(quicky);
    }

    return lhs;
}

int day18part1() {
    std::vector<std::string> input;
    std::ifstream infile(testFile);

    std::string l;
    uint64_t sum = 0;
    while (std::getline(infile, l)) {
        std::string line = removeWhitespace(l);
        QuickMafs mafs(line);

        uint64_t out = evaluateNextExpression(mafs);

        printf("Out %s %" PRIu64 "\n", line.c_str(), out);
        sum += out;
    }

    printf("The End %" PRIu64 "\n", sum);
    return 0;
}

int main() {
    std::vector<std::string> input;
    std::ifstream infile(testFile);

    std::string l;
    uint64_t sum = 0;
    while (std::getline(infile, l)) {
        std::string line = removeWhitespace(l);
        QuickMafs mafs(line);
        uint64_t out = evaluatePlusThenMul(mafs);
        printf("Out %s %" PRIu64 "\n", line.c_str(), out);
        sum += out;
    }

    printf("I AM OUT %" PRIu64 "\n", sum);
}

