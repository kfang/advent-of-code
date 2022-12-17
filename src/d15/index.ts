import fs from "fs";
import path from "path";
import { Beacon, Grid, Sensor } from "./models";

const FILENAME = "data.txt";
const DATA_RGX = /Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)/;

const grid = fs
    .readFileSync(path.resolve(__dirname, FILENAME))
    .toString()
    .split("\n")
    .reduce(
        (res, line) => {
            const matches = line.match(DATA_RGX) ?? [];

            const beaconX = Number(matches[3]);
            const beaconY = Number(matches[4]);
            const beacon = new Beacon(beaconX, beaconY);
            res.beacons.push(beacon);

            const sensorX = Number(matches[1]);
            const sensorY = Number(matches[2]);
            const sensor = new Sensor(sensorX, sensorY, beacon);
            res.sensors.push(sensor);

            return res;
        },
        new Grid(),
    )

// console.log(grid.countNoPositions(2000000));

const min = 0;
const max = 4_000_000;
// const max = 20;
grid.sensors.forEach((s,  idx) => {
    s.calcBoundaryPoints(min, max);
    console.log("DONE with " + idx);
});

const points = grid.sensors.reduce((res,  s) => {
    return res + s.boundaryPoints.length;
}, 0);
console.log(points);


for (let y = min; y <= max; y++) {
    const res = grid.getPossiblePositions(y, min, max);
    if (y % 1_000 === 0) {
        console.log(`crossed ${y}`);
    }

    if (res.length > 0) {
        console.log(`([${res}], ${y})`);
    }
}