#include <vector>
#include <algorithm>
#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cmath>
#include <map>
#include <set>
#include <inttypes.h>

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

using Rule = std::vector<std::pair<int, int>>;
using Rules = std::vector<Rule>;
using Ticket = std::vector<int>;
using Tickets = std::vector<Ticket>;

struct Input {
    Rules rules;
    Ticket myTicket;
    Tickets othersTicket;
};

Rule parseRule(const std::string& ruleStr) {
    Rule rule;
    std::string rulesPart = split(ruleStr, ':')[1];
    std::vector<std::string> rules = split(rulesPart, ' ');

    for (auto it : rules) {
        if (it == "or" || it.empty()) {
            continue;
        }

        printf("About to split %s\n", it.c_str());
        std::vector<std::string> nums = split(it, '-');

        rule.push_back({
            std::atoi(nums[0].c_str()),
            std::atoi(nums[1].c_str())
        });
    }

    return rule;
}

Ticket parseTicket(const std::string& ticketStr) {
    Ticket ticket;
    std::vector<std::string> nums = split(ticketStr, ',');
    for (auto it : nums) {
        ticket.push_back(std::atoi(it.c_str()));
    }

    return ticket;
}

Input getInput() {
    std::vector<std::string> input;
    std::ifstream infile("./d16.input.prod");

    // Read until empty line, these are rules
    std::string ruleStr;
    Rules rules;
    while (getline(infile, ruleStr) && ruleStr.size() > 0) {
        rules.push_back(parseRule(ruleStr));
    }

    // Skip line (your ticket:)
    // parse my times
    std::string throwAway;
    std::string ticketStr;
    getline(infile, throwAway);
    getline(infile, ticketStr);
    Ticket myTicket = parseTicket(ticketStr);

    // kappa emote is offensive to people who are kappa
    //
    getline(infile, throwAway);
    getline(infile, throwAway);
    Tickets others;
    while (getline(infile, ticketStr)) {
        others.push_back(parseTicket(ticketStr));
    }

    return {
        rules,
        myTicket,
        others,
    };
}

bool inRules(Input& input, int number) {
    bool found = false;

    // Copies?
    Rules rules = input.rules;

    for (int i = 0; !found && i < rules.size(); ++i) {
        for (auto it : rules[i]) {
            if (number >= it.first && number <= it.second) {
                found = true;
                break;
            }
        }
    }

    return found;
}

void printRules(int index, std::set<int>& matched) {
    printf("Rule(%d): ", index);
    for (int x : matched) {
        printf("%d, ", x);
    }

    printf("\n");
}

std::vector<std::set<int>> getMatches(Input& input, Tickets& tickets) {
    std::vector<std::set<int>> out;
    size_t ticketLength = tickets[0].size();

    // for each column
    for (size_t i = 0; i < ticketLength; ++i) {
        std::set<int> matchedColumns;

        // For each ticket
        for (const Ticket& ticket : tickets) {
            std::set<int> rulesMatched;
            int number = ticket[i];

            // For each rule
            for (size_t rIdx = 0; rIdx < input.rules.size(); ++rIdx) {
                for (const auto& it : input.rules[rIdx]) {
                    if (number >= it.first && number <= it.second) {
                        rulesMatched.insert(rIdx);
                        break;
                    }
                }
            }

            if (matchedColumns.empty()) {
                matchedColumns = rulesMatched;
            } else {
                std::set<int> intersection;
                std::set_intersection(
                        matchedColumns.begin(),
                        matchedColumns.end(),
                        rulesMatched.begin(),
                        rulesMatched.end(),
                        std::inserter(intersection, intersection.begin()));

                matchedColumns = intersection;
            }
        }

        printRules(i, matchedColumns);
        out.push_back(matchedColumns);
    }

    return out;
}

int main() {
    Input input = getInput();

    int sum = 0;
    Tickets others = input.othersTicket;

    std::vector<Ticket> filtered;
    for (int ticketIdx = others.size() - 1; ticketIdx >= 0; --ticketIdx) {
        Ticket ticket = others[ticketIdx];

        bool validTicket = true;
        for (int i = 0; validTicket && i < ticket.size(); ++i) {
            validTicket = inRules(input, ticket[i]);
        }

        if (validTicket) {
            filtered.push_back(ticket);
        }
    }

    std::vector<std::set<int>> matchedByColumns = getMatches(input, filtered);
    std::vector<std::pair<int, std::set<int>>> sorted;
    for (size_t i = 0; i < matchedByColumns.size(); ++i) {
        sorted.push_back({
            i,
            matchedByColumns[i],
        });
    }

    std::sort(sorted.begin(), sorted.end(),
        [](const auto& a, const auto& b) {
            return a.second.size() < b.second.size();
        });

    std::vector<int> ruleOrder;
    ruleOrder.reserve(input.rules.size());

    std::set<int> usedRules;

    for (const auto& byColumn : sorted) {
        int foundCount = 0;

        for (int it : byColumn.second) {
            auto found = usedRules.find(it);
            if (found == usedRules.end()) {
                usedRules.insert(it);
                printf("set ruleOrder[%d] = %d\n", byColumn.first, it);
                ruleOrder[byColumn.first] = it;
                foundCount++;
            }
        }

        if (foundCount > 1) {
            printf("WE HAVE A PROBLEM!!!!\n");
            printf("WE HAVE A PROBLEM!!!!\n");
            printf("WE HAVE A PROBLEM!!!!\n");
            printf("WE HAVE A PROBLEM!!!!\n");
            printf("WE HAVE A PROBLEM!!!!\n");
        }
    }

    uint64_t out = 1;
    printf("RULE SIZE %zu \n", ruleOrder.size());
    for (int i = 0; i < 6; ++i) {
        printf("check rules for %d \n", i);
        for (size_t j = 0; j < input.rules.size(); ++j) {
            printf("  rule[%zu] = %d\n", j, ruleOrder[j]);
            if (ruleOrder[j] == i) {
                printf("    rule[%d] = %d, ticketValue = %d, out = %" PRIu64 "\n", j, ruleOrder[j], input.myTicket[ruleOrder[i]], out);
                out *= input.myTicket[j];
            }
        }
    }

    printf(" Out %" PRIu64 "\n", out);

    int a = 5;
}
