import * as fs from "fs";
import * as path from "path";

const contents = fs.readFileSync(path.join(__dirname, "input.prod")).
    toString().
    split("\n");

type NodeAndCount = {
    count: number;
    node: BNode;
}

type BNode = {
    counted: boolean;
    D: string;
    children: NodeAndCount[],
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

        node.children.push({
            count: +splitLines[0],
            node: getNode(bag)
        });

        splitLines = splitLines.slice(4);
    } while (splitLines.length >= 4);
});

function giveMeTheChildren(node: NodeAndCount, c: number = 1): number {
    if (node.node.children.length === 0) {
        return c * node.count;
    }

    c *= node.count;

    let total = c;
    console.log("gmyc", node.node.D, total, c);

    node.node.children.forEach((child: NodeAndCount) => {
        const t = giveMeTheChildren(child, c);
        console.log("  gmyc - children", node.node.D, child.node.D, t);
        total += t;
    });

    console.log("gmyc - end", node.node.D, total);

    return total;
}

const shinyGold = getNode("shiny gold");
const found = new Set<BNode>();
const foundCount = shinyGold.children.reduce((acc: number, child: NodeAndCount) => {
    return acc + giveMeTheChildren(child);
}, 0);

console.log("FOUND BAGZ", foundCount);
