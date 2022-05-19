# Sample application to showcase log / tracing ID correlation issue

It is currently not possible to have the TraceId/SpanID in the logs AND in Jaeger easily in axum/tokio/tower based applications.  

The application in this repo can be used by experienced rust developers to figure out an appropriate solution without having the effort to bootstrap a new application.

## Starting / using the application

### Starting jaeger
The jaeger all-in-one docker container can be started with the `./start_jaeger.sh`. The container is automatically deleted when pressing `Ctrl` + `c` / `Cmd` + `c`.

The UI is accessible at [http://localhost:16686](http://localhost:16686).

### Starting the application
The application can be started by executing `cargo run` .
The application logs all requests on Debug level by default.

### Sending requests
Http requests can be sent using the `./call_service.sh` script (uses curl).

## License
This project is licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT]).