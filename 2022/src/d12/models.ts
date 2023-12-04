const heightMap: Record<string, number> = "abcdefghijklmnopqrstuvwxyz"
    .split("")
    .reduce((res, char, idx) => {
        res[char] = idx
        return res;
    }, {} as Record<string, number>)

export type Coordinate = string;

export class Vertex {
    public static makeCoordinate(x: number, y: number): Coordinate {
        return `(${x},${y})`;
    }

    public static getAdjacent(x: number, y: number): Coordinate[] {
        return [
            Vertex.makeCoordinate(x, y + 1),
            Vertex.makeCoordinate(x, y - 1),
            Vertex.makeCoordinate(x + 1, y),
            Vertex.makeCoordinate(x - 1, y),
        ]
    }

    public readonly coord: Coordinate;
    public readonly isStart: boolean;
    public readonly isEnd: boolean;
    public readonly neighborsGoingUp: Vertex[];
    public readonly neighborsGoingDown: Vertex[];

    // for running dijkstra
    public distance: number;
    public previous: Vertex | null;

    constructor(
        public readonly x: number,
        public readonly y: number,
        public readonly letter: string,
    ) {
        this.coord = Vertex.makeCoordinate(x, y);
        this.isStart = letter === "S";
        this.isEnd = letter === "E";
        this.neighborsGoingUp = [];
        this.neighborsGoingDown = [];

        this.distance = Infinity;
        this.previous = null;
    }

    public get height(): number {
        if (this.isStart) {
            return heightMap["a"];
        }

        if (this.isEnd) {
            return heightMap["z"];
        }

        return heightMap[this.letter];
    }

    public findAndSetNeighbors(vertices: Record<Coordinate, Vertex>): void {
        Vertex.getAdjacent(this.x, this.y).forEach((coord) => {
            const candidate = vertices[coord];
            if (candidate && candidate.height <= (this.height + 1)) {
                this.neighborsGoingUp.push(candidate);
            }

            if (candidate && ((this.height - 1) === candidate.height || this.height === candidate.height)) {
                this.neighborsGoingDown.push(candidate);
            }
        });
    }
}
