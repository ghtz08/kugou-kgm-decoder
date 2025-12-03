# 酷狗混淆的歌曲文件的解码器

[![Badge](https://img.shields.io/badge/link-996.icu-%23FF4D5B.svg?style=flat-square)](https://996.icu)
[![LICENSE](https://img.shields.io/badge/license-Anti%20996-blue.svg?style=flat-square)](/LICENSE)
[![Lang](https://img.shields.io/badge/lang-rust-brightgreen?style=flat-square)](https://www.rust-lang.org)
![Repo Size](https://img.shields.io/github/repo-size/ghtz08/kuguo-kgm-decoder?style=flat-square)
![Code Size](https://img.shields.io/github/languages/code-size/ghtz08/kuguo-kgm-decoder?style=flat-square)

## 介绍

一个命令行工具，可以用来解码酷狗缓存歌曲文件和下载的单曲收费歌曲文件。

功能是 [um] 的一个子集。

[um]: https://git.um-react.app/um/cli

## 使用方式

> 使用命令 `cargo build --release` 构建或者直接下载发布的二进制文件。

- 针对单个文件

```bash
kgm-decoder <文件名>
```

- 针对某个目录下的文件（不包括子目录）

```bash
kgm-decoder <目录名>
```

- 针对某个目录下的文件（包括子目录）

```bash
kgm-decoder -r <目录名>
```

### 其它参数

| 参数 | 解释 |
| :--- | :--- |
| -k, --keep-file | 保留原文件 |

> 生成的后缀为 .mp3 的文件不一定是 mp3 格式的文件, 这是在无法判断文件类型时使用的默认后缀。

## Star 记录

[![Star History Chart](https://api.star-history.com/svg?repos=ghtz08/kugou-kgm-decoder&type=Date)](https://www.star-history.com/#ghtz08/kugou-kgm-decoder&Date)

## 许可证

[反 996 许可证版本 1.0](/LICENSE)
