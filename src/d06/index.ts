import fs from "fs";
import path from "path";

const data = fs.readFileSync(path.resolve(__dirname, "data.txt")).toString();

const samples = [
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
    "bvwbjplbgvbhsrlpgdmjqwftvncz",
    "nppdvjthqldpwncqszvftbrmjlhg",
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
];

function findMarker(transmission: string, uniqueCount: number): number {
    const chars = transmission.split("");

    for (let start = 0; start < chars.length - uniqueCount; start++) {
        const series = chars.slice(start, start + uniqueCount);
        const uniqueChars = new Set(series);
        if (uniqueChars.size === uniqueCount) {
            return start + uniqueCount;
        }
    }

    return -1;
}

samples.forEach((s) => {
    console.log(findMarker(s, 14));
})

const p1 = findMarker(data, 4);
const p2 = findMarker(data, 14);


console.log("PART 1: " + p1);
console.log("PART 2: " + p2);
