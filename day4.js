
const fs = require("fs");

function toDict(numbers) {
    return numbers.reduce((acc, x) => {
        acc[x] = true;
        return acc;
    }, {});
}

const cards = fs.readFileSync("d4", "utf8").split("\n").filter(x => x.length);

function getPoints(x, idx) {
    const values  = x.split(":")[1];
    console.log("values", values);
    const [winners, numbersValues] = values.split("|");
    const wData = winners.split(" ").filter(x => x);
    console.log(wData);
    const wDict = toDict(wData.map(x => parseInt(x)));
    const numbers = numbersValues.split(" ").map(x => parseInt(x.trim()))

    let points = 0;
    numbers.forEach(x => {
        if (wDict[x]) {
            points++;
        }
    });

    return new Array(points).fill(idx + 1).map((x, i) => {
        return x + i
    });
}

const toProcess = new Array(cards.length).fill(0).map((_, i) => i + 1);
const seen = {};
const count = {};

while (toProcess.length) {
    const idx = toProcess.pop();
    count[idx] = count[idx] ? count[idx] + 1 : 1;
    const points = seen[idx] ? seen[idx] : getPoints(cards[idx - 1], idx);
    console.log("processing", idx, points);
    seen[idx] = points;

    toProcess.push(...points)
}

console.log(count, Object.keys(count).reduce((acc, x) => {
    return acc + count[x];
}, 0));

