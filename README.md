# Dashboard test app

The project has 3 main parts:

## Client
a CSR style Leptos WASM app

To setup Leptos environment, refer to the official instructions: https://book.leptos.dev/getting_started/index.html

run it:
```
cd client
trunk serve --port 3000 --open
```
this runs the `trunk` that builds and serves the Client on `localhost:3000`, which will try to open a websocket to connoct to the server that is supposed to be running on `localhost:5000

## Server
a websocket server that continuously sends server signals to the client over a websocket

```
cd server
cargo run
```
this will run the server on `localhost:5000`

## Shared
a lib with shared code between the client and server