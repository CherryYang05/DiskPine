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
      --rw <RW>                        指定生成的请求类型 [支持参数为 r, w, rw]
      --ro <read_order>                指定读操作的顺序性(该参数当 rw 包含 r 有效) [可选参数为 rand(默认), seq]
      --wo <write_order>               指定写操作的顺序性(该参数当 rw 包含 w 有效) [可选参数为 rand(默认), seq]
      --woff <write_offset>            若只有读操作，指定已经写的数据地址(默认值: 0)
      --wsize <wsize>                  单个写请求大小范围，若没有写请求则设置为 0-0
      --rsize <rsize>                  单个读请求大小范围，若没有读请求则设置为 0-0
      --rwsize <RWSIZE>                单个请求大小范围(该参数当 wsize 和 rsize 均为 None 时有效)
      --batch <BATCH>                  设置读写操作的 batch [支持参数为 r, w, rw]
      --batch_IOw_num <batch_IOw_num>  每个 write batch 的大小范围(单位为 blk_size，该参数当 batch 包含 'w' 时有效)
      --batch_IOr_num <batch_IOr_num>  每个 read batch 的大小范围(单位为 blk_size，该参数当 batch 包含 'r' 时有效)
      --time_dist <time_dist>          生成的时间间隔满足的数学分布[支持的参数：exp:lambda(指数分布:lambda)，uni(均匀分布)，poi(泊松分布:lambda)]
      --req_dist <req_dist>            生成的请求大小满足的数学分布[支持的参数：exp:lambda(指数分布:lambda)，uni(均匀分布)，poi:lambda(泊松分布:lambda)]
  -h, --help                           Print help
```

==**[重要]**== 下面是对每个参数的详细说明

- size: 指定生成 trace 的请求总大小；

- blk_size: 指定生成请求的块大小，即粒度（建议为 512B 的 2 的幂次倍）；

- rw: 读写标志，表明生成的请求的读写操作，可选参数为 [r, w, rw]；

- woff: write_offset，指定已经顺序写入的数据的地址，在随机读和顺序写操作中需指定该参数，该参数单位可以为 KB，MB，GB，TB，不加单位默认为 B，不区分大小写；

- ro: read_order，指定读操作是随机读还是顺序读。如果是随机读，那么在 [0, woff] 区间内随机生成读请求的偏移量，直到数据量达到 size；如果是顺序读，偏移量最大到 size 表示的大小，即生成地址为 [0, size] 区间内的顺序读请求。**当该参数为 rand 时，woff 参数必须指定一个不为 0 的值。** 可选参数为 [rand, seq]；

- wo: write_order，指定写操作是随机写还是顺序写。因为是磁带操作，所以这里固定为顺序写，顺序写的起始偏移地址由 woff 指定，若不指定则该参数默认为 0。可选参数为 [rand, seq]

- wsize: 设置每个写请求的大小范围，单位是 blk_size 参数指定的值

- rsize: 设置每个读请求的大小范围，单位是 blk_size 参数指定的值

- rwsize: 设置每个读写请求的大小范围，单位是 blk_size 参数指定的值，只有当没有指定 wsize 和 rsize 时才需指定该参数；

- batch: 指定将不同小 IO 聚合成 batch 的请求类型，可选参数 [r, w, rw]。batch 含义即连续生成若干条相同读写类型的请求；

- batch_IOw_num: 设定每个 write batch 的大小范围(单位为 blk_size，该参数当 batch 包含 'w' 时有效)

- batch_IOr_num: 设定每个 read batch 的大小范围(单位为 blk_size，该参数当 batch 包含 'r' 时有效)

- time_dist: 指定请求间隔时间符合的数学分布，目前支持指数分布 exp(lambda)，均匀分布 uni，泊松分布 poi(lambda)，输入参数格式为 [exp:lambda，uni，poi:lambda]；

- req_dist: 指定请求大小符合的数学分布，目前支持指数分布 exp(lambda)，均匀分布 uni，泊松分布 poi(lambda)，输入参数格式为 [exp:lambda，uni，poi:lambda]。注意，若指定该参数，读写大小 wsize/rsize 参数将失效。


**使用样例**

**设置生成的请求为随机读操作，设定已经写到的数据地址为 230G 40-40960 个 blk_size 大小**

`cargo run --bin diskpine -- generate-tape-trace --size=10T --blk_size=256K --rw=r --ro=rand --woff=230G --rsize=40-40960`

**设置生成的请求为顺序读操作，请求大小符合 Exp(0.000002) 分布，请求的时间间隔符合 Exp(0.03) 分布：**
`cargo run --bin diskpine -- generate-tape-trace --size=10T --blk_size=256K --rw=r --ro=seq --req_dist=exp:0.000002 --time_dist=exp:0.03`

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

`./diskpine generate-tape-trace --size=10T --blk_size=256K --rw=r --ro=rand --woff=230G --rsize=40-40960`



