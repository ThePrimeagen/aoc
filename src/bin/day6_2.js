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

function benny(data) {
    let filter = 0;
    let ones = 0;
    for (let i = 0; i < 13; ++i) {
        const next = filter ^ (1 << (data[i] % 32));
        ones += next > filter ? 1 : -1;
        filter = next;
    }

    for (let i = 0; i + 14 < data.length; ++i) {
        let first = data[i];
        let last = data[i + 13];

        const last_filter = filter ^ (1 << (last % 32));
        ones += last_filter > filter ? 1 : -1;
        filter = last_filter;

        if (ones === 14) {
            return i + 14;
        }

        const first_filter = filter ^ (1 << (first % 32));
        ones += first_filter > filter ? 1 : -1;
        filter = first_filter;
    }

    return -1;
}


function time(f) {
    const start = Date.now();
    f(data);
    return Date.now() - start;
}


for (let i = 0; i < 10; ++i) {
    console.log("obj,", time(faster_obj));
}

for (let i = 0; i < 10; ++i) {
    console.log("set,", time(faster_set));
}

for (let i = 0; i < 10; ++i) {
    console.log("array,", time(faster_arr));
}

for (let i = 0; i < 10; ++i) {
    console.log("benny", time(benny));
}

