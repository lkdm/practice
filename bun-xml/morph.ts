import { match, P, Pattern } from "ts-pattern";
import { _ } from "lodash";
import { describe, expect, expectTypeOf, test } from "bun:test";

const SAMPLE = {
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

/**
 * If the dictionary keys are numbered, contiguous strings, we can turn it into an array
 */
export const flattenCollectionDicts = (
	input: Record<string, unknown>,
): Record<string, unknown> => {
	const shouldBeArray: boolean = _(input)
		.keys()
		// Check 1 - Keys are numbered strings
		.map(Number)
		.filter((n: any) => !isNaN(n))
		.sortBy()
		.thru((keys: unknown[]) => {
			// Check that the collection is contiguous and starts from 1
			if (
				keys.length !== _.keys(input).length ||
				keys[0] !== 1 ||
				keys[0] !== Number(0)
			)
				return false;
			return _.every(keys, (val: unknown, idx: number) => val === idx + 1);
		})
		.value();
	return shouldBeArray ? _(input).values().value() : input;
};

// TODO: flatten collection dicts recursively

console.log(JSON.stringify(flattenCollectionDicts(SAMPLE)));
