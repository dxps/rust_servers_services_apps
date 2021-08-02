## No Database version

This iteration does not use any database (`nodb`) and it's storing the courses in a vector inside the application state (implemented in `AppState`).

<br/>

### Run

Use `./run_tutor-nodb-svc.sh` (from the parent directory) to run the service.<br/>
The other one - `basic-server` - is just there as a minimal example of using Actix.

<br/>

### Usage examples

There are three API operations implemented, tested (using automated tests, part of unit tests).

- Do the basic check using `curl localhost:3000/health`<br/>
  This should return "iam_ok" and a counter (number) of times it has been called.

- Submit some courses using:

  - `curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"tutor_id":1, "name":"First course"}'`
  - `curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"tutor_id":1, "name":"Second course"}'`

- Get all the courses of a tutor (with id `1` in this case) using `curl localhost:3000/courses/1`

- Get the details of a course using `curl localhost:3000/courses/1/1`

You can use Thunder Client (VSCode extension), having such requests already stored in `.local/thunder-tests` directory in the project root.
