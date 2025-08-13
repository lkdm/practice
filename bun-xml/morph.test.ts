import { describe, expect, expectTypeOf, test } from "bun:test";
import { flattenCollectionDicts } from "./morph";

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
	test("ok", () => {
		expectTypeOf(fn(data)).toBeArray;
	});
	test("non-contiguous number key", () => {
		expectTypeOf(fn({ data, "5": true })).toMatchTypeOf<
			Record<string, unknown>
		>();
		expectTypeOf(fn({ data, "5": true })).not.toBeArray();
	});
	test("non-numeric key", () => {
		expectTypeOf(fn({ data, a: true })).toMatchTypeOf<
			Record<string, unknown>
		>();
		expectTypeOf(fn({ data, a: true })).not.toBeArray();
	});
});
