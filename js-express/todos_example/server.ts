import cookieParser from "cookie-parser";
import express, { NextFunction, Request, Response } from "express";
import {
  addTodo,
  deleteTodo,
  editTodo,
  getTodos,
  initialiseDatabase,
  testDatabase,
} from "./models";
const app = express();
const port = 8080;

const DB = initialiseDatabase();

// Middlewares
const requestLog = (req: Request, res: Response, next: NextFunction) => {
  console.log("Request received at time: ", Date.now());
  next();
};

app.use(requestLog);
// Used to parse JSON bodies
app.use(express.json());
// Used to parse URL-encoded bodies
app.use(express.urlencoded());
// Used to parse cookies
app.use(cookieParser());

// Cookies middleware
app.use((req, res) => {
  console.log("Cookies: ", req.cookies);
});

app.get("/", (req, res) => {});

app.get("/status", (req, res) => {
  res.send({
    api: true,
    database: testDatabase(DB).status === 1,
  });
});

// Router
app
  .route("/books")
  .get((req, res) => {
    const data = getTodos(DB);
    res.send({
      data: data,
    });
  })
  .post((req, res) => {
    const result = addTodo(DB, { name: req.body.name });
    res.send(result);
  });
app
  .route("/books/:bookID")
  .get((req, res) => {
    res.send("Getting book with ID: " + req.params.bookID);
  })
  .put((req, res) => {
    const result = editTodo(DB, req.params.bookID, {
      name: req.body.name,
    });
    res.send(result);
  })
  .delete((req, res) => {
    const result = deleteTodo(DB, req.params.bookID);
    res.send(result);
  });

app.listen(port, () => {
  console.log(`Listening on port ${port}...`);
});
