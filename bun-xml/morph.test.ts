import { describe, expect, expectTypeOf, test } from "bun:test";
import { flattenCollectionDictsRecursively } from "./morph";

describe("XML->JSON Parser", () => {
  describe("flattenCollectionDicts", () => {
    const fn = flattenCollectionDictsRecursively;
    const data = {
      "1": {
        test: "B",
      },
      "2": {
        test: "C",
      },
      "0": {
        test: "A",
      },
    };
    test("Should flatten when numeric contiguous keys start from 0", () => {
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

    test("Should flatten recursively (simple)", () => {
      const input = { grandparent: { parent: data } };
      const result = fn(input);

      expectTypeOf(result).not.toBeArray;
      expect(typeof result).toBe("object");

      // 'grandparent' remains, but 'parent' is converted to array
      expect(Array.isArray(result.grandparent.parent)).toBe(true);

      // Verify nested flattening actually happened
      expect(result.grandparent.parent).toEqual([
        { test: "A" },
        { test: "B" },
        { test: "C" },
      ]);
    });

    test("Should flatten recursively with multiple nested numeric-keyed dicts", () => {
      const input = {
        "0": data,
        "1": data,
        "2": { ...data, a: false },
      };

      const result = fn(input);

      expect(Array.isArray(result)).toBe(true);

      expect(Array.isArray(result[0])).toBe(true);
      expect(Array.isArray(result[1])).toBe(true);

      // Key "3" has an extra non-numeric key 'a', so expect normal object
      expect(typeof result[2]).toBe("object");
      expect(result[2].a).toBe(false);

      // Validate contents converted to arrays and order preserved for "1" and "2"
      expect(result[0]).toEqual([{ test: "A" }, { test: "B" }, { test: "C" }]);
      expect(result[1]).toEqual([{ test: "A" }, { test: "B" }, { test: "C" }]);

      // Contents of "3" should match input but with numeric key dicts flattened
      expect(result[2]["0"]).toEqual({ test: "A" });
      expect(result[2]["1"]).toEqual({ test: "B" });
      expect(result[2]["2"]).toEqual({ test: "C" });
    });
  });
});
