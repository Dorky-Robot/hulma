// Embedded template files installed by `sipag configure`.

// Agents
pub const AGENT_SECURITY_REVIEWER: &str =
    include_str!("../templates/agents/security-reviewer.md");
pub const AGENT_ARCHITECTURE_REVIEWER: &str =
    include_str!("../templates/agents/architecture-reviewer.md");
pub const AGENT_CORRECTNESS_REVIEWER: &str =
    include_str!("../templates/agents/correctness-reviewer.md");
pub const AGENT_BACKLOG_TRIAGER: &str =
    include_str!("../templates/agents/backlog-triager.md");
pub const AGENT_ISSUE_ANALYST: &str = include_str!("../templates/agents/issue-analyst.md");
pub const AGENT_ROOT_CAUSE_ANALYST: &str =
    include_str!("../templates/agents/root-cause-analyst.md");
pub const AGENT_SIMPLICITY_ADVOCATE: &str =
    include_str!("../templates/agents/simplicity-advocate.md");

// Skills (formerly "commands" — Claude Code merged commands into skills, where
// each lives in its own directory with a SKILL.md entrypoint. Constant names
// keep the SKILL_ prefix to stay aligned with the new terminology.)
pub const SKILL_DISPATCH: &str = include_str!("../templates/skills/dispatch/SKILL.md");
pub const SKILL_REVIEW: &str = include_str!("../templates/skills/review/SKILL.md");
pub const SKILL_TRIAGE: &str = include_str!("../templates/skills/triage/SKILL.md");
pub const SKILL_SHIP_IT: &str = include_str!("../templates/skills/ship-it/SKILL.md");
pub const SKILL_WORK: &str = include_str!("../templates/skills/work/SKILL.md");
pub const SKILL_CONSULT: &str = include_str!("../templates/skills/consult/SKILL.md");
pub const SKILL_CONSULT_NO_DIWA: &str =
    include_str!("../templates/skills/consult-no-diwa/SKILL.md");
pub const SKILL_RELEASE: &str = include_str!("../templates/skills/release/SKILL.md");

// Git hooks
pub const GIT_HOOK_PRE_COMMIT: &str = include_str!("../templates/git-hooks/pre-commit");
pub const GIT_HOOK_PRE_PUSH: &str = include_str!("../templates/git-hooks/pre-push");

// Claude Code hooks
pub const HOOK_KATULONG_PUBSUB: &str = include_str!("../templates/hooks/katulong-pubsub.sh");

// Settings template
pub const SETTINGS_LOCAL: &str = include_str!("../templates/settings.local.json");
