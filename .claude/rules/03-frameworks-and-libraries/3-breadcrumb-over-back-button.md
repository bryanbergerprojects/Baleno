---
paths:
  - "apps/web/src/{components,features,routes}/**/*.tsx"
---

# Breadcrumb over back button

Inside the authenticated layout the trail carries the way back, and nothing else does.

## Inside the authenticated layout

- Never add a « Back » button.
- Never add a back arrow either.
- A reachable level becomes a `BreadcrumbLink`.
- Declare every level in one navigation module.

## Tabs never enter the trail

- A tab is a view of its screen, never a level under it: the trail stops at the screen.
- Holds for every tab strip, present and future, with no exception.

## Outside the authenticated layout

- Only a screen with no parent level may carry a « Back » link: sign-in, OAuth callback errors.
- Name its destination: « Back to … ».
- Derive that destination, never guess it.
