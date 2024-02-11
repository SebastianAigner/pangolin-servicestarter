# Pangolin Service Starter 🦔

Pangolin is a tool that sits on a port and, when receiving an HTTP request, will start a process.

It's designed for off-by-default services that only need to boot up when you access them -- think, for example, your
checklist application or document manager.

Let's say you have a compute-intensive service that only needs to run when you access it. It's supposed to run on **port 3000**, so you start Pangolin to sit on the port:

```bash
pangolin 3000 node example-service.js
```

When you make the first HTTP request to 127.0.0.1:3000, Pangolin will temporarily redirect you to a holding page, and start the application (freeing up the port). Afterwards, Pangolin will forward you to the started application.

Pangolin doesn't act as a proxy for any of the requests to the actual service, making it **zero-overhead** once the application is started.

Because Pangolin doesn't start all applications eagerly, you can use it to have more services deployed on low-end machines (e.g. a Raspberry Pi), even if all running services together would exceed the resources available on the system (like RAM or CPU).

## TODO

- [ ] (optional) Health-check the service. If the service doesn't require liveness anymore, ask it to shut down
  gracefully.
- [ ] (optional) When available, bind the port again.