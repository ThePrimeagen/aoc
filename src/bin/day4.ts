//@ts-ignore
import fs from "fs";
//@ts-ignore
import path from "path";

function createRange(value: string): [number, number] {
	return value.split("-").map(x => +x) as [number, number];
}

type Thing = [[number, number], [number, number]];
//@ts-ignore
console.log(fs.readFileSync(path.join(__dirname, "day4.test")).
	toString().
	split("\n").
//@ts-ignore
	filter(x => x).
//@ts-ignore
	map(x => {
		let [left, right] = x.split(",");
		let one = createRange(left);
		let two = createRange(right);

		console.log("hello", one, two);

		return one[0] <= two[0] && one[1] >= two[1] ||
			two[0] <= one[0] && two[1] >= one[1];
	}).
//@ts-ignore
	filter(x => x).length);
	

