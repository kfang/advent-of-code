import fs from "fs";
import path from "path";
import { isInOrder, PacketData, Pair } from "./models";

const pairs = fs
    .readFileSync(path.resolve(__dirname, "data.txt"))
    .toString()
    .split("\n\n")
    .map((l, idx) => {
        const [left, right] = l.split("\n").map((s) => JSON.parse(s));
        return new Pair(idx + 1, left, right);
    });

pairs.filter((p) => p.isInOrder === null).forEach((p) => {
    console.log("BAD PAIR: " + p.id);
    console.dir(p);
});

const p1 = pairs
    .filter((p) => p.isInOrder)
    .reduce((tot, p) => {
        return tot + p.id;
    }, 0)

console.log("PART 1: " + p1);

const allPacketData: PacketData[] = pairs.flatMap((p) => [p.left, p.right]);
allPacketData.push([[2]], [[6]]);

let fin = false;
while (!fin) {
    let didSwap = false;
    for (let idx = 0; idx < allPacketData.length - 1; idx++) {
        const left = allPacketData[idx];
        const right = allPacketData[idx + 1];
        if (!isInOrder(left, right)) {
            allPacketData[idx] = right;
            allPacketData[idx + 1] = left;
            didSwap = true;
        }
    }
    fin = !didSwap;
}

const p2 = allPacketData.reduce((res, pd, idx) => {
    if (JSON.stringify(pd) === JSON.stringify([[2]])) {
        return res * (idx + 1)
    }

    if (JSON.stringify(pd) === JSON.stringify([[6]])) {
        return res * (idx + 1)
    }

    return res;
}, 1);

console.log("PART 2: " + p2);