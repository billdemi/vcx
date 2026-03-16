# VCX — Vibe Coding 上下文管理器

> 跨工具检测、同步和管理 AI 编程工具的上下文指令文件。

VCX 是 **AI 上下文文件的 ESLint**。它能检测 Claude Code、Cursor、Copilot、Windsurf、Aider 等工具配置中的冲突、过时引用和重复规则。

## 解决什么问题

每个 AI 编程工具都有自己的上下文/指令文件，但没人管理它们：

| 工具 | 上下文文件 |
|------|-----------|
| Claude Code | `CLAUDE.md` |
| Cursor | `.cursor/rules/*.mdc` |
| GitHub Copilot | `.github/copilot-instructions.md` |
| Windsurf | `.windsurfrules` |
| Aider | `.aider.conf.yml` |

当你同时使用多个工具时，问题就来了：
- **冲突**：一个文件写着"用 Tab 缩进"，另一个写着"用 2 空格缩进"
- **过时引用**：指令引用了已经不存在的文件
- **重复**：相同的规则在多个工具配置里复制粘贴
- **不可见**：你不知道每个工具到底被告知了什么

## 安装

```bash
cargo install vcx
```

或从源码编译：

```bash
git clone https://github.com/billdemi/vcx.git
cd vcx
cargo build --release
# 二进制文件在 target/release/vcx
```

## 使用方法

### scan — 发现所有上下文文件

```bash
vcx scan
```

```
Tool           File                            Size   Modified   Status
──────────────────────────────────────────────────────────────────────────
Claude Code    CLAUDE.md                       2.4K   2h ago     ok
Cursor         .cursor/rules/typescript.mdc    1.1K   5d ago     ok
Copilot        .github/copilot-instructions.md 1.8K   2w ago     stale?
Windsurf       (not found)                     -      -          missing
```

### lint — 检测问题

```bash
vcx lint
```

```
.cursor/rules/typescript.mdc:10  error  conflict/style
  缩进冲突："Use tabs" vs "Use 2-space indentation"
    CLAUDE.md:5  冲突指令位置

CLAUDE.md:16  error  conflict/tooling
  包管理器冲突："pnpm" vs "yarn"
    .cursor/rules/typescript.mdc:10  冲突指令位置

CLAUDE.md:47  warn  stale/file-ref
  引用了不存在的文件 "src/utils/legacy.ts"

3 problem(s) (2 error, 1 warning, 0 info)
```

发现错误时返回退出码 1，适合 CI 集成。

### status — 健康仪表盘

```bash
vcx status
```

```
VCX Context Health Report
═════════════════════════
Tools configured:  3 (Claude Code, Copilot, Cursor)
Tools missing:     3 (Windsurf, Aider, Codex)
Total rules:       23 (18 unique, 5 duplicated)
Lint:              2 error, 1 warning, 0 info
Health score:      55/100  (█████░░░░░)
```

### init — 初始化配置

```bash
vcx init --tools claude,cursor,copilot
```

创建 `.vcx.toml` 配置文件，并为选定的工具生成模板上下文文件。

## Lint 规则

| 规则 | 严重度 | 说明 |
|------|--------|------|
| `conflict/style` | error | 风格指令冲突（Tab vs 空格、引号风格等） |
| `conflict/tooling` | error | 工具偏好冲突（pnpm vs yarn、jest vs vitest） |
| `stale/file-ref` | warn | 引用了不存在的文件 |
| `stale/tool-format` | warn | 使用了废弃格式（如 `.cursorrules`） |
| `duplicate/exact` | warn | 多个文件中存在完全相同的规则 |
| `duplicate/cross-tool` | info | 同一规则出现在不同工具的配置中 |
| `format/encoding` | error | 文件编码不是 UTF-8 |
| `breadth/too-large` | warn | 上下文文件超过 5KB |

## 配置

在项目根目录创建 `.vcx.toml`：

```toml
[vcx]
version = "0.1"
tools = ["claude", "cursor", "copilot", "windsurf", "aider"]
source_of_truth = "claude"

[lint]
disable = ["duplicate/cross-tool"]

[lint.severity]
"breadth/too-large" = "error"
```

## JSON 输出

所有命令支持 `--format json`，方便程序化使用：

```bash
vcx scan --format json
vcx lint --format json
```

## 路线图

- **v0.2**：`vcx diff`（跨工具对比）、`vcx sync`（同步规则）、`vcx clean`（自动修复）
- **v0.3**：语义分析、模糊去重、GitHub Actions 集成
- **v1.0**：插件系统、LSP 服务器、MCP 工具集成

## 开源协议

MIT
