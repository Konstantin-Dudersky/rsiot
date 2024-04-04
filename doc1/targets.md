Ограничение зависимостей 

```toml
[target.'cfg(any(target_arch = "x86_64", target_arch = "aarch64"))'.dependencies]
[target.'cfg(any(target_arch = "wasm32"))'.dependencies]
```

```rust
#![cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#![cfg(any(target_arch = "wasm32"))]
```

```rust
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
```
