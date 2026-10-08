---
paths:
  - "apps/web/src/**/*.tsx"
---

# One component per file

A file declares one component, and the folder tree draws the JSX tree above it.

## Layout

- One component, one file, named after it in kebab-case.
- A component with sub-components becomes a folder.
- `index.tsx` carries the main component of its level.
- Its children sit beside it, one file each — one folder per nesting level.
- A `.ts` module only that level reads is a child too.
- A component with no sub-component stays a flat file.

## Promotion

- One reader, one branch, no promotion — the component goes down into that branch.
- A component two branches read moves up to the smallest folder both readers sit under.
- It takes its children with it: they become the folder it opens, never its neighbours at the root.
- A component the feature's provider mounts sits at the feature root: it has no JSX parent inside the folder.
- A feature never imports another: a component two features read goes to `components/`, with no API knowledge.

## Shared declarations

- A type both files need lives in a third module.
- Never import the main component from a sub-component.

## Exception

- `components/ui/` may group several components in one file.
- Only when they serve the same purpose — `Tabs`, `Tooltip`, the form controls.
- A component serving another purpose gets its own file.
