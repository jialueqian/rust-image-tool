# Rust 图像处理原型 — 进度日志

## 项目目标
借助 Claude Code，在零 Rust 基础下完成图像相似度检测工具原型。
证明：跨领域学习能力 + Agent 原生工作流。

> ⏱ 实际耗时：**1h2min**（从 `cargo init` 到 `git push`）

---

## 真实时间线

### 16:40 — 环境确认
- 发现 Rust 已安装（1.96.0）但未加入 PATH
- 修复：`export PATH="$HOME/.cargo/bin:$PATH"` 追加到 `.bashrc`
- 决策：不装 IDE、不配 LSP，纯命令行 + Claude Code 驱动

### 16:45 — 项目初始化
- `cargo init` 生成骨架
- 选依赖：`image` 做图片解码，`img_hash` 做感知哈希
- 决策：选 `img_hash` 3.x 而非 2.x，API 更现代但 DCT 算法被移入预处理层

### 16:50 — 第一版代码完成，编译报错
- 错误 1：crate 名写成 `img-hash`（连字符），crates.io 上注册为 `img_hash`（下划线）
- 错误 2：`image` 版本用了 0.25，但 `img_hash` 3.x 内部依赖 `image` 0.23，类型冲突
- 解决：将 `image` 降级到 0.23 匹配 `img_hash` 的依赖链
- 教训：Rust 生态中跨 crate 版本锁定是最常见的卡点

### 16:55 — 编译通过，但 hex 格式化报错
- `ImageHash` 不实现 `LowerHex` trait，不能用 `{:x}` 直接格式化
- 解决：手写 `hash_to_hex()`，遍历 `as_bytes()` 逐字节转 hex

### 17:00 — 首次运行成功
- 5 张测试图全部正确处理
- 效果：相同图距离 0，缩放图距离 1，旋转图距离 0
- 注：此版用的是 `HasherConfig::new()` 默认算法（Gradient），尚未启用 DCT 预处理；下一阶段才切到真正的 phash

### 17:10 — 升级：三算法对比 + CLI 参数
- 翻阅 img_hash 源码，发现 DCT 在 3.x 中是预处理选项（`preproc_dct()`），不是独立算法
- 三个算法定位：
  - ahash = Mean（均值哈希）
  - dhash = Gradient（梯度哈希）
  - phash = Mean + preproc_dct()（DCT 预处理）
- 加 `--threshold` 和 `--algo all` 参数

### 17:20 — 对比结果验证
- 三个算法都正确区分了相似图和无关图
- 缩放场景：ahash/phash 距离 0，dhash 距离 1——说明频域法对缩放更鲁棒
- 中文文件名导致的 UTF-8 截断 bug 已修复（改用字符计数而非字节切片）

### 17:35 — README + commit + push
- 完成。总耗时 1h2min。

---

## 技术决策

| 决策 | 选项 | 选择 | 理由 |
|------|------|------|------|
| 哈希算法 | SSIM / MSE / 感知哈希 | 感知哈希 | 无需原图对比，一张图一个 64-bit 指纹 |
| 感知哈希类型 | ahash / dhash / phash | phash 默认 + 对比 | phash 频域法抗缩放压缩最强 |
| 图像库版本 | image 0.25 vs 0.23 | 0.23 | 匹配 img_hash 依赖链，避免类型冲突 |
| 开发方式 | IDE + 系统学习 / Claude Code 驱动 | Claude Code 驱动 | 证明 Agent 原生工作流 |

## 三算法对比结论

| 场景 | ahash | dhash | phash |
|------|-------|-------|-------|
| 完全相同 | 0 ✅ | 0 ✅ | 0 ✅ |
| 旋转变换 | 0 ✅ | 0 ✅ | 0 ✅ |
| 缩放变换 | 0 ✅ | 1 🟡 | 0 ✅ |
| 无关图片 | - ✅ | - ✅ | - ✅ |

> phash 在缩放鲁棒性上优于 dhash。三算法协同可覆盖更多场景。
