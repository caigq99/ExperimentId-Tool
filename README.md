# ExperimentId-Tool

一个用于管理 macOS 应用程序实验 ID 的命令行工具。

## 功能特性

- 🔧 读取和设置 macOS 应用程序的 ExperimentId
- 🆔 自动生成 UUID v4 格式的实验 ID
- 🔄 同时重置 DidNonAnonymousUserLogIn 字段为 false
- 🎯 支持自定义域名和 ID
- ✅ 写入后自动验证确认
- 🖥️ 专为 macOS 系统设计

## 安装

### 前提条件

- macOS 系统
- Rust 工具链 (推荐使用 [rustup](https://rustup.rs/))

### 从源码构建

```bash
git clone <repository-url>
cd ExperimentId-Tool
cargo build --release
```

编译完成后，可执行文件位于 `target/release/ExperimentId-Tool`

## 使用方法

### 基本用法

生成新的实验 ID 并写入默认域名：

```bash
./ExperimentId-Tool
```

### 命令行选项

#### `--domain <域名>`

指定要操作的应用程序域名。

```bash
./ExperimentId-Tool --domain com.example.myapp
```

默认域名：`dev.warp.Warp-Stable`

#### `--id <UUID>`

指定要设置的 UUID，而不是自动生成。

```bash
./ExperimentId-Tool --id 550e8400-e29b-41d4-a716-446655440000
```

### 使用示例

1. **使用默认设置**：
   ```bash
   ./ExperimentId-Tool
   ```

2. **为特定应用设置实验 ID**：
   ```bash
   ./ExperimentId-Tool --domain com.mycompany.myapp
   ```

3. **设置指定的 UUID**：
   ```bash
   ./ExperimentId-Tool --domain com.mycompany.myapp --id 123e4567-e89b-12d3-a456-426614174000
   ```

## 输出说明

工具运行时会显示以下信息：

- 🔎 **旧的 ExperimentId**: 显示当前存储的实验 ID（如果存在）
- ⚠️ **未找到旧的 ExperimentId**: 首次设置时的提示
- ✅ **新的 ExperimentId 已写入**: 确认新 ID 已成功写入
- ✅ **DidNonAnonymousUserLogIn 已重置为 false**: 确认登录状态已重置
- 📌 **回读确认 ExperimentId**: 验证写入的 ID
- 📌 **回读确认 DidNonAnonymousUserLogIn**: 验证登录状态重置

## 技术细节

### 依赖项

- `uuid`: 用于生成和解析 UUID
- 标准库的 `std::process::Command`: 用于执行 macOS `defaults` 命令

### 工作原理

该工具通过 macOS 的 `defaults` 命令来读取和写入应用程序的偏好设置：

1. 使用 `defaults read` 读取当前的 ExperimentId
2. 生成新的 UUID v4 或使用用户指定的 UUID
3. 使用 `defaults write` 写入新的 ExperimentId
4. 同时使用 `defaults write` 将 DidNonAnonymousUserLogIn 重置为 false
5. 再次读取两个字段以确认写入成功

### 错误处理

- 无效的 UUID 格式会导致程序退出并显示错误信息
- `defaults` 命令执行失败会显示相应的错误信息
- 缺少必需参数时会显示使用帮助

## 开发

### 运行开发版本

```bash
cargo run -- [选项]
```

### 运行测试

```bash
cargo test
```

### 代码格式化

```bash
cargo fmt
```

### 代码检查

```bash
cargo clippy
```

## 许可证

[在此添加许可证信息]

## 贡献

欢迎提交 Issue 和 Pull Request！

## 注意事项

- 此工具仅适用于 macOS 系统
- 需要适当的权限来修改应用程序偏好设置
- 建议在修改重要应用程序的设置前先备份
