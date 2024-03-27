## HMSim 模拟器集成命令行工具

使用前需要安装 Rust 环境。[在 Linux 上安装 Rust 环境](https://www.rust-lang.org/tools/install)

### 支持的功能

目前集成命令行工具支持三个命令：

1. trace-foot-size：计算 trace 的数据量和落盘量

2. origin-to-sim：将微软原始 trace 格式转化为 HMSim 格式的 trace

3. genereate-tape-trace：生成适用于 Tape 的 trace，支持若干参数

### 使用方式

```shell
cargo run --bin diskpine -- [Command]
```

可以运行 `cargo run --bin diskpine -- --help` 查看目前支持的命令。

以其中的 `generate-tape-trace` 命令为例，查看 `help`：

```shell
Usage: diskpine generate-tape-trace [OPTIONS] --size <size> --rwrate <RWRATE>

Options:
      --size <size>                读写操作的总大小
      --rwrate <RWRATE>            读写比例(读:写)
      --wsize <wsize>              写请求大小范围
      --rsize <rsize>              读请求大小范围
      --rwsize <RWSIZE>            请求大小范围(该参数当 wsize 和 rsize 均为 None 时有效)
      --batch <BATCH>              设置读写操作的 batch，支持参数为 r, w, rw
      --batch-wsize <batch-wsize>  每个 write batch 的大小范围(该参数当 batch 有值时有效)
      --batch-rsize <batch-rsize>  每个 read batch 的大小范围(该参数当 batch 有值时有效)
  -h, --help                       Print help
```

一个使用样例为：

`cargo run --bin diskpine -- generate-tape-trace --size=1T --rwrate=1:2.5 --rsize=12M-1G --wsize=1G-62G --batch=r --batch-rsize=1G-8G`

