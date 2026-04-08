# hulma

Project-aware [Claude Code](https://docs.claude.com/en/docs/claude-code) scaffolder. `hulma configure` inspects a project and writes review agents, slash commands, and hook templates into its `.claude/` directory.

The name is Tagalog for "mold" — hulma shapes a project's Claude Code surface.

## Install

```bash
brew tap dorky-robot/tap
brew install hulma
```

Or build from source:

```bash
git clone https://github.com/Dorky-Robot/hulma
cd hulma
cargo install --path .
```

## Usage

From the root of any project:

```bash
hulma configure
```

This launches Claude Code in the current directory. Claude inspects the project (reads README, CLAUDE.md, package.json / Cargo.toml / pyproject.toml / etc.) and writes tailored versions of:

- **Review agents** (`.claude/agents/`) — security, architecture, correctness, simplicity, root-cause, backlog triage
- **Slash commands** (`.claude/commands/`) — `/dispatch`, `/review`, `/triage`, `/ship-it`, `/work`, `/consult`, `/release`
- **Hook templates** (`.claude/hooks/` and `.husky/`) — pre-commit / pre-push gates, katulong pub/sub bridge

Re-run `hulma configure` whenever the project's shape changes meaningfully — new languages, new dependencies, new architecture.

### `--static`

If Claude Code isn't installed, or you just want generic templates without the discovery pass:

```bash
hulma configure --static
```

This installs the same templates without invoking Claude. Useful in CI, in containers, or when bootstrapping a project before Claude Code is on PATH.

## How it works

`hulma configure` does two things:

1. **Discovers the project** — scans the top-level directory for config files (`package.json`, `Cargo.toml`, `pyproject.toml`, `Makefile`, etc.), reads `README.md` and `CLAUDE.md`, and assembles a structured context block.
2. **Hands the context to Claude Code with a system prompt** — `lib/prompts/configure.md`, which includes every reference template in `templates/` so Claude has the full surface to work from. Claude then writes project-specific files to `.claude/`.

The reference templates in `templates/` are the source-of-truth set. Claude uses them as a starting point but is told to adapt: which agents matter for this project, which commands need the project's testing or deploy story baked in, which hooks make sense.

## Where it sits in the dorky robot stack

```
hulma (mold)  →  .claude/ files  →  used by sipag, kubo, katulong, claude-code
```

- `hulma` shapes a project's `.claude/` surface.
- `sipag` orchestrates work using those `.claude/` files as agents and commands.
- `kubo` runs sandboxed sessions inside the configured project.
- `katulong` exposes those sessions as a remote terminal.

`hulma` was extracted from `sipag` in April 2026 — see [`docs/ecosystem-refocus.md`](https://github.com/Dorky-Robot/dorky-robot/blob/main/docs/ecosystem-refocus.md) in the dorky-robot meta-repo.

## License

MIT
