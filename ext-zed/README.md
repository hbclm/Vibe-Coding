# VibeKit Zed Extension

**47 professional slash commands for AI coding** - from project init to deployment.

## ✨ Features

- 🎯 **47 Slash Commands** - All embedded, no network needed
- ⚡ **Instant Access** - Commands load instantly from WASM
- 📦 **Self-Contained** - No external dependencies
- 🔒 **Offline Ready** - Works without internet
- 🚀 **Lightweight** - Only ~250KB WASM

## 📋 Available Commands

### Project Management

- `/init` - Initialize new project
- `/update` - Update dependencies and code
- `/vibekit` - VibeKit help and information

### Code Quality

- `/review` - Code review
- `/review-code` - Code review guidance
- `/review-architect` - Architecture review
- `/review-audit` - Comprehensive code audit
- `/clean-code` - Clean and refactor code
- `/clean-deps` - Clean up dependencies
- `/refactor` - Code refactoring guidance

### Development

- `/debug` - Debug issues and errors
- `/test` - Testing guidance
- `/lang-js` - JavaScript best practices
- `/lang-php` - PHP best practices
- `/lang-ts` - TypeScript best practices

### API & Design

- `/api-design` - Design API endpoints and structure
- `/api-docs` - Generate API documentation
- `/design-ui` - UI/UX design guidance

### Documentation (15 commands)

- `/doc` - General documentation
- `/doc-api` - API documentation
- `/doc-architect` - Architecture documentation
- `/doc-architecture` - System architecture docs
- `/doc-business` - Business documentation
- `/doc-contributing` - Contributing guidelines
- `/doc-install` - Installation documentation
- `/doc-log` - Changelog documentation
- `/doc-migration` - Migration documentation
- `/doc-prd` - Product requirements document
- `/doc-readme` - README documentation
- `/doc-risks` - Risk assessment documentation
- `/doc-roadmap` - Product roadmap documentation
- `/doc-security` - Security documentation
- `/doc-techstack` - Tech stack documentation
- `/doc-todo` - TODO documentation
- `/doc-writer` - Documentation writer assistant

### Database

- `/db` - Database design and operations
- `/migrate-db` - Database migration helper

### Performance

- `/perf-optimize` - Performance optimization
- `/perf-profile` - Performance profiling

### Security

- `/sec-audit` - Security audit
- `/sec-practices` - Security best practices

### Git & Workflow

- `/git-commit` - Git commit message helper
- `/git-pr` - Pull request helper
- `/git-workflow` - Git workflow guidance

### Deployment

- `/deploy-devops` - Deployment and DevOps setup

### Migration

- `/migrate-code` - Code migration helper

## 🚀 Installation

### From Zed Extensions (Coming Soon)

1. Open Zed
2. `Cmd+Shift+P` / `Ctrl+Shift+P` → `zed: extensions`
3. Search "VibeKit"
4. Click Install

### As Dev Extension (Now)

1. Clone this repository
2. Open Zed
3. `Cmd+Shift+P` / `Ctrl+Shift+P` → `zed: install dev extension`
4. Select the `ext-zed` directory

## 💡 Usage

Simply type `/` in the Zed assistant panel and choose any VibeKit command:

```
/init
/review
/debug
/doc-api
```

Each command provides comprehensive guidance for its specific task.

## 🔧 Development

See [DEV_GUIDE.md](DEV_GUIDE.md) for development instructions.

### Quick Start

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-wasip2

# Build
cd ext-zed
cargo build --target wasm32-wasip2 --release
```

## 📊 Technical Details

- **Language**: Rust
- **Target**: WebAssembly (wasm32-wasip2)
- **Size**: ~250KB WASM
- **Commands**: 47 embedded via `include_str!` macro
- **Performance**: Instant load, no network latency

## 🆚 vs VSCode Extension

| Feature  | VSCode       | Zed           |
| -------- | ------------ | ------------- |
| Commands | 47 (dynamic) | 47 (embedded) |
| Network  | Required     | Not required  |
| Speed    | Good         | Instant       |
| Offline  | No           | Yes           |
| Size     | Small        | ~250KB        |

## 📄 License

Apache-2.0 - See [LICENSE](../../LICENSE)

## 🔗 Links

- [VibeKit Repository](https://github.com/HBCapital/VibeCoding)
- [Report Issues](https://github.com/HBCapital/VibeCoding/issues)
- [Zed Extensions](https://zed.dev/docs/extensions)
