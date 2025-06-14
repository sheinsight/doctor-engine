# Doctor Engine

[![npm version](https://badge.fury.io/js/@shined/doctor.svg)](https://badge.fury.io/js/@shined/doctor)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

基于 Rust 和 Node.js 构建的高性能代码质量检查工具。

[English](./README.md) | 简体中文

## 特性

- 🚀 **高性能**：使用 Rust 构建，确保最大速度和效率
- 🔍 **全面分析**：支持多种编程语言和框架
- ⚡ **并行处理**：利用多线程进行更快的代码扫描

## 安装

```bash
npm install @shined/doctor
# 或者
yarn add @shined/doctor
# 或者
pnpm add @shined/doctor
```

## 快速开始

```javascript
import { Standards } from "@shined/doctor";

// 初始化 Standards 检查器
const standards = Standards.create(process.cwd());

// 运行所有验证
const results = await standards.validateAll();
```

## 配置

在项目根目录创建 `.sfconfig/spec.json` 文件：

```json
{
  "globals": {
    "yourGlobalVar": "writable"
  },
  "ignore": ["**/node_modules/**", "**/dist/**", "**/build/**", "**/target/**"]
}
```

工具还会检查以下配置文件：

- `.npmrc` - NPM 注册表配置
- `.node-version` - Node.js 版本规范
- `package.json` - 包配置

## 命令行使用

```bash
# 全局安装
npm install -g @shined/doctor

# 在当前目录运行分析
npx @shined/doctor

# 显示详细输出
npx @shined/doctor --verbose

# 指定工作目录
npx @shined/doctor --cwd /path/to/project

# 显示帮助信息
npx @shined/doctor --help
```

命令将会：

- 检查项目健康状况
- 显示发现的错误数量
- 显示执行时间
- 如果发现错误则以代码 1 退出

## API 参考

### Standards 类

代码质量验证的主类。

#### 静态方法

- `create(cwd: string): Standards` - 创建新的 Standards 实例

#### 实例方法

- `validateNpmrc(): Promise<Array<Messages>>` - 验证 npmrc 配置
- `validateNodeVersion(): Promise<Array<Messages>>` - 验证 Node.js 版本
- `validatePackageJson(): Promise<Array<Messages>>` - 验证 package.json
- `validateLint(): Promise<Array<Messages>>` - 运行代码检查验证
- `validateAll(): Promise<Array<Messages>>` - 运行所有验证

### 工具函数

#### 代码统计

```typescript
function cloc(paths: string[], opts?: { ignore?: string[] }): LanguageStats[];
```

#### 调试函数

```typescript
function initializeLogger(level?: LogLevel): void;
function unSafeInnerLint(
  globArgs: GlobArgs,
  category: NaPiCategory
): Promise<Diagnostic[]>;
```

## 贡献

欢迎贡献！请查看我们的[贡献指南](CONTRIBUTING.md)了解详情。

## 许可证

MIT © [Your Name]

## 支持

- 文档：[链接到文档]
- 问题：[GitHub Issues]
- Discord：[Discord 频道]

## 致谢

基于以下技术构建：

- [Rust](https://www.rust-lang.org/)
- [Node.js](https://nodejs.org/)
- [oxc](https://github.com/oxc-project/oxc)
