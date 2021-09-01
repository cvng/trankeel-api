export {
  array,
  boolean,
  date,
  mixed,
  number,
  object,
  setLocale,
  string,
  ValidationError,
} from "yup";

export class AssertionError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "AssertionError";
  }
}

export function unreachable(): never {
  throw new AssertionError("unreachable");
}
