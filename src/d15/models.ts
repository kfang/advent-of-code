export class Beacon {
    constructor(public x: number, public y: number) {}
}

export class Sensor {
    public readonly distToBeacon: number;

    public boundaryPoints: string[];

    constructor(
        public x: number,
        public y: number,
        public closestBeacon: Beacon,
    ) {
        this.distToBeacon = Math.abs(x - closestBeacon.x) +  Math.abs(y  -  closestBeacon.y);
        this.boundaryPoints = [];
    }

    public calcBoundaryPoints(min: number, max: number): string[] {

        const minX = this.x - this.distToBeacon - 1;
        const maxX = this.x + this.distToBeacon + 1;

        const points = [];
        for (let i = Math.max(min, minX); i <= this.x; i++) {
            points.push(`(${i}, ${i + (this.y - minX)})`)
            points.push(`(${i}, ${-i + (this.y + minX)})`)
        }


        for (let i = this.x; i <= Math.min(max, maxX); i++) {
            points.push(`(${i}, ${-i + (this.y + maxX)})`)
            points.push(`(${i}, ${i + (this.y - maxX)})`)
        }

        this.boundaryPoints = points;
        return this.boundaryPoints;
    }

    public coveredOnRow(y: number, minX?: number, maxX?: number): number[]  {
        const xOffset = this.distToBeacon - Math.abs(this.y - y);
        if (xOffset <= 0) {
            return [];
        }

        const xValues = [];

        let startX = this.x - xOffset;
        if (minX) {
            startX = Math.max(minX, startX)
        }

        let endX = this.x + xOffset;
        if (maxX) {
            endX = Math.min(maxX, endX);
        }

        for (let x = startX; x <= endX; x++) {
            xValues.push(x);
        }

        return xValues;
    }
}

export class Grid {
    public readonly beacons: Beacon[];
    public readonly sensors: Sensor[];

    constructor() {
        this.beacons = [];
        this.sensors = [];
    }

    public getPossiblePositions(y: number, min: number, max: number): number[] {
        const posx = Array(max - min + 1).fill("").map((_, i) => min + i);
        const set = new Set<number>(posx);

        for (const sensor of this.sensors) {
            if (set.size === 0) {
                break;
            }

            sensor.coveredOnRow(y, min, max).forEach((x) => set.delete(x));
        }

        return [...set.values()];
    }

    public countNoPositions(y: number, min?: number, max?: number): number {
        const set = new Set<number>();
        this.sensors.map((s) => {
            s.coveredOnRow(y, min, max).forEach((x) => set.add(x));
        });

        this.beacons
            .filter((b) => b.y === y)
            .forEach((b) => set.delete(b.x));

        return set.size;
    }
}