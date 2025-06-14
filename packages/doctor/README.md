# Doctor Engine

[![npm version](https://badge.fury.io/js/@shined/doctor.svg)](https://badge.fury.io/js/@shined/doctor)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance code quality inspection tool powered by Rust and Node.js.

## Features

- 🚀 **High Performance**: Built with Rust for maximum speed and efficiency
- 🔍 **Comprehensive Analysis**: Supports multiple programming languages and frameworks
- ⚡ **Parallel Processing**: Utilizes multi-threading for faster code scanning

## Installation

```bash
npm install @shined/doctor
# or
yarn add @shined/doctor
# or
pnpm add @shined/doctor
```

## Quick Start

```javascript
import { Standards } from "@shined/doctor";

// Initialize Standards checker
const standards = Standards.create(process.cwd());

// Run all validations
const results = await standards.validateAll({
  withDashboard: true,
  quiet: false,
});
```

## Configuration

Create a `.sfconfig/spec.json` file in your project root:

```json
{
  "globals": {
    "yourGlobalVar": "writable"
  },
  "ignore": ["**/node_modules/**", "**/dist/**", "**/build/**", "**/target/**"]
}
```

The tool will also check for:

- `.npmrc` - NPM registry configuration
- `.node-version` - Node.js version specification
- `package.json` - Package configuration

## CLI Usage

```bash
# Install globally
npm install -g @shined/doctor

# Run analysis on current directory
npx @shined/doctor

# Run with verbose output
npx @shined/doctor --verbose

# Run with custom working directory
npx @shined/doctor --cwd /path/to/project

# Show help
npx @shined/doctor --help
```

The command will:

- Check project health
- Show error count if any errors are found
- Display execution time
- Exit with code 1 if errors are found

## API Reference

### Class: Standards

Main class for code quality validation.

#### Static Methods

- `create(cwd: string): Standards` - Create a new Standards instance

#### Instance Methods

- `validateNpmrc(): Promise<Array<Messages>>` - Validate npmrc configuration
- `validateNodeVersion(): Promise<Array<Messages>>` - Validate Node.js version
- `validatePackageJson(): Promise<Array<Messages>>` - Validate package.json
- `validateLint(): Promise<Array<Messages>>` - Run linting validation
- `validateAll(opts?: RenderOpts): Promise<Array<Messages>>` - Run all validations

#### RenderOpts Interface

| Option         | Type    | Description             |
| -------------- | ------- | ----------------------- |
| withDashboard  | boolean | Enable dashboard view   |
| maxRenderCount | number  | Maximum items to render |
| quiet          | boolean | Suppress output         |

### Utility Functions

#### Code Statistics

```typescript
function cloc(paths: string[], opts?: { ignore?: string[] }): LanguageStats[];
```

#### Debug Functions

```typescript
function initializeLogger(level?: LogLevel): void;
function unSafeInnerLint(
  globArgs: GlobArgs,
  category: NaPiCategory
): Promise<Diagnostic[]>;
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

MIT © [Your Name]

## Support

- Documentation: [Link to docs]
- Issues: [GitHub Issues]
- Discord: [Discord Channel]

## Acknowledgments

Built with:

- [Rust](https://www.rust-lang.org/)
- [Node.js](https://nodejs.org/)
- [oxc](https://github.com/oxc-project/oxc)
