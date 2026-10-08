---
paths:
  - "apps/web/**/*.{ts,tsx}"
---

# Code in English, screens in French and English

Everything a visitor reads exists in French and English. Everything else — identifiers, paths, comments — is English.

## Routes

- URL segments are code, English only: `/tasks`, `/environments`, `/settings`.
- The route file is named after its path: `/tasks` is `routes/_app/tasks/`.
- A route never carries the language: one URL serves both.

## Everything else in English

- Identifiers, types, files, directories.
- Comments and docblocks.
- Test names and the paths a test invents.

## Rendered text

- Never hard-code rendered text in a component.
- Every string goes through the i18n catalogue: headings, labels, placeholders, buttons, refusals, page titles.
- A key ships with its French and its English value, never one alone.
- Agent output, diffs and repository content render as received, never translated.

## Renaming

- Renaming a route means renaming its file, regenerating `routeTree.gen.ts`, and following the path through its tests.
- Grep the old segment, not just the path — a comment naming the old route is a stale comment.
