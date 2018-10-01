# led-strip-server

A simple utility for controlling an RGB LED strip connected to a Raspberry PI using three MOSFETs.

The server will listen on a websocket for new connections.

JSON structs like this one will be accepted: `{ "r": 255, "g": 128, "b": 0 }`

The server will broadcast changes to the currently configured color to all connected clients.

## Usage

Simple start the built binary:

```
led-strip-server start
```

## Building

Make sure a toolchain for ARMv7 is installed and build the project as usual with cargo:

```
cargo build --target armv7-unknown-linux-gnueabihf --release
```

## Contributors

 - Frederick Gnodtke
