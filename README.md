# Rust 图像相似度检测工具

> 零 Rust 基础，借助 **Claude Code** 在 **1 小时 2 分钟**内完成的跨领域原型。
>
> 🎯 证明"借 AI Agent 进入陌生技术领域快速产出"的能力。

## 功能

- 支持 **三种感知哈希算法**：ahash（均值）、dhash（梯度）、phash（DCT 频域）
- 可自定义相似度阈值（`--threshold`）
- 三算法对比模式（`--algo all`），一张表看清差异
- 抗缩放、旋转、轻微压缩

## 快速开始

```bash
# 单算法模式（默认 phash）
cargo run -- test_images/

# 指定算法
cargo run -- test_images/ --algo ahash

# 三算法对比
cargo run -- test_images/ --algo all

# 自定义阈值（默认 15）
cargo run -- test_images/ --threshold 10
```

## 三算法对比结论

| 场景 | ahash (Mean) | dhash (Gradient) | phash (DCT+Mean) |
|------|-------------|------------------|-------------------|
| 完全相同 | 0 ✅ | 0 ✅ | 0 ✅ |
| 旋转变换 | 0 ✅ | 0 ✅ | 0 ✅ |
| 缩放变换 | 0 ✅ | 1 🟡 | 0 ✅ |
| 无关图片 | 排除 ✅ | 排除 ✅ | 排除 ✅ |

> phash 缩放鲁棒性最优。三算法协同覆盖更多场景。

## 技术栈

| 层 | 选型 |
|----|------|
| 语言 | Rust（项目开始时零基础） |
| 图像解码 | `image` 0.23 |
| 感知哈希 | `img_hash` 3.x |
| 开发驱动 | Claude Code 全程辅助 |

## 项目叙事

- 不会 Rust → 无需会语法 → Claude Code 驱动 → 1h2min 跑通完整原型
- 遇到 3 个编译卡点（依赖版本冲突、trait 不匹配、UTF-8 截断），均在数分钟内定位解决
- 最终产出一个可运行、有对比实验、有过程日志的工具

详见 [PROGRESS.md](./PROGRESS.md)

## License

MIT
