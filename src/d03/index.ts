import fs from "fs";
import path from "path";

const data = fs.readFileSync(path.resolve(__dirname, "data.txt")).toString().split("\n");

const sample = [
    "vJrwpWtwJgWrhcsFMMfFFhFp",
    "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
    "PmmdzqPrVvPwwTWBwg",
    "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
    "ttgJtRGJQctTZtZT",
    "CrZsJsPPZsGzwwsLwLmpwMDw",
];
const charsetScores = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    .split("")
    .reduce((map, char, idx) => {
        map[char] = idx + 1;
        return map;
    }, {} as Record<string, number>);

const answer1 = data.reduce((total, line) => {
    const leftSide = line.substring(0, line.length / 2)
    const rightSide = line.substring(line.length / 2);

    const lookupTable: Record<string, number> = {};
    leftSide
        .split("")
        .reduce((set, c) => set.add(c), new Set<string>())
        .forEach((c) => lookupTable[c] = (lookupTable[c] ?? 0) + 1)

    rightSide
        .split("")
        .reduce((set, c) => set.add(c), new Set<string>())
        .forEach((c) => lookupTable[c] = (lookupTable[c] ?? 0) + 1)

    const char = Object.entries(lookupTable).filter((e) => e[1] > 1)[0][0];
    return total + charsetScores[char];
}, 0);

const answer2 = data
    .reduce((prev, line) => {
        const buff = prev[prev.length - 1];
        const set = new Set<string>(line.split(""));

        if (buff.length < 3) {
            buff.push(set)
        } else {
            prev.push([set])
        }

        return prev;
    }, [[]] as Set<string>[][])
    .reduce((total, group) => {
        const lookupTable = group.reduce((lookup, chars) => {
            chars.forEach((c) => lookup[c] = (lookup[c] ?? 0) + 1)
            return lookup;
        }, {} as Record<string, number>);
        const char = Object.entries(lookupTable).filter(([k, v]) => v === 3)[0][0];
        return total + charsetScores[char];
    }, 0)

console.log("Part 1: " + answer1);
console.log("Part 2: " + answer2)
