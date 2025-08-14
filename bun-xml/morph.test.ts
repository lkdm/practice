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
    test("Should flatten when numeric contiguous keys start from 1", () => {
      const result = fn(data);

      // Compile-time check
      expectTypeOf(result).toBeArray;

      // Runtime checks
      expect(Array.isArray(result)).toBe(true);
    });

    test("Noop because non-contiguous number key", () => {
      const input = { ...data, "5": true };
      const result = fn(input);

      expectTypeOf(result).toMatchTypeOf<Record<string, unknown>>();
      expectTypeOf(result).not.toBeArray();

      expect(Array.isArray(result)).toBe(false);
      expect(result).toEqual(input);
    });

    test("Noop because non-numeric key", () => {
      const input = { ...data, a: true };
      const result = fn(input);

      expectTypeOf(result).toMatchTypeOf<Record<string, unknown>>();
      expectTypeOf(result).not.toBeArray();

      expect(Array.isArray(result)).toBe(false);
      expect(result).toEqual(input);
    });

    test("Should flatten recursively", () => {
      const input = { grandparent: { parent: data } };
      const result = fn(input);

      expectTypeOf(result).toBeArray;
      expect(Array.isArray(result)).toBe(true);

      // Verify nested flattening actually happened
      expect(result.parent).toEqual([
        [{ test: "A" }, { test: "B" }, { test: "C" }],
      ]);
    });
  });
});
