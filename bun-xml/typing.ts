const OkType = Symbol("Ok");
const ErrType = Symbol("Err");

type Ok<T extends object> = T & { readonly [OkType]: "OK" };
type Err<E = unknown> = { error: E; readonly [ErrType]: "ERR" };
type Result<T extends object, E = unknown> = Ok<T> | Err<E>;

const ok = <T extends object>(value: T): Ok<T> =>
  Object.assign(value, { [OkType]: "OK" }) as Ok<T>;
const err = <E = unknown>(error: E): Err<E> =>
  Object.assign({ error }, { [ErrType]: "ERR" }) as Err<E>;

const isOk = <T extends object, E>(result: Result<T, E>): boolean =>
  OkType in result;
const isErr = <T extends object, E>(result: Result<T, E>): boolean =>
  ErrType in result;

console.log(isOk(ok({})));
console.log(isErr(err({})));

const SomeType = Symbol("SomeType");
const NoneType = Symbol("NoneType");

interface Some<T> {
  readonly [SomeType]: true;
  readonly value: T;
}

interface None {
  readonly [NoneType]: true;
}

type Option<T> = Some<T> | None;

const some = <T>(value: T): Some<T> => ({ [SomeType]: true, value });
const none = (): None => ({ [NoneType]: true });

const isSome = <T>(option: Option<T>): boolean => SomeType in option;

const test = (inner: number, isOk: boolean): Option<number> => {
  return isOk ? some(inner) : none();
};

const testOk = test(382, true);
const testNoOk = test(1_000_000, false);

console.log(isSome(testOk) ? "passed" : "did not pass");
console.log(isSome(testNoOk) ? "did not pass" : "passed");

// type Result<T, E = unknown> =
//
//
// /**
//  * Result monad
//  */
// export type Result<T, E = unknown> = Readonly<Ok<T>> | Readonly<Err<E>>;
//
// /**
//  * Create an okay type
//  */
// const ok = <T>(value: T): { ok: true; value: T } => ({ ok: true, value });
//
// /**
//  * Create an Error type
//  */
// const err = <E>(error: E): { ok: false; error: E } => ({ ok: false, error });
// type Err<E> = ReturnType<typeof err<E>>;
// type Ok<T> = ReturnType<typeof ok<T>>;
// const isOk = <T, E>(result: Result<T, E>): result is Ok<T> => result.ok;
// const isErr = <T, E>(result: Result<T, E>): result is Err<E> => !result.ok;
