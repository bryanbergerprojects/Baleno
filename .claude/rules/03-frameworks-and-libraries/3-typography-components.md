---
paths:
  - "apps/web/src/**/*.tsx"
---

# Typography comes from its components

A size and a colour written on a text element are a component that already exists.

## The five

- `Heading` from `components/ui/heading`.
- `Paragraph`, `Lead`, `Muted`, `Small` from `components/ui/text`.
- `Muted` is `text-sm text-muted-foreground`.
- `Small` is `text-xs text-muted-foreground`.
- `Lead` opens a screen, `Paragraph` carries its flow.

## Using them

- `level` picks the type scale, `as` picks the tag.
- Match the token to the size on screen, never to the rank.
- Keep spacing, layout and state classes in `className`.
- Swap the rendered element with `render`, never a wrapper.

## When none fits

- Compose the closest one with a `className`.
- Never restate a class it already carries.
- A `font-mono` block is code, never typography.
- A pair a third screen repeats earns its own component.
