# CRS Rust WASM Starter

## Introduction
This project is a template for starting Rust web assembly libraries.  
This project uses standard ES Modules thus no bundlers.

This project assumes you have rust installed and dependencies like wasm-pack.  
If you don't have wasm-pack installed you can do so using
```
cargo install wasm-pack
```

Remember to update the Cargo.toml file in the rust folder with your project details.

## Building the project
```
wasm-pack build --target web --out-dir ../bin
```

The package.json file contains a script that will build it for you, should you prefer that.

Performing a build using the above command will create a directory called bin in the root of the project.  
This directory contains all the required wasm and js files to use in index.html.

## Getting started links
https://rustwasm.github.io/wasm-bindgen/introduction.html  
https://rustwasm.github.io/docs/wasm-pack/  
https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm  

## Other links
https://github.com/rustwasm/wasm-bindgen  
https://crates.io/  
https://rustwasm.github.io/wasm-bindgen/api/web_sys/   
https://doc.rust-lang.org/book/title-page.html  


## Examples
https://github.com/rustwasm/wasm-bindgen/tree/master/examples
