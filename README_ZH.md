# Cangjie LSP - Zed 配置说明

## 快速配置（3步）

### 第1步：在 Zed 中打开设置

快捷键：`Ctrl + ,`

### 第2步：添加 LSP 配置

在 `settings.json` 中添加：

```json
{
  "lsp": {
    "cangjie-lsp": {
      "binary": {
        "path": "D:\Projects\cangjie_lsp\target\release\cangjie-lsp.exe"
      }
    }
  }
}
```

**注意**：路径使用双反斜杠 `\` 或正斜杠 `/`

### 第3步：重启 Zed

- 完全关闭 Zed
- 重新打开
- 打开 `.cj` 文件测试

---

## 支持的功能

- ✅ 文档大纲（Document Symbols）
- ✅ 悬停提示（Hover）
- ✅ 跳转定义（Go to Definition）
- ✅ 代码补全（Completion）

---

## 测试文件

使用这个文件测试：
`D:\Projects\cangjie_lsp\tests\fixtures\sample.cj`
