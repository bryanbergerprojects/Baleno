---
paths:
  - "apps/web/src/**/*.{ts,tsx}"
---

# Server state lives in the generated query options

Every server read and every server write goes through the options Hey API generates in `packages/api-client`.

## The layer

- A read passes the generated `*Options()` to `useQuery`, `useSuspenseQuery` or `ensureQueryData`.
- A write passes the generated `*Mutation()` to `useMutation`.
- No component or loader calls the generated SDK or `fetch` directly.
- Never hand-write a query hook the generated options already cover.

## Keys

- Read a key off the generated options' `queryKey`.
- Never write a key at a call site.

## Invalidation

- Every write invalidates the keys its answer left stale.
- Invalidate the closest key, never the whole cache.
- An empty invalidation is a decision, not an omission: say why in a line.

## Errors and expiry

- An expired session is handled once, in `lib/query-client.ts`, never per component.
- API errors are decoded once, in `lib/api-error.ts`.
- No component branches on a raw HTTP status.

## Not server state

- URL state stays on `validateSearch` and Zod.
- State two components share stays in their provider — `3-shared-state-in-providers.md`.
- No global store.
