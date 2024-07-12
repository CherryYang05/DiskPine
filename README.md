# HMSim 模拟器集成命令行工具

使用前需要安装 Rust 环境。[在 Linux 上安装 Rust 环境](https://www.rust-lang.org/tools/install)

## 1. 支持的功能

目前集成命令行工具支持三个命令：

1. trace-foot-size：计算 trace 的数据量和落盘量

2. origin-to-sim：将微软原始 trace 格式转化为 HMSim 格式的 trace

3. genereate-tape-trace：生成适用于 tape 的 trace，支持若干参数

## 2. 使用方式

### 2.1 用 cargo run 执行

在目录 `diskpine` 下执行（注意 `diskpine` 和 `Command` 之间有两个短横杠 `--`）

```shell
cargo run --bin diskpine -- [Command]
```

第一次执行需要下载需要的库文件，可能较慢。

可以运行 `cargo run --bin diskpine -- --help` 查看目前支持的命令。


>以下命令请复制粘贴运行

#### 2.1.1 trace-foot-size 命令

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

#### 2.1.2 origin-to-sim 命令

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

#### 2.1.3 genereate-tape-trace 命令

功能：生成适用于 tape 的 trace，支持若干参数

查看 `help`：

Shell Command:

`cargo run --bin diskpine -- generate-tape-trace --help`

Output:

```shell
Usage: diskpine generate-tape-trace [OPTIONS] --size <size> --blk_size <blk_size>

Options:
      --size <size>                    读写操作的总大小
      --blk_size <blk_size>            生成的请求粒度，即块大小
      --wsize <wsize>                  单个写请求大小范围，若没有写请求则设置为 0-0
      --rsize <rsize>                  单个读请求大小范围，若没有读请求则设置为 0-0
      --rwsize <RWSIZE>                单个请求大小范围(该参数当 wsize 和 rsize 均为 None 时有效)
      --batch <BATCH>                  设置读写操作的 batch，支持参数为 r, w, rw
      --batch_IOw_num <batch_IOw_num>  每个 write batch 的大小范围(单位为 blk_size，该参数当 batch 包含 'w' 时有效)
      --batch_IOr_num <batch_IOr_num>  每个 read batch 的大小范围(单位为 blk_size，该参数当 batch 包含 'r' 时有效)
      --dist <dist>                    生成的时间间隔满足的数学分布[支持的参数：exp:lambda(指数分布:lambda)，uni(均匀分布)，poi(泊松分布:lambda)]
  -h, --help                           Print help
```

一个使用样例为：

`cargo run --bin diskpine -- generate-tape-trace --size=10T --blk_size=256K --rsize=40-40960 --wsize=0-0`

**设置生成的请求大小符合 Exp(0.0000001) 分布，请求的时间间隔符合 Exp(0.1)：**
`cargo run --bin diskpine -- generate-tape-trace --size=10T --blk_size=256K --rsize=40-40960 --wsize=0-0 --req_dist=exp:0.0000001 --time_dist=exp:0.1`

### 2.2 用二进制文件执行

用二进制文件执行命令与用 `cargo` 略有不同，具体如下

```shell
./diskpine [Command]
```

### 2.2.1 trace-foot-size
以 `2.1.1` 的子命令为例，给出一个命令样例：

`./diskpine trace-foot-size -f tape.trace`

### 2.2.2 origin-to-sim
以 `2.1.2` 的子命令为例，给出一个命令样例：
`cargo run --bin diskpine -- origin-to-sim -f tape.csv -t`

### 2.2.3 genereate-tape-trace
以 `2.1.3` 的子命令为例，给出一个命令样例：

`./diskpine generate-tape-trace --size=10T --blk_size=256K --rsize=40-40960 --wsize=0-0`



