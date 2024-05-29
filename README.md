## 1. HMSim 模拟器集成命令行工具

使用前需要安装 Rust 环境。[在 Linux 上安装 Rust 环境](https://www.rust-lang.org/tools/install)

### 2. 支持的功能

目前集成命令行工具支持三个命令：

1. trace-foot-size：计算 trace 的数据量和落盘量

2. origin-to-sim：将微软原始 trace 格式转化为 HMSim 格式的 trace

3. genereate-tape-trace：生成适用于 tape 的 trace，支持若干参数

### 3. 使用方式

在 `diskpine` 下执行：

```shell
cargo run --bin diskpine -- [Command]
```

第一次执行需要下载需要的库文件，可能较慢。

可以运行 `cargo run --bin diskpine -- --help` 查看目前支持的命令。

#### 3.1 trace-foot-size 命令

功能：计算 trace 的数据量和落盘量

查看 `help`：

Shell Command:

`cargo run --bin diskpine -- trace-foot-size --help`

Output:

```shell
计算 trace 数据量及落盘量

Usage: diskpine trace-foot-size --file <FILE>

Options:
  -f, --file <FILE>  trace 文件名
  -h, --help         Print help
```

一个使用样例为：

`cargo run --bin diskpine -- trace-foot-size -f tape.trace`

#### 3.2 origin-to-sim 命令

功能：将微软原始 trace 格式转化为 HMSim 格式的 trace

查看 `help`：

Shell Command:

`cargo run --bin diskpine -- origin-to-sim --help`

Output:

```shell
将微软原始 trace 格式转化为 HMSim 格式的 trace，修改后的文件与其同名

Usage: diskpine origin-to-sim [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>  原始 trace 文件名
  -t, --timestamp    是否保留时间戳
  -h, --help         Print help
```

一个使用样例为：

`cargo run --bin diskpine -- origin-to-sim -f tape.csv -t`

#### 3.3 genereate-tape-trace 命令

功能：生成适用于 tape 的 trace，支持若干参数

查看 `help`：

Shell Command:

`cargo run --bin diskpine -- generate-tape-trace --help`

Output:

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

`cargo run --bin diskpine -- generate-tape-trace --size=1T --blk_size=256K --rsize=10-100 --wsize=100-100000 --batch=rw --batch_IOw_num=5-15 --batch_IOr_num=1-3`

