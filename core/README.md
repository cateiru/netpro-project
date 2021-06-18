# Rust RepoSync Core

## Build

- macOS(x86)の場合

    `.cargo/config`

    ```toml
    [target.x86_64-apple-darwin]
    rustflags = [
    "-C", "link-arg=-undefined",
    "-C", "link-arg=dynamic_lookup",
    ]

    [target.aarch64-apple-darwin]
    rustflags = [
    "-C", "link-arg=-undefined",
    "-C", "link-arg=dynamic_lookup",
    ]
    ```

```bash
cargo build --release
```

## 参考

- https://haripo.com/articles/2019/pyo3/
