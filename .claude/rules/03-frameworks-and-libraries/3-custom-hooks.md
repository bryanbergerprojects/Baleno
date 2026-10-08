---
paths:
  - "apps/web/src/**/*.{ts,tsx}"
---

# Reusable gestures are custom hooks

A gesture a second screen will repeat leaves the component it was written in and becomes a hook.

## What becomes a hook

- Stateful behaviour a second screen will repeat: clipboard, focus, polling.
- A `useState` sequence describing a technical gesture, not a screen.
- Not a one-off local state: that stays a `useState` in its component.
- Not a pure function: that is a `lib/` module, no React import.

## Placement

- Domainless, no API call: `apps/web/src/hooks/`.
- Calls the API, or knows a context's domain: `features/<context>/hooks/`.
- One hook per file, named `use-<thing>.ts`, exporting `use<Thing>`.

## Boundary

- A domainless hook never imports the generated API client.
- A hook never reaches into another feature.

## Return

- Return a named, exported, fully `readonly` type.
- Return failure _reasons_, never sentences — wording differs per screen and belongs there.
- Carry a rate-limit wait as a number, told apart from a plain refusal.
- Publish the moves, never the setters.

## Tests

- Test the hook alone, no screen mounted.
- One test per answer the API can give, refusals included.
- Assert on what the hook returned, never on a stubbed callback.
- What a screen makes of a `reason` is that screen's test.
