import fs from "fs";
import path from "path";
import { Cave } from "./cave";

const cave = new Cave();

const paths = fs.readFileSync(path.resolve(__dirname, "data.txt"))
    .toString()
    .split("\n");
cave.addRockPaths(paths);

let added = true;
while (added) {
    added = cave.addSand();
    // console.log("==== " + count + " ====");
    // cave.print();
}

console.log("P1: " + cave.sandCount());

cave.clear();
cave.fillFloor();

added = true;
while (added) {
    added = cave.addSandUnbounded();
    // console.log("==== " + count + " ====");
    // cave.print();
}

console.log("P2: " + cave.sandCount());
