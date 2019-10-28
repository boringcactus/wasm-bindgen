# Bin Crate Without a Bundler

This example shows how to build a bin crate that can be run both natively
and for WebAssembly, using conditional compilation for low-level implementation
details.

You can build the example locally with:

```
$ wasm-pack build --target web --bin
```

and then opening `index.html` in a browser should run the example!
