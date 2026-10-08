---
paths:
  - "apps/web/src/**/*.tsx"
---

# React multi-step journeys

The journey owns the state; a screen only describes itself.

## State

- Journey state lives in a context.
- One context per lifetime, not per concern.
- The context carries state, screens carry wording.
- Title, step number and labels stay props.
- Never drill journey state through props.

## Access

- The access hook throws outside its provider.
- Never return a default journey.

## Navigation

- Derive navigation from a table.
- A missing entry disables the capability.

## Fields

- Preselect on answer change, never on mount.
- A mount effect overwrites deliberate choices.

## Transitions

- Move focus onto the new heading.
- Assert `document.activeElement` after every transition.
- Never let axe-core stand for that.
