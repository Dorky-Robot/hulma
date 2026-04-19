# hulma

hulma is a project-aware Claude Code scaffolder. `hulma configure` inspects a project and writes review agents, skills (slash commands), and hook templates to its `.claude/` directory.

Claude Code discovers skills as `.claude/skills/<name>/SKILL.md` directories. Each SKILL.md must start with YAML frontmatter containing both `name:` and `description:`. The legacy `.claude/commands/*.md` layout still runs at invocation time but is invisible to the autocomplete menu, so hulma writes the directory layout exclusively.

## Architecture

- `src/main.rs` — entry point, parses CLI, dispatches to `cli::run`
- `src/cli.rs` — clap parser, single `Configure` subcommand
- `src/configure_project.rs` — `run_configure()`, project discovery, prompt builder, template installer
- `src/templates.rs` — `include_str!` constants for embedded template files
- `templates/agents/*.md` — review-agent reference templates (security, architecture, correctness, simplicity, root-cause, backlog, issue analyst)
- `templates/skills/<name>/SKILL.md` — skill (slash-command) reference templates (dispatch, review, triage, ship-it, work, consult, consult-no-diwa, release)
- `templates/git-hooks/{pre-commit,pre-push}` — husky-style git hook templates
- `templates/hooks/{katulong-pubsub.sh,safety-gate.sh,safety-gate.toml,README.md}` — Claude Code hook templates
- `templates/settings.local.json` — `.claude/settings.local.json` reference
- `prompts/configure.md` — system prompt for the generative configure pass

All templates are embedded via `include_str!` in `src/templates.rs`. Hulma is a single static binary with zero runtime filesystem lookups for templates. The build will fail if any referenced template is missing.

## How `configure` works

Two paths:

1. **Generative (default)** — `claude_available()` returns true. `discover_project()` builds a structured context block from the directory listing, config files, README, and CLAUDE.md. Hulma execs `claude --append-system-prompt <prompt> <initial_message>`, replacing process so Claude takes over the terminal. Claude writes project-tailored files to `.claude/`.

2. **Static** (`--static` or claude not on PATH) — `install_static_templates()` writes the reference templates verbatim. Used in CI, containers, or when Claude Code isn't available.

The diwa-aware variant of `/consult` is selected at install time based on whether `diwa` is on PATH. The static path checks via `diwa_available()`; the generative path bakes the chosen variant into the system prompt so the static and generative outputs stay consistent on the same machine.

## Development

```bash
cargo build               # debug build
cargo test                # run all tests (including the placeholder test that catches missing template references)
cargo install --path .    # install hulma to ~/.cargo/bin
```

The placeholder test (`build_configure_prompt_replaces_all_placeholders`) is the canary for template-surface drift: if you add a new `{SKILL_*}` or `{AGENT_*}` token to `prompts/configure.md` without wiring up a corresponding `replace()` call in `build_configure_prompt()`, this test fails.

## Adding a template

1. Drop the file under `templates/agents/`, `templates/skills/<name>/SKILL.md`, or `templates/hooks/`.
2. Add a `pub const X: &str = include_str!("../templates/...")` constant in `src/templates.rs`.
3. Add a `TemplateFile` entry to the appropriate table in `src/configure_project.rs` (`TEMPLATES`, `CLAUDE_HOOK_TEMPLATES`, `HOOK_TEMPLATES`). For skills, the `relative_path` must be `skills/<name>/SKILL.md` so it matches Claude Code's discovery layout.
4. If the template should be referenced in the generative prompt, add a `{X}` placeholder to `prompts/configure.md` and a `.replace("{X}", templates::X)` call in `build_configure_prompt()`.
5. Run `cargo test` — the placeholder test will catch any missed wiring.

## Where hulma sits

hulma was extracted from `sipag` in April 2026, splitting the scaffolder concern out from the work-dispatcher concern. See `docs/ecosystem-refocus.md` in the dorky-robot meta-repo for the full reasoning.

- `hulma` shapes a project's `.claude/` surface.
- `sipag` is the work dispatcher that orchestrates Claude Code sessions in those configured projects.
- `kubo` is the sandbox that runs each session.
- `katulong` is the remote terminal that exposes those sessions.

The boundary: hulma writes files, sipag runs work. They do not share runtime state.

## Conventions

- **Single binary**, zero external runtime dependencies. All templates embedded via `include_str!`. No `~/.hulma`, no config files, no daemons.
- **Static-first**: every change must keep `--static` working. The generative path is a layer on top, not a replacement.
- **Templates are source-of-truth**, not Claude output. The reference templates in `templates/` are deliberate, reviewed, and live in git. Generative output is allowed to deviate from them, but the static path ships these exact files.
- **No `unwrap()` in `run_configure()`** — every fallible operation returns a `Result` with `anyhow::Context`. The `unwrap()` calls in tests are fine.
