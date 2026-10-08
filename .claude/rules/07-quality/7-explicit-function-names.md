---
paths:
  - "apps/web/**/*.{ts,tsx}"
---

# Function names say what they do

A function name states the act it performs. A reader who has only the call site knows what happens.

## Banned alone

- `start`, `finish`, `run`, `go`, `handle`, `process`, `doIt`, `execute`.
- Any name that describes a position in a sequence rather than an act.
- Any name that would fit half the functions in the file.

## Shape

- Verb plus its object: `cancelRun`, `openPreview`, `downloadTextFile`.
- A technical prefix is acceptable only followed by the domain verb: `runTaskCreation`, never `run`.
- Event handlers are `handle<What>`: `handleSubmit`, never `handle`.
- Booleans read as a claim: `previewReady`, `cancellingRun`.
- Same rule for hook members, callbacks in props, and variables holding a function.

## Scope

- Applies to a two-line local function as much as to an exported one.
- A name needing a comment to be understood is the wrong name — rename, then keep the comment for the why.
