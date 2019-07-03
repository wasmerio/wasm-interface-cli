# Wasm Interface CLI

A command line to check Wasm modules against a set of [interfaces][wasm-interface-lib]

To use it, just run:

```
wasm-interface wasi-module.wasm -i example_interfaces/wasi_unstable.wasm_interface
```

You can install it with `cargo install wasm-interface`


[wasm-interface-lib]: https://github.com/wasmerio/wapm-cli/tree/master/lib/wasm-interface
