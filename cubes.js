const fs = require('fs');

const contents = fs.readFileSync('d2', 'utf8').split('\n').reduce((acc, line) => {
    if (line === "") {
        return acc;
    }

    const max = {
        red: 0,
        blue: 0,
        green: 0,
    }

    line.split(":")[1].split(";").forEach(game => {
        game.split(",").forEach(dice => {
            const color = dice.trim().split(" ")[1]
            const count = +dice.trim().split(" ")[0]
            max[color] = Math.max(max[color], count);
        });
    });

    const newMax = max['red'] * max['blue'] * max['green']

    return newMax + acc;
}, 0);

console.log(contents);
