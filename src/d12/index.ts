import fs from "fs";
import path from "path";
import { runDijkstraGoingUp } from "./dijkstra";
import { Coordinate, Vertex } from "./models";

const grid: string[][] = fs
    .readFileSync(path.resolve(__dirname, "data.txt"))
    .toString()
    .split("\n")
    .map((row) => row.split(""));

let start: Vertex;
let end: Vertex | null = null;
const vertices: Record<Coordinate, Vertex> = {};

// convert grid to vertices
for (let y = 0; y < grid.length; y++) {
    const row = grid[y];
    for (let x = 0; x < row.length; x++) {
        const letter = row[x];
        const vertex = new Vertex(x, y, letter);
        vertices[vertex.coord] = vertex;
        if (vertex.isEnd) {
            end = vertex;
        }

        if (vertex.isStart) {
            start = vertex;
        }
    }
}

// calculate all the neighbors
Object.values(vertices).forEach((v) => v.findAndSetNeighbors(vertices));

runDijkstraGoingUp(vertices, start!, end!);
console.log("PART 1: " + end?.distance);

let min = Infinity;
Object.values(vertices)
    .filter((v) => v.height === 0)
    .forEach((v) => {
        const res = runDijkstraGoingUp(vertices, v, end!, min);
        min = Math.min(min, res);
    });

console.log("PART 2: " + min);