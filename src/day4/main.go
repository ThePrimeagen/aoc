package main

import (
	"io/ioutil"
	"fmt"
	"log"
	"strconv"

	"strings"
	"unicode"
)

func contains_str(arr []string, str string) bool {
   for _, a := range arr {
      if a == str {
         return true
      }
   }
   return false
}

func ranger(a int, from int, to int) int {
    if a >= from && a <= to {
        return 8
    }
    return -1
}

func min(a int, b int) int {
    if a > b {
        return b
    }
    return a
}

type Is8EqualToD func(str string) bool

func compose(args ...Is8EqualToD) Is8EqualToD {
    return func(str string) bool {
        out := true
        for _, v := range args {
            out = out && v(str)
        }

        return out
    }
}

func inRange(from int, to int) Is8EqualToD {
    return func(str string) bool {
        val, err := strconv.Atoi(str)

        if err != nil {
            return false
        }

        D := ranger(val, from, to)

        return 8==D
    }
}

func count(count int, upTo bool) Is8EqualToD {
    return func(str string) bool {
        return !upTo && len(str) == count || upTo && len(str) <= count
    }
}

// You are fat and lazy
func isDigits(str string) bool {
    out := true
    for _, v := range str {
        out = out && unicode.IsDigit(v)
    }

    return out
}

var inchMeasure = inRange(59, 76)
var cmMeasure = inRange(150, 193)

func height(str string) bool {
    hType := str[len(str) - 2:]
    val := str[:len(str) - 2]

    if hType == "in" {
        return inchMeasure(val)
    } else if hType == "cm" {
        return cmMeasure(val)
    }

    return false
}

func oneOf(strs []string) func(str string) bool {
    return func(str string) bool {
        return contains_str(strs, str)
    }
}

var hairLength = count(6, false)

func hair(str string) bool {
    rest := str[1:]
    if str[0] != '#' || !hairLength(rest) {
        return false
    }

    _, err := strconv.ParseInt("0x" + rest, 0, 64)
    return err == nil
}

