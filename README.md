# Cangjie Language Support for Zed

This extension provides comprehensive Cangjie (仓颉) language support for the [Zed](https://zed.dev) editor.

## Features

- **Syntax Highlighting** - Full syntax highlighting using Tree-sitter grammar
- **LSP Integration** - Code completion, diagnostics, go-to-definition, and more via Cangjie Language Server
- **Linting** - Static analysis through LSP diagnostics
- **Debugging** - Debug support via Cangjie Debugger (cjdb) with DAP protocol
- **Code Snippets** - 30+ common code templates for rapid development
- **Text Objects** - Vim-style navigation for functions, classes, blocks, etc.

## Requirements

- [Zed Editor](https://zed.dev)
- [Cangjie SDK](https://developer.huawei.com/consumer/cn/cangjie/) installed
- `CANGJIE_HOME` environment variable set to your SDK installation path

## Installation

### From Zed Extensions

1. Open Zed
2. Go to Extensions (`Cmd+Shift+X` on macOS, `Ctrl+Shift+X` on Linux/Windows)
3. Search for "Cangjie"
4. Click Install

### Development Installation

1. Clone this repository
2. Install Rust with WASM target:
   ```bash
   rustup target add wasm32-wasip1
   ```
3. Build the extension:
   ```bash
   cargo build --release --target wasm32-wasip1
   ```
4. In Zed, open Extensions and select "Install Dev Extension"
5. Choose the `cangjie_zed` directory

## Configuration

### LSP Configuration (Required)

Add the following to your Zed settings (`settings.json`):

```json
{
  "lsp": {
    "cangjie-lsp": {
      "settings": {
        "serverPath": "D:\Cangjie\tools\bin\LSPServer.exe",
        "proxyPath": "D:\path\to\cangjie-lsp-proxy.exe"
      }
    }
  }
}
```

**Configuration Options:**
- `serverPath` (required): Full path to `LSPServer.exe` from Cangjie SDK
- `proxyPath` (optional): Path to LSP proxy for protocol compatibility fix

> **Note:** The LSP proxy is required for Cangjie SDK v1.0.4 and earlier due to a protocol compatibility issue. The SDK's LSPServer sends `Content-Length:N` (missing space after colon), which violates the LSP specification. The proxy fixes this automatically. This is expected to be fixed in SDK v1.0.5+.

### Getting the LSP Proxy

Download pre-built binaries from [GitHub Releases](https://github.com/aspect-meta/cangjie-zed/releases), or build from source:

```bash
cd lsp-proxy
cargo build --release
# Binary: target/release/cangjie-lsp-proxy.exe
```

### Environment Setup

Ensure `CANGJIE_HOME` is set in your shell environment:

```bash
# Linux/macOS
export CANGJIE_HOME=/path/to/cangjie-sdk

# Windows (PowerShell)
$env:CANGJIE_HOME = "C:\path\to\cangjie-sdk"
```

### Debug Configuration

Create a `.zed/debug.json` in your project:

```json
{
  "configurations": [
    {
      "name": "Debug Cangjie Program",
      "adapter": "cangjie",
      "request": "launch",
      "program": "${workspaceFolder}/build/main",
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

### Tasks Configuration

Copy `tasks/cangjie.json` to `.zed/tasks.json` in your project for cjpm integration:

| Task | Description |
|------|-------------|
| `Cangjie: Build` | Build the project |
| `Cangjie: Build Release` | Build with release optimizations |
| `Cangjie: Run` | Run the project |
| `Cangjie: Test` | Run tests |
| `Cangjie: Clean` | Clean build artifacts |
| `Cangjie: Format` | Format current file |
| `Cangjie: Lint` | Lint current file |

## Snippets

Type the prefix and press `Tab` to expand:

| Prefix | Description |
|--------|-------------|
| `main` | Main function |
| `func` | Function definition |
| `class` | Class definition |
| `classe` | Class with inheritance |
| `struct` | Struct definition |
| `interface` | Interface definition |
| `enum` | Enum definition |
| `if` / `ife` | If / If-else statement |
| `match` | Match expression |
| `for` / `forr` | For loop / Range loop |
| `while` | While loop |
| `try` / `tryf` | Try-catch / Try-catch-finally |
| `prop` / `propg` | Property / Read-only property |
| `init` | Constructor |
| `let` / `var` | Immutable / Mutable variable |
| `lam` | Lambda expression |
| `spawn` | Spawn concurrent task |
| `sync` | Synchronized block |
| `imp` / `impf` | Import / From-import |
| `pl` / `pr` | println / print |
| `test` | Test function |

## Project Structure

```
cangjie_zed/
├── extension.toml              # Extension metadata
├── Cargo.toml                  # Rust dependencies
├── .cargo/config.toml          # WASM build target
├── src/lib.rs                  # Extension implementation
├── lsp-proxy/                  # LSP protocol compatibility proxy
│   ├── Cargo.toml
│   └── src/main.rs
├── languages/cangjie/
│   ├── config.toml             # Language configuration
│   ├── highlights.scm          # Syntax highlighting
│   ├── brackets.scm            # Bracket matching
│   ├── indents.scm             # Indentation rules
│   ├── outline.scm             # Code outline
│   ├── folds.scm               # Code folding
│   └── textobjects.scm         # Vim text objects
├── snippets/
│   └── cangjie.json            # Code snippets
├── tasks/
│   └── cangjie.json            # Task templates
└── debug_adapter_schemas/
    └── Cangjie.json            # Debug configuration schema
```

## Troubleshooting

### LSP not starting

1. Verify `serverPath` is correctly configured in Zed settings
2. Check that the LSPServer executable exists at the specified path
3. If using SDK v1.0.4 or earlier, ensure `proxyPath` is configured
4. Restart Zed after changing settings

### "invalid LSP message header" error

This error indicates the LSP protocol compatibility issue. Configure `proxyPath` in your Zed settings to use the LSP proxy.

### Debug not working

1. Ensure `cjdb` is available at `$CANGJIE_HOME/tools/bin/cjdb`
2. Verify your program is compiled with debug symbols

### Snippets not working

1. Ensure you're in a `.cj` file
2. Type the prefix exactly and press `Tab`
3. Check if snippets are enabled in Zed settings

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## Acknowledgments

- [Cangjie Programming Language](https://developer.huawei.com/consumer/cn/cangjie/)
- [Tree-sitter Cangjie Grammar](https://gitcode.com/Cangjie-SIG/tree-sitter-cangjie)
- [Zed Editor](https://zed.dev)
