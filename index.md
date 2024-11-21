# WASI

This is a series of four mini 15-minute presentations that can bootstrap your WASI programming.

### 1. Writing your first wasi app
  The ideal story would be that you could simply compile your existing code with the new build target 'wasm32-wasi' without modifying any of your code. But in reality...

  "Talk is cheap. Show me the code." - Linus Torvalds
  - [Hello World](./hello-world/README.md)
  - [Http Request - Wasi crate](./http_request_wasi/README.md)
  - [Http Request - Wit bindgen crate](./http_request_witbindgen/README.md)
  - [Http Request - Waki](./http_request_waki/README.md)
  - [Http service](./http_service/README.md)
  - Lisa
    - [With Waki](https://github.com/seungjin/lisa/tree/127c9576af00a47d12285e2f4e0f5e68ac61216d)
    - [With Wasi direct](https://github.com/seungjin/lisa/tree/9bbf03f0c75bf4bb1ac256a0994f655d471342c8)

  References:
  - `rustup target add wasm32-wasip2`
  - [Wit bindgen crate] (https://docs.rs/wit-bindgen-rt/latest/wit_bindgen_rt/)
  - [Wasi crate](https://docs.rs/wasi/0.13.3+wasi-0.2.2/wasi/index.html)
  - [Wasmtime](https://github.com/bytecodealliance/wasmtime)
  - [Cargo component](https://github.com/bytecodealliance/cargo-component)


### 2. How to run your wasi in Cloud(?).
  - Nginx Unit
  - Fermyon Spin

### 3. How can I talk with foreigners?
  - Component model, Why is everybody talking about it?
  - Bindgen
  - Wit
