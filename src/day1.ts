import _ from "@nomiclabs/hardhat-ethers";
import { ethers } from "hardhat";
import * as fs from "fs";
import * as path from "path";

async function deploy() {
    const Day1 = await ethers.getContractFactory("Day1");
    const d = await Day1.deploy();
    await d.deployed();
    const contents = fs.readFileSync(path.join(__dirname, "day1.input")).
        toString().
        split("\n").
        filter(x => x.length).
        map(x => +x);

    for (let i = 0; i < contents.length; ++i) {
        await d.addInput(contents[i]);
    }

    console.log("The answer is", await d.getDay1_2Res(), " brought to you buy this shitty language javascript");
}

deploy();

