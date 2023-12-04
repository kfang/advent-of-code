import { Coordinate, Vertex } from "./models";

const byDistance = (a: Vertex, b: Vertex) => a.distance - b.distance;

export function runDijkstraGoingUp(
    vertices: Record<Coordinate, Vertex>,
    start: Vertex,
    end: Vertex,
    minDist: number = Infinity,
): number {
    const queue = Object.values(vertices).map((v) => {
        v.distance = Infinity;
        v.previous = null;
        return v;
    });
    start.distance = 0;
    queue.sort(byDistance);

    let u = queue.shift();
    while (u !== undefined) {
        if (u!.distance > minDist) {
            break;
        }

        u.neighborsGoingUp.forEach((v) => {
            const d = u!.distance + 1;
            if (d < v.distance) {
                v.distance = d;
                v.previous = u!;
            }
        });

        queue.sort(byDistance);
        u = queue.shift();
    }

    return end.distance;
}
