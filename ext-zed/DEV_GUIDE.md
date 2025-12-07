# VibeKit Zed Extension - Development Guide

## Prerequisites

### Required

- **Rust** installed via [rustup](https://rustup.rs/)

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

  > ⚠️ **Important**: Rust MUST be installed via rustup. Homebrew or other package managers will NOT work for Zed extensions.

- **WASM target** for Rust:

  ```bash
  rustup target add wasm32-wasip2
  ```

- **Zed editor** installed

### Optional

- **cargo-watch** for auto-rebuild:
  ```bash
  cargo install cargo-watch
  ```

## Project Structure

```
ext-zed/
├── extension.toml      # Extension manifest
├── Cargo.toml          # Rust project config
├── src/
│   └── lib.rs         # Main extension code
├── README.md
└── DEV_GUIDE.md       # This file
```

## Building the Extension

### One-time Build

```bash
cd ext-zed
cargo build --target wasm32-wasip2 --release
```

The compiled WASM module will be at:

```
target/wasm32-wasip2/release/vibekit.wasm
```

### Watch Mode (Auto-rebuild)

```bash
cargo watch -x 'build --target wasm32-wasip2'
```

## Installing as Dev Extension

1. **Open Zed**

2. **Open Command Palette**

   - macOS: `Cmd+Shift+P`
   - Windows/Linux: `Ctrl+Shift+P`

3. **Run Command**

   ```
   zed: install dev extension
   ```

4. **Select Directory**

   - Navigate to and select the `ext-zed` directory

5. **Verify Installation**
   - Open Extensions panel
   - You should see "VibeKit" with "Overridden by dev extension" label

## Testing

### Configure Extension

Add to Zed settings (`Cmd+,` / `Ctrl+,`):

```json
{
  "vibekit": {
    "git_repo": "HBCapital/VibeCoding",
    "git_branch": "main",
    "rules_path": "rules/vibekit.md",
    "commands_path": "agents"
  }
}
```

### Test Sync

1. Open Assistant panel (`Cmd+?` / `Ctrl+?`)
2. Type `/vibekit sync`
3. Check for success messages
4. Verify `.cursorrules` file created in project root

### Test Commands

1. Type `/vibekit list`
2. Verify commands are listed
3. Type `/vibekit init` (or any command)
4. Verify command content appears

## Debugging

### View Logs

**Option 1: Command Palette**

```
zed: open log
```

**Option 2: Verbose Logging**

```bash
# Close Zed first, then run:
zed --foreground
```

This shows INFO level logs in the terminal.

### Common Issues

#### Build Errors

```bash
# Clean and rebuild
cargo clean
cargo build --target wasm32-wasip2 --release
```

#### Extension Not Loading

1. Check `extension.toml` syntax
2. Verify WASM file exists in `target/wasm32-wasip2/release/`
3. Reinstall dev extension

#### Runtime Errors

Check `Zed.log` for error messages. Common issues:

- Invalid JSON in settings
- Network errors (check internet connection)
- GitHub API rate limits (use token)

## Code Structure

### Main Components

**`VibeKitExtension`**

- Main extension struct
- Implements `zed::Extension` trait
- Manages command cache

**`get_config()`**

- Reads Zed settings
- Validates configuration

**`fetch_from_github()`**

- Fetches content from GitHub API
- Handles authentication

**`sync_rules()`**

- Fetches rules from repo
- Writes to `.cursorrules`

**`fetch_commands()`**

- Lists files in commands directory
- Fetches and caches each command

**`run_slash_command()`**

- Entry point for `/vibekit` command
- Routes to sync/list/execute

### Adding New Features

1. **Add new subcommand**:

   ```rust
   match subcommand.as_str() {
       "sync" => { /* ... */ }
       "list" => { /* ... */ }
       "your-new-command" => {
           // Your code here
       }
       _ => { /* ... */ }
   }
   ```

2. **Rebuild and test**:

   ```bash
   cargo build --target wasm32-wasip2 --release
   ```

3. **Reload extension** in Zed (reinstall dev extension)

## Publishing

### Prerequisites

1. Extension tested and working
2. Version bumped in `extension.toml` and `Cargo.toml`
3. README and CHANGELOG updated

### Steps

1. **Build release**:

   ```bash
   cargo build --target wasm32-wasip2 --release
   ```

2. **Create PR to zed-industries/extensions**:

   ```bash
   # Fork https://github.com/zed-industries/extensions
   # Add your extension to the repo
   # Create PR
   ```

3. **Wait for review** from Zed team

See [Zed Extension Publishing Guide](https://zed.dev/docs/extensions/developing-extensions#publishing-your-extension) for details.

## Resources

- [Zed Extension API Docs](https://docs.rs/zed_extension_api/)
- [Zed Extension Examples](https://github.com/zed-industries/zed/tree/main/extensions)
- [Rust WASM Book](https://rustwasm.github.io/docs/book/)
- [GitHub API Documentation](https://docs.github.com/en/rest)

## Tips

- **Fast iteration**: Use `cargo watch` for auto-rebuild
- **Test with public repos first**: Easier to debug without auth
- **Check API rate limits**: GitHub allows 60 requests/hour without token, 5000 with token
- **Cache aggressively**: Minimize API calls
- **Handle errors gracefully**: Show helpful messages to users
