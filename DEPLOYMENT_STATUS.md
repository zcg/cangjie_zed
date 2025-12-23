# ✅ Zed 扩展部署完成

## 已完成的操作

### 1. 文件部署

**Zed 扩展目录**：`C:\Users\zcg\AppData\Local\Zed\extensions\work\cangjie\`

**已部署文件**：
- ✅ `extension.toml` - 配置文件（已更新）
- ✅ `extension.wasm` - WASM 扩展代码（265KB）
- ✅ `bin/cangjie-lsp.exe` - LSP 服务器（4.2MB）

### 2. 配置修改

**extension.toml**：
```toml
[language_servers.cangjie-lsp.binary]
command = "bin/cangjie-lsp"  # Zed 在 Windows 上会自动加 .exe
args = []
```

---

## 🎯 下一步操作

### ⚠️ 重要：必须重启 Zed

**Zed 扩展是 WASM 运行的**，修改文件后需要：

1. **完全关闭 Zed**（不是最小化）
2. **重新启动 Zed**
3. **打开 `.cj` 文件**

---

## 🧪 测试步骤

### 步骤 1：重启 Zed
- 完全关闭 Zed
- 重新打开

### 步骤 2：打开测试文件

在 Zed 中打开：
```
D:\Projects\cangjie_lsp\tests\fixtures\sample.cj
```

或者你的项目：
```
D:\cangjie_repos\cangjie_demo\*.cj
```

### 步骤 3：验证功能

**检查 LSP 是否启动**：
- 查看 Zed 状态栏（右下角）
- 应该显示 LSP 服务器状态

**测试功能**：
- ✅ 左侧面板 → 文档大纲
- ✅ 悬停 → 类型信息
- ✅ Ctrl+Click → 跳转定义
- ✅ 输入 `fu` → 补全提示 `func`

---

## 📂 部署位置确认

```
C:\Users\zcg\AppData\Local\Zed\extensions\work\cangjie\
├── extension.toml          ✅ 已更新
├── extension.wasm          ✅ 已更新（265KB）
└── bin/
    └── cangjie-lsp.exe     ✅ 已复制（4.2MB）
```

---

## 🔧 如果还有问题

### 检查 Zed 日志

1. 在 Zed 中打开日志：
   - Windows: `Ctrl+Shift+P` → 输入 "log" → 选择 "Open Log"

2. 查找错误：
   - 搜索 `cangjie-lsp`
   - 查看 `ERROR` 或 `WARN` 行

3. 常见错误：

**"系统找不到指定的文件"**：
- 确认 `bin/cangjie-lsp.exe` 存在
- 确认路径没有拼写错误

**"协议错误"**：
- 我们的实现符合规范，不应该有这个问题

**"LSP 崩溃"**：
- 手动运行测试：
  ```bash
  cd C:\Users\zcg\AppData\Local\Zed\extensions\work\cangjie\bin
  .\cangjie-lsp.exe
  ```
  - 应该等待输入（说明正常）

---

## 📝 下次更新 LSP

如果你修改了 LSP 代码：

```bash
# 1. 重新构建 LSP
cd D:\Projects\cangjie_lsp
cargo build --release

# 2. 复制到扩展目录
cp target/release/cangjie-lsp.exe /c/Users/zcg/AppData/Local/Zed/extensions/work/cangjie/bin/

# 3. 重启 Zed
```

---

## ✅ 当前状态

- [x] LSP 服务器已构建（4.2MB）
- [x] 文件已复制到 Zed 扩展目录
- [x] extension.toml 配置正确（去掉.exe后缀）
- [x] extension.wasm 已更新
- [ ] **等待你重启 Zed 并测试**

---

**请重启 Zed，然后打开 .cj 文件测试！** 🚀
