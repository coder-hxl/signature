# Signature

An automated tool for batch signing files. By configuring `signature.config.json`, an existing signing tool (such as `signtool.exe` ) is invoked to concurrently sign multiple files and output structured results.

> This tool is not a replacement for signtool.exe, but a batch signature automation wrapper that can implement batch, concurrent, and highly controllable signature tasks by invoking existing signature tools (such as signtool.exe). It is especially suitable for CI/CD scenarios or automatic signing of a large number of files.

## Features

- ✅ Batch signature support - Support `glob` mode (such as `**/*.exe`) through `include` configuration to process multiple files at one time
- ✅ Concurrent execution - Use Tokio to run multiple signature tasks asynchronously, significantly speeding up the signature efficiency
- ✅ JSON configuration - All parameters are written to `signature.config.json`, which is clear and easy to reuse
- ✅ Pluggable signature tools - Support any command line signature tool (such as Microsoft `signtool.exe`), regardless of the source of the tool
- ✅ JSON result output - Signature results are written to `result.json` for subsequent processing, display or automated analysis
- ✅ Clear error tracking - Error details are recorded when each file fails to sign, which is convenient for troubleshooting
- ✅ Easy to integrate - Can be used in CI/CD pipelines or as part of the build process

## Install

Please go to the release page to download the corresponding installation package: [releases](https://github.com/coder-hxl/signature/releases)

## Use

Put `signature.config.json` in the same directory as `signature.exe`, and the program will automatically read the configuration and process the signature when started.

1. Create a configuration file `signature.config.json` in the same directory as `signature.exe`

2. Configure the signature tool path, parameters and matching file path

3. Execute: `signature.exe`
4. View the signature result output in `result.json`

## Example

```json
{
  "signTool": "C:\\Program Files (x86)\\Windows Kits\\10\\App Certification Kit\\signtool.exe",
  "args": [
    "sign",
    "/fd",
    "sha256",
    "/sha1",
    "YOUR-CERT-SHA1",
    "/tr",
    "http://timestamp.sectigo.com",
    "/td",
    "sha256"
  ],
  "include": ["./release/**/*.exe", "./release/**/*.dll", "./debug/*.exe"]
}
```

## Configuration

signature.config.json configuration item

### signTool

- Type: `string`

The full path to the signature tool, for example, use signtool.exe, which comes with the Microsoft Windows SDK.
This field determines which signing tool the program actually calls.

```json
"signTool": "C:\\Program Files (x86)\\Windows Kits\\10\\App Certification Kit\\signtool.exe"
```

### args

- Type: `string[]`

An array of parameters passed to the signing tool.
You can write the full signature command parameters here (except for the file name to be signed, which the program automatically appends).

```json
"args": [
  "sign",
  "/fd", "sha256",
  "/sha1", "YOUR-CERT-SHA1",
  "/tr", "http://timestamp.sectigo.com",
  "/td", "sha256"
]
```

### include

- Type: `string[]`

The glob wildcard syntax is supported. For example, `**/*.exe` matches all.exe files in multi-layer directories.
The program recursively finds the file and signs it based on these patterns.

```json
"include": [
  "./bin/**/*.exe",
  "./libs/*.dll",
  "./release/**/*.exe",
  "./release/**/*.dll"
]
```

---

# Signature

一个用于批量签名文件的自动化工具。通过配置 `signature.config.json`，调用现有签名工具（如 `signtool.exe`）对多个文件并发签名，并输出结构化结果。

> 本工具不是替代 `signtool.exe`，而是一个**批量签名自动化封装器**，通过调用现有签名工具（如 `signtool.exe`）实现批量、并发、高可控的签名任务，特别适合 CI/CD 场景或大量文件自动化签名。

## 特性

- ✅ 批量签名支持 - 通过 `include` 配置支持 `glob` 模式（如 `**/*.exe`），一次性处理多个文件
- ✅ 并发执行 - 使用 Tokio 异步运行多个签名任务，显著加快签名效率
- ✅ JSON 配置 - 所有参数统一写入 `signature.config.json`，清晰、易复用
- ✅ 可插拔签名工具 - 支持任意命令行签名工具（如微软 `signtool.exe`），不限制工具来源
- ✅ JSON 结果输出 - 签名结果写入 `result.json`，方便后续处理、展示或自动化分析
- ✅ 错误追踪清晰 - 每个文件签名失败时都会记录错误详情，便于排查问题
- ✅ 易集成 - 可用于 CI/CD 流水线或作为构建流程的一部分

## 安装

请到发布页下载对应的安装包：[releases](https://github.com/coder-hxl/signature/releases)

## 使用

把 `signature.config.json` 放在 `signature.exe` 同一个目录里，启动后程序会自动读取配置并处理签名。

1. 在 `signature.exe` 同级目录下创建配置文件 `signature.config.json`

2. 配置签名工具路径、参数和匹配文件路径

3. 执行：`signature.exe`
4. 查看签名结果输出于 `result.json`

## 示例

```json
{
  "signTool": "C:\\Program Files (x86)\\Windows Kits\\10\\App Certification Kit\\signtool.exe",
  "args": [
    "sign",
    "/fd",
    "sha256",
    "/sha1",
    "YOUR-CERT-SHA1",
    "/tr",
    "http://timestamp.sectigo.com",
    "/td",
    "sha256"
  ],
  "include": ["./release/**/*.exe", "./release/**/*.dll", "./debug/*.exe"]
}
```

## 配置

signature.config.json 配置项

### signTool

类型：`string`

签名工具的完整路径，例如可使用微软 Windows SDK 自带的 `signtool.exe`。
这个字段决定了程序实际调用哪个签名工具。

```json
"signTool": "C:\\Program Files (x86)\\Windows Kits\\10\\App Certification Kit\\signtool.exe"
```

### args

类型：`string[]`

传递给签名工具的参数数组。
你可以在这里写入完整的签名命令参数（除了待签名的文件名，程序会自动追加）。

```json
"args": [
  "sign",
  "/fd", "sha256",
  "/sha1", "YOUR-CERT-SHA1",
  "/tr", "http://timestamp.sectigo.com",
  "/td", "sha256"
]
```

### include

类型：`string[]`

待签名的文件路径，支持 glob 通配符语法，例如 `**/*.exe` 可以匹配多层目录下的所有 `.exe` 文件。

程序会根据这些模式递归查找文件并对其进行签名。

```json
"include": [
  "./bin/**/*.exe",
  "./libs/*.dll",
  "./release/**/*.exe",
  "./release/**/*.dll"
]
```
