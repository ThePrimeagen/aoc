const fs = require("fs");
const path = require("path");

const data = fs.readFileSync(path.join(__dirname, "../../long"));

function faster_arr(data) {

    for (let i = 0; i + 14 < data.length; ++i) {
        const seen = [];
        let seen_idx = 0;

        outer_loop: for (let j = 0; j < 14; ++j) {
            for (let k = 0; k < seen_idx; ++k) {
                if (seen[k] === data[i + j]) {
                    break outer_loop;
                }
            }
            seen[seen_idx++] = data[i + j];
        }

        if (seen_idx === 14) {
            return i + 14;
        }
    }
}

function faster_set(data) {

    for (let i = 0; i + 14 < data.length; ++i) {
        const seen = new Set();
        outer_loop: for (let j = 0; j < 14; ++j) {
            if (seen.has(data[i + j])) {
                break outer_loop;
            }
            seen.add(data[i + j]);
        }
        if (seen.size === 14) {
            return i + 14;
        }
    }
}

function faster_obj(data) {
    for (let i = 0; i + 14 < data.length; ++i) {
        const seen = {};
        outer_loop: for (let j = 0; j < 14; ++j) {
            if (seen[data[i + j]]) {
                break outer_loop;
            }

            seen[data[i + j]] = true;
        }
        if (seen.size === 14) {
            return i + 14;
        }
    }
}


function time(f) {
    const start = Date.now();
    f(data);
    return Date.now() - start;
}

console.log("set", time(faster_set));
console.log("obj", time(faster_obj));
console.log("array", time(faster_arr));
