# wit-bindgen handles

This project goal is to build a system where the JS side can provide the implementation for some resource, and have it usable on the Rust side.

`delegate.wit` describes the JS-implemented resource. It is the used in `service.wit`.

To run the demo, run a http server in this directory, eg. with `python http.server` and access `http://localhost:8000/demo/index.html`. Then click on the `Run Test` button and look at the messages.

The `add-delegate()` function expects a `delegate` parameter, but there doesn't seem to be a way to create such an instance and "import" it into the WASM memory.
