## No Database version

This iteration does not use any database (`nodb`) and it's storing the courses in a vector inside the application state (implemented in `AppState`).

### Usage examples

There are three API operations implemented, tested (using automated tests, part of unit tests).

- Do the basic check using `curl localhost:3000/health`<br/>
  This should return "iam_ok" and a counter (number) of times it has been called.

- Submit some courses using:

  - `curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"tutor_id":1, "name":"First course"}'`
  - `curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"tutor_id":1, "name":"Second course"}'`

- Get all the courses of a tutor (with id `1` in this case) using `curl localhost:3000/courses/1`

- Get the details of a course using `curl localhost:3000/courses/1/1`
