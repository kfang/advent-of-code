export class RockPath {

    public readonly rockPoints: number[][];
    public readonly minX: number;
    public readonly maxX: number;
    public readonly minY: number;
    public readonly maxY: number;

    constructor(path: string) {
        this.rockPoints = [];

        const points = path.split(" -> ");
        for (let i = 0; i < points.length - 1; i++) {
            const [startX, startY] = points[i].split(",").map(Number);
            const [endX, endY] = points[i + 1].split(",").map(Number);

            if (startX === endX) {
                // go down Y axis
                const minY = Math.min(startY, endY);
                const maxY = Math.max(startY, endY);
                for (let y = minY; y <= maxY; y++) {
                    const p = [startX, y];
                    this.rockPoints.push(p);
                }
            }

            if (startY === endY) {
                // go down X axis
                const minX = Math.min(startX, endX);
                const maxX = Math.max(startX, endX);
                for (let x = minX; x <= maxX; x++) {
                    const p = [x, startY];
                    this.rockPoints.push(p);
                }
            }
        }

        const xValues = this.rockPoints.map((p) => p[0]);
        this.minX = Math.min(...xValues);
        this.maxX = Math.max(...xValues);

        const yValues = this.rockPoints.map((p) => p[1]);
        this.minY = Math.min(...yValues);
        this.maxY = Math.max(...yValues);
    }
}

export class Cave {
    private rockPaths: RockPath[]
    private rockPathYMax: number;

    private gridXOffset: number;
    private gridCols: number;
    private gridRows: number;

    private grid: string[][];

    constructor() {
        this.rockPaths = [];
        this.rockPathYMax = 0;

        this.gridXOffset = 0;
        this.gridCols = 0;
        this.gridRows = 0;

        this.grid = [];
        this.addRockPaths = this.addRockPaths.bind(this);
    }

    public addRockPaths(paths: string[]): void {
        this.rockPaths = paths.map((p) => new RockPath(p));
        const rockPathXMin = Math.min(...this.rockPaths.map((rp) => rp.minX))
        const rockPathXMax = Math.max(...this.rockPaths.map((rp) => rp.maxX))
        this.rockPathYMax = Math.max(...this.rockPaths.map((rp) => rp.maxY))

        this.gridCols = rockPathXMax - rockPathXMin + 1;
        this.gridRows = this.rockPathYMax + 3;
        this.gridXOffset = rockPathXMin;

        this.renderGrid();
    }

    public addSand(): boolean {
        let currX = 500 - this.gridXOffset;
        let currY = 0;

        let overEdge = false;
        let canFall = true;
        while (canFall) {
            // try down
            if (this.grid[currY + 1][currX] === ".") {
                currY = currY + 1;
                continue;
            }

            const leftX = currX - 1;
            if (leftX < 0) {
                overEdge = true;
                break;
            }

            // try down left
            if (this.grid[currY + 1][leftX] === ".") {
                currY = currY + 1;
                currX = currX - 1;
                continue;
            }

            const rightX = currX + 1
            if (rightX >= this.gridCols) {
                overEdge = true;
                break;
            }

            // try down right
            if (rightX < this.gridCols && this.grid[currY + 1][rightX] === ".") {
                currY = currY + 1;
                currX = currX + 1;
                continue;
            }

            canFall = false;
        }

        if (!overEdge) {
            this.grid[currY][currX] = "+";
        }

        return !overEdge;
    }

    public addSandUnbounded(): boolean {
        let currX = 500 - this.gridXOffset;
        let currY = 0;

        let canFall = true;
        while (canFall) {
            // try down
            if (this.grid[currY + 1][currX] === ".") {
                currY = currY + 1;
                continue;
            }

            let leftX = currX - 1;
            if (leftX < 0) {
                this.addCol("start");
                leftX++;
            }

            // try down left
            if (this.grid[currY + 1][leftX] === ".") {
                currY = currY + 1;
                currX = leftX;
                continue;
            }

            const rightX = currX + 1
            if (rightX >= this.gridCols) {
                this.addCol("end");
            }

            // try down right
            if (rightX < this.gridCols && this.grid[currY + 1][rightX] === ".") {
                currY = currY + 1;
                currX = currX + 1;
                continue;
            }

            canFall = false;
        }

        this.grid[currY][currX] = "+";

        return currY !== 0;
    }

    public renderGrid(): void {
        this.grid = Array(this.gridRows).fill("")
            .map(() => Array(this.gridCols).fill("."));

        this.rockPaths.forEach((rp) => {
            for (const [x, y] of rp.rockPoints) {
                this.grid[y][x - this.gridXOffset] = "#";
            }
        });
    }

    public fillFloor(): void {
        this.grid[this.gridRows - 1].fill("#");
    }

    public clear(): void {
        this.grid.forEach((row) => {
            row.forEach((s, idx) => {
                if (s === "+") {
                    row[idx] = ".";
                }
            })
        })
    }

    public sandCount(): number {
        let c = 0;
        this.grid.forEach((row) => {
            row.forEach((s, idx) => {
                if (s === "+") {
                    c++;
                }
            })
        });
        return c;
    }

    public addCol(direction: "start" | "end"): void {
        // add col to beginning
        if (direction === "start") {
            this.grid.forEach((row) => row.unshift("."));
            this.gridXOffset--;
        }

        if (direction === "end") {
            this.grid.forEach((row) => row.push("."));
        }

        this.gridCols++;
        this.fillFloor();
    }

    public print(): void {
        this.grid.forEach((row) => {
            console.log(row.join(""));
        });
    }
}