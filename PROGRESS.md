# Rust 图像处理原型 — 进度日志

## 项目目标
24 小时内借助 Claude Code，用 Rust 完成图像相似度检测工具原型。
证明：跨领域学习能力 + Agent 原生工作流。

## 进度记录

### 2026-06-29 — 项目启动 ✅
- [x] 环境搭建（Rust 工具链安装）
- [x] 项目初始化（cargo init）
- [x] 技术方案选定（感知哈希 phash）
- [x] 核心功能实现（单张哈希 → 两两比较 → 批量处理）
- [x] 测试验证通过
- [ ] GitHub 上传 + README

## 突破点记录

### 突破 1：版本兼容解决
- 问题：`img_hash` 3.x 依赖 `image` 0.23，但 Cargo.toml 指定了 0.25，导致类型不匹配
- 解决：将 image 降级到 0.23 匹配 img_hash 的依赖链
- 教训：Rust 生态中 crate 版本锁定是常见卡点，查 `cargo tree` 定位

### 突破 2：phash 的实际效果验证
- lbl.jpg vs lbl_copy.jpg: 距离 0（完美识别完全相同的图）
- lbl.jpg vs lbl_rotation.jpg: 距离 0（旋转不影响 phash）
- lbl.jpg vs lbl_small.jpg: 距离 1（缩小到 5% 也只差 1 位）
- 结果验证了 phash 的抗缩放/旋转能力远优于简单哈希

## 决策日志

| 时间 | 决策 | 理由 |
|------|------|------|
| 16:50 | 选感知哈希而非 SSIM/MSE | 无需求导，计算量小，一张图一个 64-bit 指纹即可比较 |
| 17:20 | 选 phash 而非 ahash | phash 基于频域（DCT），抗缩放/压缩更强 |
| 17:25 | image 降级到 0.23 | 匹配 img_hash 的依赖链，避免引入两套 image 版本 |
