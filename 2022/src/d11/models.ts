
export class WorryLevel {
    private value: bigint;

    constructor(v: string) {
        this.value = BigInt(v);
        this.isDivisibleBy = this.isDivisibleBy.bind(this);
        this.updateWith = this.updateWith.bind(this);
    }

    public isDivisibleBy(d: bigint): boolean {
        return this.value % d === BigInt(0);
    }

    public updateWith(calc: string[], modulo: bigint): void {
        const a = calc[0] === "old" ? this.value : BigInt(calc[0]);
        const b = calc[2] === "old" ? this.value : BigInt(calc[2]);
        const op = calc[1];
          
        if (op === "*") {
            this.value = (a * b) % modulo;
        } else if (op === "+") {
            this.value = (a + b) % modulo;
        } else {
            throw new Error("unknown op " + calc);
        }
    }
}

export class Monkey {

    public items: WorryLevel[];
    public calc: string[];
    public divisor: bigint;
    public trueTargetId: number;
    public falseTargetId: number;

    public inspectCount: number;

    constructor(public readonly id: number) {
        this.items = [];
        this.calc = [];
        this.divisor = BigInt(0);
        this.trueTargetId = -1;
        this.falseTargetId = -1;
        this.inspectCount = 0;
    }

    public inspect(monkeys: Monkey[], modulo: bigint) {
        // console.log("MONKEY " + this.id);        
        
        let item = this.items.shift();
        while (item) {
            // console.log("  CURR: " +  item.score);
            item.updateWith(this.calc, modulo);
            // console.log("  NEXT: " + item.score);
            
            // console.log("div");
            // console.log(item.isDivisibleBy(this.divisor));
            const target = item.isDivisibleBy(this.divisor)
                ? this.trueTargetId
                : this.falseTargetId;

            // console.log("  TARG: " + target);

            // console.log("throw");
            monkeys[target].items.push(item);
            item = this.items.shift();
            this.inspectCount++;
        }
    }
}