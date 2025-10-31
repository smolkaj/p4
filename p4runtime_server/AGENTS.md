# Agent Guidelines

## Critical Thinking

Question and challenge every suggestion before executing:
- Are we solving the right problem?
- Is there a better approach?
- Could this lead to technical debt?
- Voice concerns rather than blindly following suggestions.

For non-trivial tasks, explore alternatives first:
- Document several approaches with pros, cons, and tradeoffs in a markdown file.
- Consider architectural implications before diving into implementation.

## Assumptions

Never assume what you cannot guarantee locally. Handle assumptions in this order:

1. **Type system** - Capture in types (unless overly complex or hurts readability).
2. **Runtime checks** - Validate and return errors (unless too noisy or inefficient).
3. **Refactor** - Restructure to enable (1) or (2).
4. **Assertions** - Use `assert!`/`assert_eq!` to document invariants.
5. **Comments** - Document assumptions explicitly.

## Code Style

- **Simplicity over cleverness** - Optimize for readers, not writers.
- **Idiomatic Rust** - Follow established patterns and conventions.
- **Global consistency** - Prefer codebase-wide consistency over local optimizations.
- **Error messages** - Make them descriptive and actionable. What did the code assume? What happened instead?
- **Avoid brittleness** - Write code that remains correct when requirements change.
- **Defensive programming** - Consider edge cases and failure modes.
- **Testability** - Write code that's easy to test and verify.

## Naming

- **Descriptive, discriminative names** - Avoid generic names; coin new terms if needed and define them.
- **Consistent file/struct naming** - Files and their core structs/concepts should have consistent, matching names.
- **Name length matches scope** - Short-lived variables can (and typically should) have short names when context is clear (e.g., `translator` for `P4RuntimeP4rsTranslator`). Core structs/concepts should never use indiscriminate names.
- **Names describe roles, not types** - Variable names should explain the variable's role in context, not merely repeat its type (unless there's nothing else useful to say).

