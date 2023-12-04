import fs from "fs";
import path from "path";
import { Monkey, WorryLevel } from "./models";


const MONKEY_RGX = /^Monkey ([0-9]+):$/;
const ITEMS_TOK = "Starting items: ";
const CALC_TOK = "Operation: new = ";
const TEST_TOK = "Test: divisible by ";
const TRUE_TOK = "If true: throw to monkey "
const FALSE_TOK = "If false: throw to monkey "

const lines = fs
    .readFileSync(path.resolve(__dirname,  "data.txt"))
    .toString()
    .split("\n")
    .map((l) => l.trim());

const monkeys: Monkey[] = [];

for (const line of lines) {
    if (MONKEY_RGX.test(line)) {
        const id = line.match(MONKEY_RGX)?.[1];
        const monkey = new Monkey(Number(id));
        monkeys.unshift(monkey);
    }

    if (line.startsWith(ITEMS_TOK)) {
        const items = line.substring(ITEMS_TOK.length).split(",").map((n) => new WorryLevel(n));
        monkeys[0].items = items;
    }

    if (line.startsWith(CALC_TOK)) {
        const calc = line.substring(CALC_TOK.length).split(" ");
        monkeys[0].calc = calc;
    }

    if (line.startsWith(TEST_TOK)) {
        const divisor = BigInt(line.substring(TEST_TOK.length));
        monkeys[0].divisor = divisor;
    }

    if (line.startsWith(TRUE_TOK)) {
        const trueTargetId = Number(line.substring(TRUE_TOK.length));
        monkeys[0].trueTargetId = trueTargetId;
    }
    
    if (line.startsWith(FALSE_TOK)) {
        const falseTargetId = Number(line.substring(FALSE_TOK.length));
        monkeys[0].falseTargetId = falseTargetId;
    }
}

monkeys.reverse();

const modulo = monkeys.reduce((res, monkey) => res * monkey.divisor, BigInt(1));

for (let round = 1; round <= 10_000; round++) {
    monkeys.forEach((m) => m.inspect(monkeys, modulo));
    
    if (round === 1) {
        console.log("ROUND " + round);
        monkeys.forEach((m) => {
            console.log(m.id + ": " + m.inspectCount)
        })
    }
    if (round === 20) {
        console.log("ROUND " + round);
        monkeys.forEach((m) => {
            console.log(m.id + ": " + m.inspectCount)
        })
    }
   
    if (round % 1000 === 0) {
        console.log("ROUND " + round);
        monkeys.forEach((m) => {
            console.log(m.id + ": " + m.inspectCount)
        })
    }
}
console.log("---")


const [first, second] = monkeys.map((m) => m.inspectCount).sort((a, b) => b - a);
const monkeyBusiness = first * second;
console.log("MONKEY BUSINESS: " + monkeyBusiness);