# 酷狗混淆的歌曲文件的解码器

[![Badge](https://img.shields.io/badge/link-996.icu-%23FF4D5B.svg?style=flat-square)](https://996.icu)
[![LICENSE](https://img.shields.io/badge/license-Anti%20996-blue.svg?style=flat-square)](/LICENSE)
[![Lang](https://img.shields.io/badge/lang-rust-brightgreen)](https://www.rust-lang.org)
![Repo Size](https://img.shields.io/github/repo-size/ghtz08/kuguo-kgm-decoder?style=flat-square)
![Code Size](https://img.shields.io/github/languages/code-size/ghtz08/kuguo-kgm-decoder?style=flat-square)

## 介绍

一个命令行工具，可以用来解码酷狗缓存歌曲文件和下载的单曲收费歌曲文件。

解码原理来自博客[孤心浪子 - 闲来无事研究一下酷狗缓存文件kgtemp的加密方式](https://www.cnblogs.com/KMBlog/p/6877752.html)和 [ix64] 的 [unlock-music] 项目中的[酷狗解码实现]。

感谢 [ix64] 提供用于解码的 Key，[ix64] 的 [unlock-music] 项目提供了包括酷狗、网易云等多个平台的歌曲文件解码功能，并有网页和命令行两种使用方式。

[ix64]: https://github.com/ix64
[unlock-music]: https://github.com/ix64/unlock-music
[酷狗解码实现]: https://github.com/ix64/unlock-music/blame/1d415cae524dccc565cb339ba1a0225baf0b28fc/src/decrypt/kgm.js#L49-L59

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
| -c, --stdout | 输出到标准输出而不是文件，并且保留原文件（默认行为是删除） |
| -k, --keep | 保留原文件 |

> 生成的后缀为 .mp3 的文件不一定是 mp3 格式的文件，后续考虑加上自动判断解码后的文件类型这个功能。

## 许可证

[反 996 许可证版本 1.0](/LICENSE)