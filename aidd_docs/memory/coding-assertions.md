# Coding Assertions

## Before commit

LeftHook also runs Biome on staged files and commitlint on the message.

| Order | Command     | Checks                |
| ----- | ----------- | --------------------- |
| 1     | `pnpm lint` | Biome format and lint |

## Before push

| Order | Command                                    | Checks                     |
| ----- | ------------------------------------------ | -------------------------- |
| 1     | `moon run :fmt`                            | rustfmt                    |
| 2     | `moon run :clippy`                         | Clippy, warnings as errors |
| 3     | `moon run :test`                           | nextest, then doctests     |
| 4     | `moon run :deny`                           | licenses, advisories, sources |
| 5     | `bash scripts/check-dependency-rule.sh`    | crate dependency rule      |

## Behavior

If a fix is needed, spawn 1 agent per assertion to fix (e.g typechecking / tests / rules violated on category UI = 3 agents).
