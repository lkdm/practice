import { _ } from "lodash";

/**
 * If the dictionary keys are numbered, contiguous strings, we can turn it into an array
 */
export const flattenCollectionDictsRecursively = (input: any): any => {
  if (typeof input !== "object" || input === null) return input;

  const keys = _.keys(input);
  const numericKeys = keys
    .filter((k) => !isNaN(k))
    .map(Number)
    .sort((a, b) => a - b);

  const shouldBeArray =
    numericKeys.length === keys.length && // all keys are numeric
    numericKeys[0] === 0 && // start from 0
    numericKeys.every((val, idx) => val === idx); // contiguous keys

  // console.debug("Input keys:", keys);
  // console.debug("Numeric keys:", numericKeys);
  // console.debug("Should be array:", shouldBeArray);

  if (shouldBeArray) {
    return numericKeys.map((key) => {
      return flattenCollectionDictsRecursively(input[key]);
    });
  } else {
    const result: Record<string, any> = {};
    for (const key of keys) {
      result[key] = flattenCollectionDictsRecursively(input[key]);
    }
    return result;
  }
};
