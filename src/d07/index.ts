import fs from "fs";
import path from "path";

const FILENAME = "data.txt";
const TOTAL_DISK = 70000000;
const UNUSED_DISK_NEEDED = 30000000;

const lines = fs.readFileSync(path.resolve(__dirname, FILENAME)).toString().split("\n");

const changeDirRgx = /^\$ cd (.+)$/

class Node {
    public readonly children: Node[];
    constructor(public readonly name: string, public readonly size?: number, public readonly parent?: Node) {
        this.children = [];
    }

    public changeTo(name: string): Node {
        if (name === "..") {
            if (this.parent) {
                return this.parent
            } else {
                throw new Error("No Parent")
            }
        }

        const next = this.children.find((child) => child.name === name);
        if (!next) {
            const child = new Node(name, undefined, this);
            this.children.push(child);
            return child;
        }
        return next;
    }

    public addChild(name: string): void {
        const [desc, actualName] = name.split(" ");
        const size = desc === "dir" ? undefined : Number(desc);
        const exists = this.children.some((child) => child.name === actualName);
        if (!exists) {
            this.children.push(new Node(actualName, size, this));
        }
    }

    public totalSize(): number {
        if (this.size) {
            return this.size;
        }

        return this.children.reduce((total, child) => {
            return total + child.totalSize();
        }, 0)
    }

    public print(indents: number = 0): void {
        console.log(" ".repeat(indents) + this.printedName);
        this.children.forEach((child) => {
            child.print(indents + 2);
        })
    }

    public get printedName(): string {
        const desc = this.size ? `(file, size=${this.size})` : `(dir, total=${this.totalSize()})`
        return this.name + " " + desc;
    }

    public get isDir(): boolean {
        return this.size === undefined;
    }

}

const root = new Node("_ROOT_");
let cwd: Node = root;

for (let idx = 0; idx < lines.length; idx++) {
    const line = lines[idx];

    if (changeDirRgx.test(line)) {
        const dirname = line.match(changeDirRgx)?.[1];
        if (!dirname) {
            throw new Error("NO DIRNAME: " + line);
        }
        cwd = cwd.changeTo(dirname);
    }

    if (line === "$ ls") {
        idx++;
        let next = lines[idx];
        while (idx < lines.length && next && !next.startsWith("$")) {
            cwd.addChild(next);
            idx++;
            next = lines[idx];
        }
        idx--;
    }
}

function findDirs(root: Node): Node[]  {
    const rootDirs = root.children.filter((c) => c.isDir);
    const childDirs = rootDirs.flatMap(findDirs);
    return [...rootDirs, ...childDirs]
}

console.log(root.print());

const part1 = findDirs(root)
    .filter((n) => {
        return n.totalSize() < 100_000
    })
    .reduce((total, node) => {
        return total + node.totalSize();
    }, 0)

console.log(part1);

const spaceLeft = TOTAL_DISK - root.totalSize();
const minSizeToDelete = UNUSED_DISK_NEEDED - spaceLeft;

const p2 = findDirs(root)
    .filter((node) => node.totalSize() > minSizeToDelete)
    .sort((a, b) => a.totalSize() - b.totalSize())[0]

console.log(p2.printedName);

