# Project Brief

## What it is

- Baleno: a self-hosted web interface to administer the Docker containers, Compose stacks, images and volumes of several servers.
- Personal open-source project (AGPL-3.0-only), single-user in v1, data model ready for multiple accounts and roles.

## Why it exists

- Drive VPS and bare-metal hosts from one central web app, grouping containers by Compose project the way OrbStack does.

## Domain language

| Term | Meaning |
| ---- | ------- |
| central | the web app and API that the user opens; holds the database |
| agent | one process per managed server; dials out to the central and drives the local Docker |
| server | a managed host, reached only through its agent |
| stack | a Compose project on one server |

## Key features

- Containers grouped by Compose stack, across several servers.
- Real-time logs, container and host terminals, file browser.
- Monitoring per container, per stack and per host.
