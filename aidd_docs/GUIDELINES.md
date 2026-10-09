# AI Operating Guidelines

How this team drives AI coding assistants on this project. Keep it short and specific to this repo. Fill the placeholders, drop what does not apply.

## House rules

- A failing test comes before any bug fix.
- Each plan phase ends in one atomic, intention-revealing commit.
- Ask before adding a Cargo crate or an npm package.

## Validation depth

- Docs or config only: `pnpm lint`. Any code change: every check in `memory/coding-assertions.md`.
- Before a merge: every coding assertion green, then a review with `aidd-dev:05-review`.

## When the AI drifts

- Reset the session and restate the objective in one sentence.

For the general AIDD playbook (planning, review loops, prompting and context hygiene, anti-patterns), see the framework docs: <https://github.com/ai-driven-dev/framework>.
