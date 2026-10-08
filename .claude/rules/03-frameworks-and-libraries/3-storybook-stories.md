---
paths:
  - "apps/web/src/**/*.stories.tsx"
---

# Storybook stories

A story shows a component in the catalogue without freezing it, covering it, or drifting from it.

## Placement

- `title` is `<Group>/<Component>`, the component under its export name.
- The group comes from the closed `storySort.order` list of the Storybook preview.
- A new group goes into that list first.
- One export keeps one story name across every file.

## Docs

- The meta carries `parameters.docs.description.component`: when to use it, and what to use instead.
- Every story but the first carries a one-sentence `parameters.docs.description.story`.
- A props row showing `unknown`, a bare `union` or no type declares `table.type.summary`.
- A `cva` union lists its options, with its default in `table.defaultValue.summary`.

## Opening a surface

- Open with `defaultOpen`, `defaultValue` or `defaultChecked`.
- Never pass the controlled prop: the canvas freezes.

## Where the props hang

- Attach `argTypes` to the part that owns the props — `DialogContent`, `TabsList` — never the root.
- List every part mounted by hand under `subcomponents`.

## Isolation

- A `fixed inset-0` layer or a module singleton such as a toaster renders in its own iframe.
- Nothing covers or shares state across stories on the docs page.

## Variants

- Pin the `cva` variant options to the component's types.
- A variant added without a story then fails to compile.

## Data

- Fixed dates and frozen data only.
- Never `Date.now()`, never `Math.random()`.
