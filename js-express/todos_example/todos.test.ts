import { expect, test } from "bun:test";
import { getTodos, initialiseDatabase } from "./models";

const DB = initialiseDatabase();

test("Test listing Todos", () => {
  const todos = getTodos(DB);
  console.log(typeof todos, todos);
  expect(todos).toEqual([]);
});
