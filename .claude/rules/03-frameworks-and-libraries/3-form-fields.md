---
paths:
  - "apps/web/src/**/*.{ts,tsx}"
---

# Form fields

A field is validated by the schema the API already publishes, and made accessible by the components that wrap it.

## Validation

- Validate with the Zod schema generated in `packages/api-client`, never a hand-written copy.
- Pass `revalidateLogic()` to every form: silent until the first submit, revalidated on every change after.
- Type drafts as `z.input`, never `z.output`: TanStack Form compares the validator's input, the normalized output is refused.

## Accessibility

- TanStack Form renders nothing; the field components in `components/ui/` wire the ARIA.
- They derive `htmlFor`, `aria-invalid` and `aria-describedby` from the field name alone.
- After a refusal, focus the first control marked `aria-invalid`, in DOM order, never in schema order.
- Show one message per field, the first: three refusals under one control is noise a screen reader reads whole.

## Choice fields

- Build choice fields on Base UI, never on a restyled native control.
- The trigger carries `aria-invalid` and `aria-describedby`; the label reaches it through `aria-labelledby`.
- A select emits `null` when nothing is chosen, never `""`: an untouched field then fails its schema.
- A long list to filter goes through a combobox, never a select.
- A radio group is a `<fieldset>` with a `<legend>`; its error hangs on the fieldset, never on a radio.
- Focus on a refused radio group goes to its checked radio, else its first.

## Wiring

- Build a form a component hands to another in a hook, `use-<x>-form.ts`.
- The child types what it receives as `ReturnType<typeof useXForm>`: no import cycle, none of `ReactFormExtendedApi`'s generics.
