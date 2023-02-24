# `wasi-messaging-demo`

This repo. contains a demo. of [`wasi-messaging`](https://github.com/WebAssembly/wasi-messaging) in action.

## Repository Structure

- `src/`: contains an example guest implementation of the interface.
- `host/`: contains an example host implementation of the interface.

## Run

To run the demo.:

```sh
make build
make componentize
make run
```
