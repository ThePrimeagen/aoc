const fs = require("fs");

const contents = fs.readFileSync("d3", "utf8").split("\n").map(x => x.split("")).filter(x => x.length > 0);

/** @param {string} char **/
function isCharNumber(char) {
    return !isNaN(parseInt(char));
}

function isDot(char) {
    return char === ".";
}

const dirs = [
    [-1, -1], [0, -1], [1, -1],
    [-1, 0], [1, 0],
    [-1, 1], [0, 1], [1, 1]
];

function get(i, j, [y, x]) {
    const chars = contents[i + y]
    if (chars === undefined) {
        return undefined;
    }
    return chars[j + x];
}

let sum = 0;
for (let y = 0; y < contents.length; ++y) {
    const row = contents[y];
    let isNumber = false;
    let currentNumber = "";
    let check = true

    for (let x = 0; x < row.length; ++x) {
        isNumber = isCharNumber(get(y, x, [0, 0]));

        if (!isNumber && !check) {
            console.log("currentNumber", currentNumber);
            sum += parseInt(currentNumber);
        }

        if (!isNumber) {
            currentNumber = "";
            check = true;
        }

        if (isNumber && check) {
            const is = dirs.reduce((acc, [dy, dx]) => {
                const char = get(y, x, [dy, dx]);
                return acc || !isDot(char) && !isCharNumber(char) && char !== undefined;
            }, false);

            if (is) {
                check = false;
            }
        }

        if (isNumber) {
            currentNumber += get(y, x, [0, 0]);
        }

    }

    if (isNumber && !check) {
        console.log("currentNumber", currentNumber);
        sum += parseInt(currentNumber);
    }
}

console.log(sum);
