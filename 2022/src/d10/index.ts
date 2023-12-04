import fs from "fs";
import path from "path";
import { Instruction, Processor } from "./models";

const instructions = fs
    .readFileSync(path.resolve(__dirname, "data.txt"))
    .toString()
    .split("\n")
    .map(Instruction.build);

const proc = new Processor();

for (const instruction of instructions) {
    let cycle = 1;
    let isDone = false;
    while (!isDone) {
        isDone = instruction.run(cycle, proc);
        cycle++;
        proc.incPC();
    }
    // console.log([proc.programCounter, proc.r0, instruction.name, instruction.args]);
}

// console.log(proc.history);

let totSigStr = 0;
for (let i = 20; i <= 220; i+=40) {
    const sigStr = i * proc.history[i];
    // console.log([i,  proc.history[i], sigStr]);
    totSigStr += sigStr;
}

console.log("PART 1: " + totSigStr);
console.log(proc.history);

const screen: string[][] = Array(6).fill("").map(() => Array(40).fill("."));

for (let i = 0; i < 240; i++) {
    const r0 = proc.history[i + 1];
    const row = Math.floor(i / 40);
    const col = i % 40;

    if (col >= r0 - 1 && col <= r0 + 1) {
        screen[row][col] = "#";
    }
}


screen.forEach((row) => console.log(row.join("")));