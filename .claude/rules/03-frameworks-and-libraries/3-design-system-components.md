---
paths:
  - "apps/web/src/**/*.tsx"
---

# Components come from the design system

A hand-written component is the last resort, never the first move.

## Before writing one

- Search `components/ui/` first.
- Then the shadcn/ui registry, then Base UI.
- Install it with the `shadcn` CLI, into `components/ui/`.
- Never copy a primitive by hand.

## Writing one anyway

- Only when the feature truly requires it.
- Compose installed primitives before styling anything.
- Never fork a primitive into a feature folder.
- Say in the docblock what was missing.
