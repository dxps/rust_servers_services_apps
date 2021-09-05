# EzyTutors WebApp Server-Side Rendered UI

## Usage

These iterations show how to rendered different elements in HTML using Tera template engine.

- iter 1

  - It shows how to render a page based on a template, including some context params, plus serving static content.

- iter 2

  - It shows a form and the submitted items are received and rendered in a second page.

- iter 3

  - It shows how to display a dynamic list in a template rendered page.
  - Run it and access http://localhost:8080/tutors

- iter 4

  - It shows how to fetch the tutors from an external source and then render the dynamic list in the page.
  - First, go to `tutor-web-service` and run `./run_iter5.sh`
  - Next, in this `tutor-web-app-ssr` project, run `./run_iter4.sh`
  - Go to http://localhost:8080/tutors to see the result.

- iter 5
  - It shows the registration form as the langing page (accessing the '/' path).
  - Registration (of both user and tutor data) submit creates the user in this tutor-web-app-ssr (in its database) and the tutor in the tutor-web-service, called by this back-end handler.
