export type PacketData = (number | PacketData)[];

function isNil<T>(v?: T): boolean {
    return v === undefined || v === null;
}

export function isInOrder(left: PacketData, right: PacketData): boolean | null {
    for (let idx = 0; idx < Math.max(left.length, right.length); idx++) {
        const leftVal = left[idx];
        const rightVal = right[idx];

        if (!isNil(leftVal) && isNil(rightVal)) {
            return false;
        } else if (isNil(leftVal) && !isNil(rightVal)) {
            return true;
        } else if (typeof leftVal === "number" && typeof rightVal === "number") {
            if (leftVal < rightVal) {
                return true;
            }
            if (leftVal > rightVal) {
                return false;
            }
        } else if (Array.isArray(leftVal) && !Array.isArray(rightVal)) {
            const res = isInOrder(leftVal, [rightVal]);
            if (res !== null) {
                return res;
            }
        } else if (!Array.isArray(leftVal) && Array.isArray(rightVal)) {
            const res = isInOrder([leftVal], rightVal);
            if (res !== null) {
                return res;
            }
        } else if(Array.isArray(leftVal) && Array.isArray(rightVal)) {
            const res = isInOrder(leftVal, rightVal);
            if (res !== null) {
                return res;
            }
        }
    }
    return null;
}

export class Pair {

    public readonly isInOrder: boolean | null;
    constructor(
        public readonly id: number,
        public readonly left: PacketData,
        public readonly right: PacketData,
    ) {
        this.isInOrder = isInOrder(left, right);
    }
}