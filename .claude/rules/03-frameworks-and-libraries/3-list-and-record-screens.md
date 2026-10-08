---
paths:
  - "apps/web/src/**/*.tsx"
---

# List and record screens

Every collection — tasks, environments, repositories, audit entries — wears the same screen, and that screen never grows a control at runtime.

## Nothing appears along the way

- A control is in the DOM at the first render, or it does not exist.
- It changes state; it never surges.
- Never on hover, never on the first keystroke, never past a volume threshold.
- An empty table keeps its search, its filters, its headers and its pagination bar.
- A block that only exists once its first row does is the defect this rule names.

## The table

- The shared `DataTable` in `components/` is a render engine — it holds no state of its own.
- Sort, page and filters live in the URL, read by `validateSearch` and Zod.
- The API sorts, pages and filters, and answers the rows with their total.
- The sort parameter is a `z.enum` of the sortable columns, never a free string.
- The row is clickable through a real `a[href]` under its primary cell, so the keyboard and a new tab both work.
- The `⋮` column is permanent, fixed width, last in the row.
- The table spans the full width of its screen.

## The empty state

- An empty body asks two questions, each with one answer for every collection.
- « Is the collection empty? » — an unfiltered count, its own request beside the listing.
- Never re-derive it from the filter values: a page out of bounds then reads as unfiltered and lies.
- « Is the page out of bounds? » — the loader compares its page to the total and redirects with `replace`.
- `DataTable` clamps the page it displays, never the one it was given.

## The filters

- At least one filter as soon as a business axis exists.
- The rule forces you to look for one; it never forces you to invent one.
- No axis, no filter — and say so, rather than shipping a decorative select.
- A filter is a permanent control with a default value, never a checkbox that reveals a subset.
- The screen renders search and filters above `DataTable`, never inside it.

## The creation

- A modal, opened from the screen — never a route of its own.
- One form, one submit; no wizard for a single record.
- On success, navigate to the created record.
- The creation action belongs to the screen, never to `DataTable`.

## The record

- Its own page, outside the tabbed layout the list lives in.
- Data reads as tiles, one per data group — never a permanent form, never an « Edit » button for the page.
- Each group edits in its own dialog, prefilled, writing only its own fields.
- An empty datum reads as not set, with a pencil that opens its dialog.
- A filled tile copies its value through a named copy button and confirms in a toast.
- The status is a badge above the name, never a field.
- Lifecycle acts — archive, stop, delete — live in the `⋮`, never as buttons in the flow.
- Cap the content around 768 px: a tile read edge to edge is read twice.
- The breadcrumb carries the record's name, loaded, never a static label.

## The vocabulary

- Name the domain term, singular: Task, Environment, Repository.
- The same word in the menu, the breadcrumb, the tab, the heading and the empty state.
- A screen that renames a thing halfway teaches two names for one thing.
