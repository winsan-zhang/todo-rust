# todo-rust

## 介绍
用rust写的todo web应用（webassembly）  
后端：rust + socket + postgresql + diesel
前端：rust + wasm-bindgen +seed
### Compiles for development
```
cargo make
```
后端：
```
cargo run -p backend --bin backend
```
前端：
```
cd frontend
cargo cargo install microserver
microserver
```
访问地址：http://localhost:9090


