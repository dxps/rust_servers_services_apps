# Scenario 1

This scenario illustrates basic client server development setup.

## Workspace

`scenario1` is a workspace that contains 4 projects:

| name         | project type | description/contains                          |
| ------------ | ------------ | --------------------------------------------- |
| `tcpserver`  | binary       | TCP server code                               |
| `tcpclient`  | binary       | TCP client code                               |
| `http`       | library      | HTTP protocol functionality                   |
| `httpserver` | binary       | HTTP server code that uses the `http` library |

## Usage

For `httpserver`, do `./run_httpserver.sh` to start it and then:

- Get the homepage (content of `public/index.html` file) using:<br/>
  `curl localhost:3000` or `curl localhost:3000/index.html`<br/>
  This also serves a referred CSS file using the same `StaticPageHandler`.
- Get some health page (content of `public/health.html` file) using:<br/>
  `curl localhost:3000/health.html`
- Call an API operation (that returns the content of `data/orders.json` file) using:<br/>
  `curl localhost:3000/api/shipping/orders`
