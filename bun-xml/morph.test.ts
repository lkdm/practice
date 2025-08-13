import { describe, expect, expectTypeOf, test } from "bun:test";
import { flattenCollectionDicts } from "./morph";

describe("XML->JSON Parser", () => {
  describe("flattenCollectionDicts", () => {
    const fn = flattenCollectionDicts;
    const data = {
      "2": {
        test: "B",
      },
      "3": {
        test: "C",
      },
      "1": {
        test: "A",
      },
    };
    test("Should flatten", () => {
      // If the input dictionary's keys are numeric strings, contiguous, and start from 0 or 1; then it should be collapsed into an array.
      expectTypeOf(fn(data)).toBeArray;
    });
    test("Noop because non-contiguous number key", () => {
      // If the input dictionary's keys are non-contiguous, then it should retain its original shape
      expectTypeOf(fn({ data, "5": true })).toMatchTypeOf<
        Record<string, unknown>
      >();
      expectTypeOf(fn({ data, "5": true })).not.toBeArray();
    });
    test("Noop because non-numeric key", () => {
      // If any of the input dictionary's keys are non-numeric, it should retain its original shape
      expectTypeOf(fn({ data, a: true })).toMatchTypeOf<
        Record<string, unknown>
      >();
      expectTypeOf(fn({ data, a: true })).not.toBeArray();
    });
  });
});
