## How this project created
```console
# cargo component new --bin hello-world
    Creating binary (application) `hello-world` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
     Updated manifest of package `hello-world`
   Generated source file `src/main.rs`
suthep2:~/Works/presentations/wasi-2024-11-21(:|✔)
# cd hello-world
suthep2:~/Works/presentations/wasi-2024-11-21/hello-world(main|…)
# cargo component build --target wasm32-wasip2
   Compiling wit-bindgen-rt v0.35.0
   Compiling bitflags v2.6.0
   Compiling hello-world v0.1.0 (/var/home/seungjin/Works/presentations/wasi-2024-11-21/hello-world)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.89s
    Creating component target/wasm32-wasip1/debug/hello-world.wasm
suthep2:~/Works/presentations/wasi-2024-11-21/hello-world(main|…)
# wasmtime target/wasm32-wasip2/debug/hello-world.wasm
Hello, world!
```

## Check the code:
[src/main.rs](src/main.rs)
