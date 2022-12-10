
export class Processor {
    
    public r0: number;
    private pc: number;

    public readonly history: Record<number, number>;

    constructor() {
        this.pc = 1;
        this.r0 = 1;
        this.history = {  1: 1 };
    }

    public get programCounter(): number {
        return this.pc;
    }

    public incPC(): void {
        this.pc++;
        this.history[this.pc] = this.r0;
    }
}

export abstract class Instruction {
    public static build(line: string): Instruction {
        const [command, ...args] = line.split(" ");

        if (command === "addx") {
            return new AddxInsruction(args);
        }

        if (command === "noop") {
            return new NoopInstruction();
        }

        throw new Error("unrecognized instruction line: " + line);
    }

    constructor(public name: string, public cycles: number, public args: string[]) {}

    public abstract run(cycle: number, proc: Processor): boolean;
}

class NoopInstruction extends Instruction {
    constructor() {
        super("noop", 1, []);
    }

    public run(cycle: number): boolean{
        return cycle === 1;
    }
}

class AddxInsruction extends Instruction {
    constructor(args: string[]) {
        super("addx", 2, args);
    }
    
    public run(cycle: number, proc: Processor): boolean {
        if (cycle === 2) {
            proc.r0 += Number(this.args[0])
            return true;
        }
        return false;
    }
}