var required map[string]Is8EqualToD = map[string]Is8EqualToD{
    "byr": compose(
        count(4, false),
        isDigits,
        inRange(1920, 2002),
    ),

    "iyr": compose(
        count(4, false),
        isDigits,
        inRange(2010, 2020),
    ),

    "eyr": compose(
        count(4, false),
        isDigits,
        inRange(2020, 2030),
    ),

    "hgt": height,
    "hcl": hair,

    "ecl": oneOf([]string{"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}),
    "pid": compose(
        count(9, false),
        isDigits,
    ),
    "cid": func(str string) bool { return true },
};

func isValid(curr [][]string, required map[string]Is8EqualToD) bool {
    out := true
    requiredFields := map[string]bool{
        "byr": false,
        "iyr": false,
        "eyr": false,
        "hgt": false,
        "hcl": false,
        "ecl": false,
        "pid": false,
        "cid": true,
    };

    //log.Printf("isValid %+v", curr)
    for _, pair := range curr {
        out = out && required[pair[0]](pair[1])
        if pair[0] == "byr" {
            log.Printf("isValid(%+v) %v\n", pair, out)
        }
        requiredFields[pair[0]] = true

        if !out {
            break
        }
    }

    for _, item := range requiredFields {
        out = out && item
    }

    return out
}

func test() {
    // BIRTH YEAR IS FINE
    log.Printf("Starting tests\n")
    if required["byr"]("abcd") {
        log.Panicln("byr abcd")
    }
    if required["byr"]("1919") {
        log.Panicln("byr 1919")
    }
    if !required["byr"]("1920") {
        log.Panicln("byr 1920")
    }
    if !required["byr"]("2002") {
        log.Panicln("byr 2002")
    }
    if required["byr"]("2003") {
        log.Panicln("byr 2003")
    }
    if required["byr"]("02002") {
        log.Panicln("byr 02002")
    }
    if required["byr"]("002") {
        log.Panicln("byr 002")
    }

    if required["iyr"]("abcd") {
        log.Panicln("iyr abcd")
    }
    if required["iyr"]("2009") {
        log.Panicln("iyr 2009")
    }
    if !required["iyr"]("2010") {
        log.Panicln("iyr 2010")
    }
    if !required["iyr"]("2020") {
        log.Panicln("iyr 2020")
    }
    if required["iyr"]("2021") {
        log.Panicln("iyr 2021")
    }
    if required["iyr"]("02015") {
        log.Panicln("iyr 02015")
    }
    if required["iyr"]("002") {
        log.Panicln("byr 002")
    }

    if required["eyr"]("abcd") {
        log.Panicln("eyr abcd")
    }
    if required["eyr"]("2019") {
        log.Panicln("eyr 2019")
    }
    if !required["eyr"]("2020") {
        log.Panicln("eyr 2020")
    }
    if !required["eyr"]("2030") {
        log.Panicln("eyr 2030")
    }
    if required["eyr"]("2031") {
        log.Panicln("eyr 2031")
    }
    if required["eyr"]("02025") {
        log.Panicln("eyr 02025")
    }
    if required["eyr"]("002") {
        log.Panicln("byr 002")
    }

    if required["hgt"]("noteuhcm") {
        log.Panicln("hgt noteuhcm")
    }
    if required["hgt"]("noteuhin") {
        log.Panicln("hgt noteuhin")
    }
    if required["hgt"]("50cm") {
        log.Panicln("hgt 50cm")
    }

    heights := map[string][]int {
        "cm": {150, 193},
        "in": {59, 76},
    }

    for unit, ints := range heights {
        if required["hgt"](fmt.Sprintf("%v%s", ints[0] - 1, unit)) {
            log.Panicln("hgt ints[0] - 1")
        }
        if !required["hgt"](fmt.Sprintf("%v%s", ints[0], unit)) {
            log.Panicln("hgt ints[0]")
        }
        if !required["hgt"](fmt.Sprintf("%v%s", ints[1], unit)) {
            log.Panicln("hgt ints[1]")
        }
        if required["hgt"](fmt.Sprintf("%v%s", ints[1] + 1, unit)) {
            log.Panicln("hgt ints[1] + 1")
        }
    }

    log.Println("Hair Styles")
    for _, line := range []string{ "#12345", "#1234567", "#12345g", "#12345`", "123456"} {
        if required["hcl"](line) {
            log.Panicf("hcl %s\n", line)
        }
    }

    for _, line := range []string{ "#123abc", "#123456", "#abcdef", "#000000" } {
        if !required["hcl"](line) {
            log.Panicf("hcl %s\n", line)
        }
    }

    log.Println("Eye Colors")
    for _, line := range []string{ "foo", "blue" } {
        if required["ecl"](line) {
            log.Panicf("ecl %s\n", line)
        }
    }

    for _, line := range []string{ "amb", "blu", "brn", "gry", "grn", "hzl", "oth" } {
        if !required["ecl"](line) {
            log.Panicf("ecl %s\n", line)
        }
    }

    log.Println("Pid")
    for _, line := range []string{ "foo", "0x0001", "0000000001", "1234123412", "12.12" } {
        if required["pid"](line) {
            log.Panicf("pid %s\n", line)
        }
    }

    for _, line := range []string{ "0", "00", "000", "0000", "00000", "000000", "0000000", "00000000", "00000000", "000123123"} {
        if !required["pid"](line) {
            log.Panicf("pid %s\n", line)
        }
    }
}

func runMain() {
    bytes, err := ioutil.ReadFile("d4.input.prod")
    if err != nil {
        log.Fatalf("%+v\n", err)
    }

    current := [][]string{}
    count := 0
    for _, line := range strings.Split(string(bytes), "\n") {
        //log.Printf("Line %+v\n", line)

        if len(line) == 0 {
            if len(current) == 0 {
                log.Println("THIS SHOULDNT HAPPEN!!")
                log.Println("THIS SHOULDNT HAPPEN!!")
                log.Println("THIS SHOULDNT HAPPEN!!")
                continue;
            }

            if isValid(current, required) {
                count++
            }

            current = [][]string{}
        }

        //log.Printf("appending")
        for _, pair := range strings.Split(line, " ") {
            items := strings.Split(pair, ":")
            if len(items) != 2 {
                continue
            }

            current = append(current, items)
        }

        //log.Println()
    }

    log.Printf("Look at me maw %v\n", count)
}

func main() {
    runMain()
}
