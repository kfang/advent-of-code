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


function checkSensors(point: number[], exclude: number): boolean {
    let idx = 0;
    for (const s of grid.sensors) {
        if (idx !== exclude) {
            const isInRange = s.isInRange(point);
            if (isInRange) {
                return true;
            }
        }
        idx++;
    }

    return false;
}


const min = 0;
const max = 4_000_000;
// const max = 20;

let point: number[] = [];
grid.sensors.forEach((s,  idx) => {
    const points = s.calcBoundaryPoints(min, max);
    points.forEach(p => {
        const check = checkSensors(p, idx);
        
        if (check === false) {
            point = p;
        }
    })

    console.log("DONE with " + idx, points.length);
});


const p2 = (point[0] * 4000000) + point[1];
console.log(p2);