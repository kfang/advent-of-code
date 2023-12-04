import fs from "fs"
import path from "path";

const lines = fs.readFileSync(path.resolve(__dirname, "data.txt")).toString().split("\n");

class Coord {
    constructor(public segment: number, public x: number, public y: number) {}
    public move(direction: string) {
        if (direction === "R") {
            this.x++;
        }

        if (direction === "L") {
            this.x--;
        }

        if (direction === "U") {
            this.y++;
        }

        if (direction === "D") {
            this.y--
        }
    }

    public follow(coord: Coord): void {
        const xDiff = coord.x - this.x;
        const yDiff = coord.y - this.y;

        if (xDiff > 1) {
            this.x++;
        }
        if (xDiff < -1) {
            this.x--;
        }
        if (yDiff > 1) {
            this.y++;
        }
        if (yDiff < -1) {
            this.y--;
        }

        // deal with diagonals
        if (Math.abs(xDiff) > 1 && Math.abs(yDiff) === 1) {
            this.y += yDiff;
        }

        // deal with diagonals
        if (Math.abs(yDiff) > 1 && Math.abs(xDiff) === 1) {
            this.x += xDiff;
        }
    }

    public get posStr(): string {
        return [this.x, this.y].join(",");
    }
}

const rope = Array(10).fill("").map((_, idx) => new Coord(idx, 0, 0));

const positions = new Set<string>(["0,0"]);

for (const line of lines) {
    const [direction, distance] = line.split(" ");

    console.log("-- " + line + " ------------------");
    for (let step = 0; step < Number(distance); step++) {
        // move the head
        rope[0].move(direction);

        // reconcile the rest
        for(let idx = 1; idx < rope.length; idx++) {
            const segment = rope[idx];
            segment.follow(rope[idx - 1]);
        }

        positions.add(rope[rope.length - 1].posStr)
    }
    console.log(rope)
}
// console.log(head);
// console.log(tail);
console.log(positions.size);
// console.log(positions.keys());
