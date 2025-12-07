# VibeKit Default Guidelines

## 1. Context Awareness

**Read in order:**

1. `README.md` - Project overview
2. `docs/TECH_STACK.md` - Available libraries (**DO NOT** add new without asking)
3. `docs/TODO.md` - Current tasks/phases
4. `docs/*.md` - Related docs

Understand project structure before making changes.

## 2. Efficiency

- **Concise** - No verbose explanations or repetition
- **Skip obvious** - Don't explain what code clearly shows
- **Batch updates** - Group changes, minimize back-and-forth

## 3. Reporting

- **Changes only** - Report what changed, not what exists
- **No redundancy** - Don't repeat content from .md files
- **Update tracking** - Update `TODO.md` directly

## 4. Quality

- Follow existing code style and patterns
- Include error handling and edge cases
- Self-documenting code with clear naming
- **Modular** - Break into independent parts
- **Tests** - Write tests for critical logic
- **Docs** - Update when changing behavior
- **Comments** - Explain complex logic and decisions
- **Changelog** - Update `CHANGELOG.md` with changes
- **Files** - Docs → `docs/` (except `README.md`, `CHANGELOG.md`)

## 5. Workflow

- **Project:** Read context → Plan → Implement → Test → Document
- **Task:** Read → Plan → Code → Verify → Update

## 6. Communication

- Ask **ONE** question if blocked
- Suggest alternatives when unclear
- Confirm destructive operations

## 7. Safety

- **Backup** - Commit before major changes
- **Incremental** - Small changes, avoid big-bang
- **Dependencies** - Reuse existing utils, avoid duplicates
- **Breaking changes** - Document and update all callers
- **Failure recovery** - Summarize progress if task fails
