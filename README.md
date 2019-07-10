# Wasm Interface CLI

A command line tool to check Wasm modules against a set of [interfaces][wasm-interface-lib]

To use it, just run:

```
wasm-interface --dir=. check wasi-module.wasm -i example_interfaces/wasi_unstable.wai
```

## Install

You can install wasm-interface by running:

```
wapm install -g wasm-interface
```

*Note: You can also alternatively install via cargo with `cargo install wasm-interface-cli`


[wasm-interface-lib]: https://github.com/wasmerio/wapm-cli/tree/master/lib/wasm-interface
