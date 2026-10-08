---
paths:
  - "apps/web/src/**/*.{ts,tsx}"
---

# Absorption belongs to the contract

A rejection is swallowed once, by the function that promises the outcome, never by its callers.

## Where it belongs

- One absorption, in the function that promises the outcome.
- Say it in the return type: `Promise<TOutcome | null>`.
- The same `.catch` repeated at call sites is a contract defect.

## Where it does not

- Never absorb what an `isError` alert already paints.
- Never absorb a redirect the router already navigates.

## What stays

- A call site outside the wrapper keeps its own, one line of why.
- A bare `catch {}` over a contract returning a typed reason is carrying, not blind.
