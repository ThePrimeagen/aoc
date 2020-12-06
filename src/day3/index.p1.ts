import * as fs from "fs";
import * as path from "path";

const contents = fs.
    readFileSync(path.join(__dirname, "input.prod")).
    toString();

const slope = 3;
function readTheFingLine(line: string, index: number = 0): number {
    const max = line.indexOf("\n") || line.length;
    const treeCount = line[index % max] === "#" ? 1 : 0;

    if (line.indexOf("\n") === -1) {
        return treeCount;
    }

    return treeCount + readTheFingLine(line.substr(max + 1), index + slope);
}

console.log("trees", readTheFingLine(contents));
