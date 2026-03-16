# VCX — Vibe Coding conteXt Manager

> Lint, sync, and manage AI coding context files across all major tools.

VCX is the **ESLint for AI context files**. It detects conflicts, stale references, and duplicates across your AI coding tool configurations — Claude Code, Cursor, Copilot, Windsurf, Aider, and more.

## The Problem

AI coding tools each have their own context/instruction files, but nobody manages them:

| Tool | Context File |
|------|-------------|
| Claude Code | `CLAUDE.md` |
| Cursor | `.cursor/rules/*.mdc` |
| GitHub Copilot | `.github/copilot-instructions.md` |
| Windsurf | `.windsurfrules` |
| Aider | `.aider.conf.yml` |

When you use multiple tools, problems emerge:
- **Conflicts**: "Use tabs" in one file, "Use 2-space indent" in another
- **Stale references**: Instructions pointing to files that no longer exist
- **Duplicates**: Same rules copy-pasted across tool configs
- **No visibility**: You don't know what each tool is being told

## Install

```bash
cargo install vcx
```

Or build from source:

```bash
git clone https://github.com/billdemi/vcx.git
cd vcx
cargo build --release
# Binary at target/release/vcx
```

## Usage

### Scan — Discover all context files

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

### Lint — Find problems

```bash
vcx lint
```

```
.cursor/rules/typescript.mdc:10  error  conflict/style
  Conflicting indentation: "Use tabs" vs "Use 2-space indentation"
    CLAUDE.md:5  Conflicting instruction here

CLAUDE.md:16  error  conflict/tooling
  Conflicting package manager: "pnpm" vs "yarn"
    .cursor/rules/typescript.mdc:10  Conflicting preference here

CLAUDE.md:47  warn  stale/file-ref
  References "src/utils/legacy.ts" which does not exist

3 problem(s) (2 error, 1 warning, 0 info)
```

Exit code 1 when errors are found — CI-friendly.

### Status — Health dashboard

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

### Init — Bootstrap config

```bash
vcx init --tools claude,cursor,copilot
```

Creates `.vcx.toml` config and template context files for selected tools.

## Lint Rules

| Rule | Severity | Description |
|------|----------|-------------|
| `conflict/style` | error | Contradictory style instructions (tabs vs spaces, quotes, etc.) |
| `conflict/tooling` | error | Contradictory tool preferences (pnpm vs yarn, jest vs vitest) |
| `stale/file-ref` | warn | References to nonexistent files |
| `stale/tool-format` | warn | Deprecated formats (e.g., `.cursorrules`) |
| `duplicate/exact` | warn | Identical rules in multiple files |
| `duplicate/cross-tool` | info | Same rule across different tools |
| `format/encoding` | error | Non-UTF-8 files |
| `breadth/too-large` | warn | Context file exceeds 5KB |

## Configuration

Create `.vcx.toml` in your project root:

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

## JSON Output

All commands support `--format json` for programmatic use:

```bash
vcx scan --format json
vcx lint --format json
```

## Roadmap

- **v0.2**: `vcx diff` (cross-tool comparison), `vcx sync` (synchronize rules), `vcx clean` (auto-fix)
- **v0.3**: Semantic analysis, fuzzy duplicate detection, GitHub Actions integration
- **v1.0**: Plugin system, LSP server, MCP tool integration

## License

MIT
