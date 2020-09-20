# 酷狗混淆的歌曲文件的解码器

一个命令行工具，可以用来解码酷狗缓存歌曲文件和下载的单曲收费歌曲文件。解码原理来自博客[孤心浪子 - 闲来无事研究一下酷狗缓存文件kgtemp的加密方式](https://www.cnblogs.com/KMBlog/p/6877752.html)和 [ix64] 的 [unlock-music] 项目中的[酷狗解码实现]。感谢 [unlock-music] 的作者 [ix64] 提供用于解码的 KEY，[unlock-music] 提供了包括酷狗、网易云等多个平台的歌曲文件解码功能并有网页和命令行两种使用方式。

[ix64]: https://github.com/ix64
[unlock-music]: https://github.com/ix64/unlock-music
[酷狗解码实现]: https://github.com/ix64/unlock-music/blame/1d415cae524dccc565cb339ba1a0225baf0b28fc/src/decrypt/kgm.js#L49-L59

## 使用方式

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

默认情况下，如果转码成功会删除原文件并生成后缀为 .mp3 的文件，通过参数 `-k` 保留原文件。
> 后缀为 .mp3 不一定是 mp3 格式的文件，后续考虑加上自动判断解码后的文件类型这个功能。

## 许可证

[MIT License](/LICENSE)