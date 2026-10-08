---
paths:
  - "apps/web/**/*.{ts,tsx}"
  - "crates/**/*.rs"
  - "guest/**"
---

# Comments carry the why

The default is no comment. Code says what it does; a comment is the exception that earns its place.

## Write none

- Start from zero comments, always.
- Rename until the code reads itself.
- Extract a named function before explaining a block.
- Name a constant before explaining a value.
- A file with no comment is the normal file, not a suspicious one.

## The exception

- Only genuinely complex code may carry one.
- A third-party bug being worked around.
- A legal or security constraint — the ceiling holds, the reference goes to `aidd_docs/memory/external/`.
- A counter-intuitive algorithm or ordering.
- An option rejected, and why.
- Two lines at most, and one is better.

```ts
// A session lock, not a PID-suffixed database: nothing ever drops those.
await client.query('SELECT pg_advisory_lock($1)', [LOCK_KEY])
```

## Delete

- Anything restating the code below.
- A docblock a typed signature already states.
- A file header narrating phases, specs, or history.
- A note on wording the rendered text already carries.
- A geometry or styling essay on a class name.
- `Decorative` beside an `aria-hidden` that says it.
- A debug session, its date, its logs.
- A summary of what just changed.

## Longer rationale

- Past three lines, write `aidd_docs/`.
- Source keeps a pointer, not the story.
- A docblock is a comment: the ceiling holds there too, `///` and `//!` included.

## Elsewhere

- Naming: `07-quality/7-explicit-function-names.md`.
- Language: `07-quality/7-english-code-bilingual-screens.md`.
