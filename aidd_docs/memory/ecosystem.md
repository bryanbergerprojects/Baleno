# Ecosystem

```mermaid
flowchart LR
  Agent([Agent])
  Vcs["GitHub repo · vcs.md"]
  Tracker["GitHub Issues + Project · backlog.md"]

  Agent -- cli --> Vcs
  Agent -- cli --> Tracker
```
