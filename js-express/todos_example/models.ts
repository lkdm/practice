import { Database } from "bun:sqlite";

/**
 * Common properties shared by all entities in the database
 */
interface BaseEntity {
  ID: string;
  created_at: string;
  modified_at?: string;
  deleted_at?: string;
}

/**
 * Contains editable properties
 */
interface TodoModel {
  name: string;
}

/**
 * Returned from model functions
 */
interface TodoEntity extends BaseEntity, TodoModel {}

export const initialiseDatabase = () => {
  const db = new Database(":memory:");

  try {
    db.run(`
		CREATE TABLE IF NOT EXISTS Todos (
			ID INTEGER PRIMARY KEY AUTOINCREMENT,
			name TEXT NOT NULL,
			created_at INTEGER DEFAULT (strftime('%s', 'now')),
			deleted_at INTEGER DEFAULT NULL,
			modified_at INTEGER DEFAULT NULL
		)
`);
    console.log("Database migrated succesfully");
  } catch (error) {
    console.error("Migration failed: ", error);
    throw error;
  }
  return db;
};

export const testDatabase = (db: Database) => {
  const query = db.query("SELECT 1 as status;");
  return query.get();
};

export const getTodos = (db: Database) =>
  db
    .query(
      `
SELECT ID, name, created_at, modified_at FROM Todos WHERE deleted_at IS NULL OR deleted_at > strftime('%s', 'now');`,
    )
    .all();

export const addTodo = (db: Database, dto: TodoModel) =>
  db
    .query(
      `
	INSERT INTO Todos (name)
	VALUES ($name)
	RETURNING ID;
`,
    )
    .all({ $name: dto.name });

export const editTodo = (db: Database, id: string, dto: TodoModel) =>
  db
    .query(
      `
	UPDATE Todos
	SET name = $name,
		modified_at = strftime('%s', 'now')
	WHERE ID = $id;
`,
    )
    .run({ $id: id, $name: dto.name });

export const deleteTodo = (db: Database, id: string) =>
  db
    .query(
      `
	UPDATE Todos
	SET deleted_at = strftime('%s', 'now')
	WHERE ID = $id;
`,
    )
    .run({ $id: id });
