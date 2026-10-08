---
paths:
  - "apps/web/src/**/*.{ts,tsx}"
---

# Memoization on proof only

Memoization enters the code only when a measured performance problem demands it.

- Never memoize by default.
- `useMemo`, `useCallback`, `memo` need a measurement.
- Build the value inline instead.
- Record the measurement beside the memoization.
- Drop memoization no measurement backs.
- shadcn files under `components/ui/` keep their shipped memoization.
