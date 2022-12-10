import fs from "fs";
import path from "path";
const data = fs.readFileSync(path.resolve(__dirname, "data.txt")).toString().split("\n");

const sample = [
    "    [D]",
    "[N] [C]",
    "[Z] [M] [P]",
    " 1   2   3",
    "",
    "move 1 from 2 to 1",
    "move 3 from 1 to 3",
    "move 2 from 2 to 1",
    "move 1 from 1 to 2",
];

interface Instruction {
    count: number;
    source: number;
    destination: number;
}

const stacks: Record<number, string[]> = {};
const instructions: Instruction[] = [];

const instructionRgx = /^move ([0-9]+) from ([0-9]+) to ([0-9]+)$/;

for (const line of data) {
    const [_, count, source, destination] = (line.match(instructionRgx) ?? []).map(Number);
    if (count && source && destination) {
        instructions.push({ count, source, destination })
    } else {
        const chars = line.split("");
        for (let idx = 1; idx < chars.length; idx += 4) {
            const col = Math.round((idx / 4)) + 1
            const char = chars[idx];
            if (char !== " " && chars[idx-1] === "[") {
                const stack = (stacks[col] ?? []);
                stacks[col] = [...stack, char]
            }
        }
    }
}

// part 1 - cranemover 9000
// instructions.forEach((instruction) => {
//     const sourceStack = stacks[instruction.source];
//     const destinationStack = stacks[instruction.destination];
//     for (let moved = 0; moved < instruction.count; moved++) {
//         const item = sourceStack.shift()!;
//         destinationStack.unshift(item);
//     }
// });

instructions.forEach((instruction) => {
    const sourceStack = stacks[instruction.source];
    const destinationStack = stacks[instruction.destination];

    const items = sourceStack.splice(0, instruction.count);
    destinationStack.unshift(...items);
});

console.log(stacks);
console.log(instructions);

const numStacks = Object.keys(stacks).length
let result = "";
for (let i = 1; i <= numStacks; i++) {
    result += stacks[i][0];
}
console.log(result);