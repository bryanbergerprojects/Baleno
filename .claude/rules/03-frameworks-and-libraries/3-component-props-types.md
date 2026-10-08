---
paths:
  - "apps/web/src/**/*.tsx"
---

# Component props are a declared type

Every component names its props in a type declared above it. No component destructures an inline shape.

## Declaration

- Declare the type, never annotate the parameter inline.
- `({ step, title }: { step: number })` is a defect, whatever the component.
- Name it `<Component>Props`, matching the component exactly.
- Use `type`, never `interface` — `interface` is for module augmentation only.
- Export it whenever the component is exported.
- Keep it unexported when the component is local to its file.

## Members

- Mark every member `readonly`.
- Document a member on the member, never in the destructuring.
- Say what a prop means when the name does not: `busy`, `initialState`, `backLabel`.
- Extend a primitive by intersection: `ButtonPrimitive.Props & VariantProps<typeof buttonVariants>`.

## Placement

- The type sits between the imports and the component.
- The component docblock stays glued to the component, under the type.
- One component per file means one `Props` type per file.
