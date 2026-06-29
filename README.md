# Rust 图像相似度检测工具

借助 **Claude Code**，在零 Rust 基础下，**24 小时内**完成的跨领域原型项目。

> 🎯 证明"借 AI Agent 进入陌生技术领域快速产出"的能力。

## 功能

给定一个图片文件夹，自动检测其中相似的图片对：

- 使用**感知哈希（phash）**算法，基于频域 DCT 变换
- 抗缩放、旋转、轻微压缩——比简单像素哈希可靠得多
- 输出所有相似图片对及汉明距离

## 运行

```bash
cargo run -- test_images/
```

输出示例：

```
找到 5 张图片

[哈希] test_images\a.jpg → 4c0e8e0607272f09
[哈希] test_images\a_copy.jpg → 4c0e8e0607272f09
[哈希] test_images\a_small.jpg → 4c4e8e0607272f09

--- 相似结果 ---
相似: a.jpg  <->  a_copy.jpg  (距离: 0)
相似: a.jpg  <->  a_small.jpg  (距离: 1)
```

## 技术栈

| 层 | 选型 |
|----|------|
| 语言 | Rust（项目开始时零基础） |
| 图像解码 | `image` 0.23 |
| 感知哈希 | `img_hash` 3.x (phash / DCT-based) |
| 开发驱动 | Claude Code 全程辅助 |

## 为什么做这个

面向 DeepSeek Agent Harness 团队的实习投递。Harness 团队的要求是：

> "会不会 Rust 不重要，能不能借 AI 一天内用 Rust 跑通原型才重要。"

这个项目就是这句话的兑现。

## License

MIT
