---
paths:
  - "apps/web/src/**/*.{ts,tsx}"
---

# Shared state lives in its provider

A context provider owns the state it publishes. Nothing above it holds that state on its behalf.

## Placement

- A feature context lives under `features/<context>/contexts/`, never beside the components that read it.
- An app-wide context, such as the theme, lives under `apps/web/src/contexts/`.
- One context per file, named `<thing>-context.tsx`.
- Its reducer sits beside it under the same stem, `<thing>-state.ts`, and its test beside that.

## Ownership

- State read by two components or more belongs to their provider.
- Never pass a provider its `value` as a prop.
- A component that only builds a `value` and renders a provider is a pipe — delete it.
- State one component reads stays a `useState` in that component.
- Publish the moves too, never the setters.

## Reducer

- Three `useState` or more moving together become a `useReducer`.
- One transition changing two fields or more is one action.
- The reducer is a pure function in its own `.ts`, with no React import.
- Name actions after what happened, never after what to set.
- Justify a reducer by atomicity and testability, never by performance — React batches either way.

## Props

- Props describe the screen: step number, heading, labels.
- Shared state and its moves come from the context.
- Never thread a context value through a prop.

## Drilling

- Drilling is never the reflex — it is the answer to two questions, both yes.
- Do several components read this value, and do they need it shared? Only then a context.
- A value crossing a component that never reads it is drilled, whatever the depth.
- Server data answers neither question — read its query options at the point of use.
- A context never holds a copy of the query cache.
- The loader's `ensureQueryData` keeps that cache warm, so reading at the leaf costs no request.
- Loader data no query owns stays a prop.

## What is left to carry

- What the cache cannot answer: which dialog is open, which form is submitting, which refusal a field carries.
- That is display state, and it is shared the moment a second component reads it.
- One screen, one such context — not one per dialog.
- A form still owns its field values: TanStack Form holds them, the context holds what the screen is showing.
- Publish a replayable gesture, never its dialog's state: a second opener would put two popups on one screen.

## Tests

- Test the reducer without a DOM, one transition at a time.
- Mount the real provider, never a stub of it.
- Open it on the state under test through a documented seeding prop.
- Assert on the state that came out, never on a stubbed callback.

## Import cycles

- Initial values shared by the reducer and the screens live in a third module.
- That module imports nothing from the components.
