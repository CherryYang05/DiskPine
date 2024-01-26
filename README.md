## 运行方式

使用前需要安装 Rust 环境，[在 Linux 上安装 rustup](https://www.rust-lang.org/tools/install)

```rust
cargo run [cache_size] [trace_size]
```

其中：

- trace_size 是要生成的随机请求的总数据量大小，**单位 TB**

- cache_size 是缓存大小，也是提前要填充的数据量大小，**单位 GB**

其余具体说明见代码中的注释。