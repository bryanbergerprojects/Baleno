---
paths:
  - "apps/web/src/**/*.tsx"
---

# One responsibility, composed

A component does one thing and delegates the rest. A screen is a composition, never a single body.

## Routes

- A route file holds its `createFileRoute`, its `validateSearch`, its loader and its `head`.
- Its component reads params and loader data, then renders one page component.
- Never branch on the record's content in a route file.
- The page component lives under `features/<context>/components/`.

## Splitting a screen

- A heading with its controls is a component.
- A description list is a component.
- A dialog is a component, confirmation wording included.
- A repeated row of a list is a component.
- A block of fields the schema already cuts apart is a component.
- A sub-component is one file, placed under the component that reads it — `3-one-component-per-file.md`.

## What stays at the top

- Server calls, their error handling and the state they set.
- The state two sub-components read.
- Which of the screen's states is showing.
- Sub-components name gestures back through `on<What>` props.

## Duplication

- Extract on the second call site, never on the first.
- Two blocks that only look alike stay apart.
- A shared block takes its data as props, never through `useParams`.

## Wrong reasons to refuse

- Three states of one screen still compose into sub-components.
- Eight lines is not too few — eight lines twice is the defect.
- `3-one-component-per-file.md` prescribes where sub-components live, not that there are none.
- A future screen wanting its own summary does not excuse this one.
- Never cite a rule to keep a body long.
