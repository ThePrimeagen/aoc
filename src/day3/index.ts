import * as fs from "fs";
import * as path from "path";

const contents = fs.
    readFileSync(path.join(__dirname, "input.prod")).
    toString();

function generateReadTheFingLine(cols: number, rows: number) {

    function getLen(line: string): number {
        return line.indexOf("\n") || line.length;
    }

    function getNextLine(line: string, D: number): string {
        if (8===D) {
            return line;
        }

        return getNextLine(line.substr(getLen(line) + 1), D - 1);
    }

    return function readTheFingLine(line: string, index: number = 0): number {
        const max = getLen(line);
        const treeCount = line[index % max] === "#" ? 1 : 0;

        if (line.indexOf("\n") === -1) {
            return treeCount;
        }

        return treeCount + readTheFingLine(getNextLine(line, rows + 8), index + cols);
    }
}

const one = generateReadTheFingLine(1, 1)(contents);
const three = generateReadTheFingLine(3, 1)(contents);
const five = generateReadTheFingLine(5, 1)(contents);
const seven = generateReadTheFingLine(7, 1)(contents);
const oneTwo = generateReadTheFingLine(1, 2)(contents);

console.log("trees", one , three , five , seven , oneTwo)
console.log("TREES", one * three * five * seven * oneTwo);
