# VCS

## Setup

- Main branch: `main`
- Platform: GitHub

## Branches

- Format: `type/<issue-number>` when the work starts from a GitHub issue, `type/short-description` otherwise.
- Types in use: the commit types below.

## Commits

- Convention: Conventional Commits, enforced by commitlint (`commitlint.config.ts`).
- Format: `type(scope): description`
- Rules: header 72 characters max, type and scope included; types limited to `build`, `chore`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `test`.

## Commit Strategy

AI should auto commit: `after phase`
