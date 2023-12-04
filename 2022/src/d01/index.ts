import fs from "fs";
import path from "path";

const data = fs.readFileSync(path.resolve(__dirname, "data.txt")).toString().split("\n");

const results = [];

const buff = [];
for (const line of data) {
    if (line === "") {
        console.log()
        const arr = buff.splice(0, buff.length);
        const sum = arr.reduce((res, num) => res + num, 0);
        results.push(sum)
    } else {
        const num = Number(line);
        buff.push(num);
    }
}

const sorted = results.sort((a, b) => b - a);

console.log(results);
console.log(sorted);

console.log("Part 1: " + Math.max(...sorted));
console.log("Part 2: " + (sorted[0] + sorted[1] + sorted[2]))