import fs from "fs";
import path from "path";

const lines = fs.readFileSync(path.resolve(__dirname, "data.txt")).toString().split("\n");

class Tree {
    public leftScore: number;
    public rightScore: number;
    public topScore: number;
    public botomScore: number;
    constructor(readonly height: number, public isVisible: boolean) {
        this.leftScore = 0;
        this.rightScore = 0;
        this.topScore= 0;
        this.botomScore = 0;
    }

    get score(): number {
        return this.leftScore * this.rightScore * this.topScore * this.botomScore;
    }
}

const forest = lines.map((line, row) => {
    const cols = line.split("").map(Number);
    return cols.map((height, col) => {
        const isVisible = row === 0  || row === (lines.length - 1) || col === 0 || col === (cols.length - 1);
        return new Tree(height, isVisible)
    });
});

function scan(treeline: Tree[]): void {
    let maxHeight: number;

    // scan from left
    maxHeight = treeline[0].height;
    treeline.forEach((tree) => {
        if (tree.height > maxHeight) {
            maxHeight = tree.height;
            tree.isVisible = true;
        }
    });

    // scan from right
    maxHeight = treeline[treeline.length - 1].height;
    for (let col = treeline.length - 1; col > 0; col--) {
        const tree = treeline[col];
        if (tree.height > maxHeight) {
            maxHeight = tree.height;
            tree.isVisible = true;
        }
    }
}

// scanning from left and right
forest.forEach((treeline, row) => {
    // skip first and last, they're all visible already
    if (row === 0 || row === forest.length - 1) {
        return;
    }
    scan(treeline);
});

// scanning from top and bottom
for (let col = 1; col < forest[0].length - 1; col++) {
    const column = forest.map((line) => line[col]);
    scan(column);
}

const p1 = forest.reduce((total, treeline) => {
    return total + treeline.filter((t) => t.isVisible).length;
}, 0)

for (let rowNum = 0; rowNum < forest.length; rowNum ++) {
    const row = forest[rowNum];
    for (let colNum = 0; colNum < row.length; colNum++) {
        const col = forest.map((r) => r[colNum]);

        const tree = row[colNum];

        // looking left
        for (let idx = colNum - 1; idx >= 0; idx--) {
            const n = row[idx];
            tree.leftScore++;
            if (n.height >= tree.height) {
                break;
            }
        }

        // looking right
        for (let idx = colNum + 1; idx < row.length; idx++) {
            const n = row[idx];
            tree.rightScore++;
            if (n.height >= tree.height) {
                break;
            }
        }

        // look up
        for (let idx = rowNum - 1; idx >= 0; idx--) {
            const n = col[idx];
            tree.topScore++;
            if (n.height >= tree.height) {
                break;
            }
        }

        // look down
        for (let idx = rowNum + 1; idx < col.length; idx++) {
            const n = col[idx];
            tree.botomScore++;
            if (n.height >= tree.height) {
                break;
            }
        }
    }
}

console.log(forest);
const p2 = Math.max(...forest.flatMap((txs) => txs).map((t) => t.score));
console.log(p2);

