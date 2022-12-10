import { EnumEntry, Match, Optional } from "@kfang/typescript-fp";
import fs from "fs";
import path from "path";

enum RPS {
   Rock,
   Paper,
   Scissors
}

class RPSShape extends EnumEntry<RPS> {

   public static ROCK = new RPSShape(RPS.Rock, "A", "X", RPS.Scissors, RPS.Paper, 1);
   public static PAPER = new RPSShape(RPS.Paper, "B", "Y", RPS.Rock, RPS.Scissors, 2);
   public static SCISSORS = new RPSShape(RPS.Scissors, "C", "Z", RPS.Paper, RPS.Rock, 3);

   public static withP1Value(str: string): Optional<RPSShape> {
      return Optional.of(RPSShapes.entries.find(({ p1Value }) => str === p1Value));
   }

   public static withP2Value(str: string): Optional<RPSShape> {
      return Optional.of(RPSShapes.entries.find(({ p2Value }) => str === p2Value));
   }

   constructor(
       value: RPS,
       public readonly p1Value: string,
       public readonly p2Value: string,
       public readonly beats: RPS,
       public readonly loses: RPS,
       public readonly score: number,
    ) {
      super(value);
   }

   public scoreAgainst(shape: RPSShape): number {
      if (shape.value === this.value) {
         return 3;
      }

      if (shape.beats === this.value) {
         return 0;
      }

      return 6;
   }
}

const RPSShapes = EnumEntry.seal(RPSShape);

const data = fs.readFileSync(path.resolve(__dirname, "data.txt")).toString().split("\n");
const d = ["A Y", "B X", "C Z"];

const part1 = data.reduce((t, line) => {
   const predictions = line.split(" ");
   const p1 = RPSShapes.withP1Value(predictions[0]).getOrThrow(new Error(line))
   const p2 = RPSShapes.withP2Value(predictions[1]).getOrThrow(new Error(line));
   const score = p2.score + p2.scoreAgainst(p1);
   return t + score
}, 0);

const part2 = data.reduce((t, line) => {
   const predictions = line.split(" ");
   const p1 = RPSShapes.withP1Value(predictions[0]).getOrThrow(new Error(line))

   const p2 = RPSShapes.withValue(
       Match
          .case("X", p1.beats)
          .case("Y", p1.value)
          .case("Z", p1.loses)
          .toOptional()(predictions[1])
          .get()
   ).get();

   const score = p2.score + p2.scoreAgainst(p1);
   return t + score
}, 0)

console.log("Part 1: " + part1);
console.log("Part 2: " + part2);
