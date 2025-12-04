import * as borsh from "borsh";
export class numberMath {
  no = 0;
  constructor({ no }: { no: number }) {
    this.no = no;
  }
}

export const schema = {
  struct: {
    no: "u32",
  },
};

export const GREETING_SIZE = borsh.serialize(
  schema,
  new numberMath({ no: 0 }),
).length;
