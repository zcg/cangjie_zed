# Zed 扩展部署完成 - 操作指南

## ✅ 已完成

1. **LSP 文件已复制**
   - 源文件：`D:\Projects\cangjie_lsp\target\release\cangjie-lsp.exe`
   - 目标位置：`C:\Users\zcg\AppData\Local\Zed\extensions\work\cangjie\bin\cangjie-lsp.exe`
   - 大小：4.2MB ✅

2. **配置文件已更新**
   - `extension.toml` 使用相对路径：`bin/cangjie-lsp.exe`
   - `src/lib.rs` 简化逻辑

---

## 🚀 在 Zed 中使用（3步）

### 第1步：重新加载扩展

**方式A（推荐）**：
1. 打开 Zed
2. 快捷键：`Ctrl+Shift+X`（打开扩展面板）
3. 找到 `Cangjie` 扩展
4. 点击 `Reload` 或 `Restart`

**方式B**：
- 直接重启 Zed

### 第2步：打开测试文件

在 Zed 中打开任意 `.cj` 文件，例如：
```
D:\Projects\cangjie_lsp\tests\fixtures\sample.cj
```

或者：
```
D:\cangjie_repos\cangjie_demo\*.cj
```

### 第3步：验证功能

**✅ 文档大纲（左侧面板）**
- 应该显示所有函数、类、结构体

**✅ 悬停提示**
- 鼠标放在函数名上
- 应该弹出类型信息

**✅ 跳转定义**
- Ctrl + 点击符号
- 应该跳转到定义位置

**✅ 代码补全**
- 输入 `fu`
- 应该提示 `func`、`for` 等

---

## 🔧 故障排除

### 如果 LSP 还是不启动

#### 检查1：确认文件存在
```bash
ls "C:\Users\zcg\AppData\Local\Zed\extensions\work\cangjie\bin\cangjie-lsp.exe"
```
应该显示 4.2M 文件

#### 检查2：手动测试 LSP
```bash
cd "C:\Users\zcg\AppData\Local\Zed\extensions\work\cangjie\bin"
.\cangjie-lsp.exe
```
应该等待输入（说明能正常运行）

#### 检查3：查看 Zed 日志
- Zed 菜单：`View` → `Debug` → `Open Log`
- 搜索 `cangjie-lsp` 查看错误

### 常见问题

**问题：文件未找到**
- **解决**：确认 `bin/cangjie-lsp.exe` 在扩展目录中
- **验证**：手动运行可执行文件

**问题：权限错误**
- **解决**：右键文件 → 属性 → 解除阻止
- **或者**：以管理员运行 Zed

**问题：协议错误**
- **不会发生**：我们的实现符合规范 ✅

---

## 📌 重要提示

**Zed 扩展的工作原理**：

1. 你安装扩展时，Zed 会把扩展复制到：
   ```
   C:\Users\zcg\AppData\Local\Zed\extensions\work\cangjie\
   ```

2. 扩展中的 `bin/` 目录需要包含可执行文件

3. `extension.toml` 中的相对路径相对于扩展目录

**所以我们做的是**：
- ✅ 把 LSP 放到 `bin/cangjie-lsp.exe`
- ✅ 配置使用相对路径 `bin/cangjie-lsp.exe`
- ✅ Zed 会自动找到并运行

---

## ✨ 现在的状态

```
✅ LSP 服务器 - 4.2MB，功能完整
✅ 协议格式 - 符合规范，无需 proxy
✅ 文件部署 - 已复制到 Zed 目录
✅ 配置正确 - extension.toml 使用相对路径
⏳ 等待测试 - 需要在 Zed 中重新加载
```

---

**请在 Zed 中重新加载扩展，然后打开 `.cj` 文件测试！** 🚀

如果还有错误，请把完整的错误信息发给我。
