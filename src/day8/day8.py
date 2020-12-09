import os
from copy import deepcopy


def test(lines):
    seen = [False for i in lines]
    idx = 0
    acc = 0

    while idx < len(seen) and not seen[idx]:
        seen[idx] = True
        if lines[idx][0] == "acc":
            acc += lines[idx][1]
            idx += 1
        elif lines[idx][0] == "nop":
            idx += 1
        elif lines[idx][0] == "jmp":
            idx += lines[idx][1]

    print("Ran", idx, acc, len(seen))

    return idx == len(seen), acc

def change_instruction(lines, start_idx, _from, to):
    changed = False
    i = start_idx
    print("change_instruction")

    while i < len(lines):
        print("change_instruction#while", i, lines[i][0], _from)
        if lines[i][0] == _from:
            lines[i][0] = to
            changed = True
            break

        i += 1

    print("change_instruction", len(lines), i, start_idx, _from, to, changed)
    return i, changed

def run_through(lines, _from, to):
    idx = 0
    acc = 0
    found_exit = False

    while True:
        new_lines = deepcopy(lines)
        next_idx, changed = change_instruction(new_lines, idx, _from, to)

        if not changed:
            break

        found, acc = test(new_lines)
        if found:
            found_exit = True
            break

        idx = next_idx + 1

    return found_exit, idx, acc

def run_the_test(lines):
    found, idx, acc = run_through(lines, "nop", "jmp")
    if found:
        print("FOUND nop", idx, acc)
        return

    found, idx, acc = run_through(lines, "jmp", "nop")
    if found:
        print("FOUND jmp", idx, acc)
        return

    print("YOU ARE DUMB")

def parse(line):
    parts = line.split(" ")
    return [parts[0], int(parts[1])]

f = open(os.path.dirname(__file__) + "/input.prod", "r")
lines = [parse(i) for i in f.read().splitlines()]
seen = [False for i in lines]

run_the_test(lines)

