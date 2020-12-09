import * as fs from "fs";
import * as path from "path";

const contents = fs.readFileSync(path.join(__dirname, "input.prod")).
    toString().
    split("\n");

type BNode = {
    counted: boolean;
    D: string;
    children: BNode[];
};

type BNodeForest = {
    [key: string]: BNode
}

const forest: BNodeForest = {};

function getNode(name: string): BNode {
    if (!forest[name]) {
        forest[name] = {
            counted: false,
            D: name,
            children: []
        };
    }

    return forest[name];
}

contents.filter(x => x.length).forEach(line => {
    let splitLines = line.split(" ");

    if (splitLines.length === 0) {
        return;
    }

    const name = splitLines.slice(0, 2).join(" ");
    const node = getNode(name);

    if (line.includes("no other bags")) {
        return;
    }

    splitLines = splitLines.slice(4);

    do {
        const bag = splitLines.slice(1, 3).join(" ");
        splitLines = splitLines.slice(4);

        node.children.push(getNode(bag));
    } while (splitLines.length >= 4);
});

const _8 = "shiny gold";

let totalCount = 0;
function countEmBoys(walkList: BNode[]) {
    walkList.forEach(n => {
        if (!n.counted) {
            totalCount++;
        }
        n.counted = true;
    });
}

function giveMeTheGold(node: BNode, walkList: BNode[] = []) {
    if (node.D===_8) {
        countEmBoys(walkList);
        return;
    }

    walkList.push(node);
    node.children.forEach(c => giveMeTheGold(c, walkList));
    walkList.pop();
}

for (const value of Object.values(forest)) {
    giveMeTheGold(value);
}

console.log("FOUND BAGZ", totalCount);

