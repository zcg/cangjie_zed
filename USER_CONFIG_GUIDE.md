# Cangjie LSP - Zed 用户配置指南

## 配置方法

在 Zed 的用户设置文件中添加以下配置：

### 打开 Zed 设置

1. 快捷键：`Ctrl+,`（Windows/Linux）或 `Cmd+,`（macOS）
2. 或者：菜单 → `File` → `Settings`

### 添加配置

在 `settings.json` 中添加：

```json
{
  "lsp": {
    "cangjie-lsp": {
      "binary": {
        "path": "D:\\Projects\\cangjie_lsp\\target\\release\\cangjie-lsp.exe"
      }
    }
  }
}
```

**注意**：
- Windows 路径使用双反斜杠 `\\`
- 或者使用正斜杠 `/` 也可以：`"D:/Projects/cangjie_lsp/target/release/cangjie-lsp.exe"`

---

## 完整配置示例

```json
{
  "lsp": {
    "cangjie-lsp": {
      "binary": {
        "path": "D:\\Projects\\cangjie_lsp\\target\\release\\cangjie-lsp.exe"
      },
      "settings": {
        "enableSemanticHighlighting": true,
        "trace": {
          "server": "verbose"
        }
      }
    }
  },
  "languages": {
    "Cangjie": {
      "language_servers": ["cangjie-lsp"],
      "tab_size": 4
    }
  }
}
```

---

## 路径选择

### 选项 1：开发版本（推荐用于测试）

```json
"path": "D:\\Projects\\cangjie_lsp\\target\\release\\cangjie-lsp.exe"
```

**优点**：
- 可以随时重新构建
- `cargo build --release` 后自动更新

### 选项 2：固定位置

把 LSP 复制到固定位置：
```bash
cp D:\Projects\cangjie_lsp\target\release\cangjie-lsp.exe C:\tools\cangjie-lsp.exe
```

配置：
```json
"path": "C:\\tools\\cangjie-lsp.exe"
```

### 选项 3：添加到 PATH

把 LSP 目录添加到系统 PATH，然后：
```json
"path": "cangjie-lsp"
```

---

## 验证配置

### 1. 保存设置
- `Ctrl+S` 保存 `settings.json`

### 2. 重启 Zed
- 完全关闭并重新打开

### 3. 打开 `.cj` 文件
- 打开任意仓颉文件

### 4. 检查 LSP 状态
- 右下角状态栏应该显示 LSP 信息
- 或者查看日志：`Ctrl+Shift+P` → "Open Log"

---

## 故障排除

### 问题：LSP 未启动

**检查步骤**：

1. **确认路径正确**
   ```bash
   ls "D:\Projects\cangjie_lsp\target\release\cangjie-lsp.exe"
   ```

2. **检查配置语法**
   - JSON 格式是否正确
   - 路径中的反斜杠是否转义（`\\`）

3. **查看 Zed 日志**
   - 搜索 `cangjie-lsp` 相关错误
   - 查看启动失败原因

### 问题：路径找不到

**Windows 路径格式**：

```json
// ✅ 正确（双反斜杠）
"path": "D:\\Projects\\cangjie_lsp\\target\\release\\cangjie-lsp.exe"

// ✅ 正确（正斜杠）
"path": "D:/Projects/cangjie_lsp/target/release/cangjie-lsp.exe"

// ❌ 错误（单反斜杠）
"path": "D:\Projects\cangjie_lsp\target\release\cangjie-lsp.exe"
```

---

## 配置模板

### 最小配置（仅 LSP 路径）

```json
{
  "lsp": {
    "cangjie-lsp": {
      "binary": {
        "path": "D:\\Projects\\cangjie_lsp\\target\\release\\cangjie-lsp.exe"
      }
    }
  }
}
```

### 完整配置（带额外选项）

```json
{
  "lsp": {
    "cangjie-lsp": {
      "binary": {
        "path": "D:\\Projects\\cangjie_lsp\\target\\release\\cangjie-lsp.exe",
        "arguments": []
      },
      "settings": {
        "enableSemanticHighlighting": true,
        "trace": {
          "server": "verbose"
        }
      },
      "initialization_options": {}
    }
  },
  "languages": {
    "Cangjie": {
      "language_servers": ["cangjie-lsp"],
      "tab_size": 4,
      "hard_tabs": false,
      "formatter": "language_server"
    }
  }
}
```

---

## 更新 LSP

当你重新构建 LSP 后：

```bash
# 重新构建
cd D:\Projects\cangjie_lsp
cargo build --release

# 无需其他操作！
# Zed 会自动使用新的可执行文件
```

---

## 📝 快速参考

**配置文件位置**：
- Zed 菜单：`File` → `Settings`
- 快捷键：`Ctrl+,`

**LSP 可执行文件**：
- `D:\Projects\cangjie_lsp\target\release\cangjie-lsp.exe`

**测试文件**：
- `D:\Projects\cangjie_lsp\tests\fixtures\sample.cj`

---

**现在你可以在 Zed 设置中自己配置 LSP 路径了！** ✨
