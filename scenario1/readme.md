## Scenario 1

### Workspace

Scenario1's workspace contains 4 projects.

```
   .-----------.     .------------.
   | tpcclient |     | httpserver |
   |  (proj1)  |     |  (proj4)   |
   '-----------'     '------------'

   .-----------.     .------------.
   | tpcserver |     | http (lib) |
   |  (proj2)  |     |  (proj3)   |
   '-----------'     '------------'
```
where:

| name          | project type     | description/contains         |
| --------------|------------------|------------------------------|
| `tcpserver`   | binary           | TCP server code              |
| `tcpclient`   | binary           | TCP client code              |
| `httpserver`  | binary           | HTTP server code             |
| `http`        | library          | HTTP protocol functionality  |

