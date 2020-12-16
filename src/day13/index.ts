import * as fs from "fs";
import * as path from "path";

const contents = fs.readFileSync(path.join(__dirname, "input.prod")).toString().split("\n");

const ttD = +contents[0];
const allDun = contents[1].
    split(",").
    map(x => +x).
    filter(x => !isNaN(x)).
    reduce((acc, x) => {
        const out = x - ttD % x;
        console.log("ttD", ttD, "x", x, "out", out);
        if (out < acc[1]) {
            acc[0] = x;
            acc[1] = out;
        }

        return acc;
    }, [0, 100000000]);


console.log("ALL DUN", allDun);


