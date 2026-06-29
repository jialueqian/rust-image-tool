# rust-image-tool — Claude Code 项目约定

## 项目概述
图像相似度检测 CLI 工具。基于感知哈希（ahash/dhash/phash），汉明距离比较。

## 常用命令
```bash
cargo build              # 编译
cargo run -- test_images/          # 运行（默认 phash，阈值 15）
cargo run -- test_images/ --algo all   # 三算法对比
cargo check              # 仅类型检查，不生成二进制
```

## 依赖约定
- `image` 版本锁死在 **0.23**（`img_hash` 3.x 内部依赖链绑定 0.23，升级会类型冲突）
- `img_hash` 3.x，DCT 是预处理选项（`preproc_dct()`），不是独立算法

## 代码约定
- 中文注释、英文标识符
- 参数解析手写，不引入 clap（减少依赖）
- Unicode truncate 用字符计数（`.chars()`），不用字节切片
