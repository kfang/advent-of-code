import fs from "fs";
import path from "path";
const data = fs.readFileSync(path.resolve(__dirname, "data.txt")).toString().split("\n");

const sample = [
    "2-4,6-8",
    "2-3,4-5",
    "5-7,7-9",
    "2-8,3-7",
    "6-6,4-6",
    "2-6,4-8",
];

const convertToPairs = (line: string): number[][] => {
    return line.split(",").map((pair) => pair.split("-").map(Number));
}

const contains = (a: number[], b: number[]): boolean => {
    return (a[0] <= b[0] && a[1] >= b[1]) || (b[0] <= a[0] && b[1] >= a[1]);
}

const overlaps = (a: number[], b: number[]): boolean => {
    const seen = new Set<number>();
    for (let i = a[0]; i <= a[1]; i++) {
        seen.add(i);
    }

    for (let i = b[0]; i <= b[1]; i++) {
        if (seen.has(i)) {
            return true;
        }
        seen.add(i);
    }

    return false;
}

const p1 = data.reduce((total, line) => {
    const [first, second] = convertToPairs(line);
    return contains(first, second) ? total + 1 : total;
}, 0);

const p2 = data.reduce((total, line) => {
    const [first, second] = convertToPairs(line);
    return overlaps(first, second) ? total + 1 : total;
}, 0);

console.log("PART 1: " + p1);
console.log("PART 2: " + p2);
