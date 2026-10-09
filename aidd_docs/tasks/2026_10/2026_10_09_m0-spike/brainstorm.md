---
title: "M0 · Spike : lever les risques techniques de Baleno"
status: stable
updated: 2026-10-09
owner: bryan
---

# M0 · Spike : lever les risques techniques de Baleno

## 🎯 Objectif

Chaque risque technique connu de Baleno reçoit une réponse prouvée, ou une alternative décidée, avant la première feature.
Le périmètre dépasse volontairement v0.1.0 : les terminaux et le credential agent sont inclus pour que les jalons suivants partent avec leurs réponses.
M0 est terminé quand ses huit issues sont fermées.

## 📌 Points à lever

| Issue | Point | Nature | VM de test |
| --- | --- | --- | --- |
| [#12](https://github.com/bryanbergerprojects/Baleno/issues/12) | `exec` bollard avec redimensionnement TTY | expérience | oui |
| [#5](https://github.com/bryanbergerprojects/Baleno/issues/5) | `portable-pty` : fermeture de session et processus orphelins | expérience | oui |
| [#6](https://github.com/bryanbergerprojects/Baleno/issues/6) | sortie OpenAPI d'utoipa 6 consommée par Hey API 0.99 | expérience | non |
| [#7](https://github.com/bryanbergerprojects/Baleno/issues/7) | fallback `/_shell.html` servi par Axum sous une CSP stricte | expérience | non |
| [#8](https://github.com/bryanbergerprojects/Baleno/issues/8) | `BEGIN IMMEDIATE` / `begin_with` via SQLx 0.9 | expérience | non |
| [#9](https://github.com/bryanbergerprojects/Baleno/issues/9) | charge SQLite : 100 conteneurs échantillonnés toutes les 10 s, lectures UI concurrentes | mesure | oui |
| [#10](https://github.com/bryanbergerprojects/Baleno/issues/10) | CVE-2026-102989 de TanStack Start : impact en mode SPA, version corrigée | lecture de l'avis | non |
| [#11](https://github.com/bryanbergerprojects/Baleno/issues/11) | credential agent : mTLS ou clé signée | décision argumentée | non |

## ✅ Règles du spike

- Le code écrit pour le spike est jetable et n'entre jamais dans le workspace Cargo ni pnpm.
- Une issue se ferme avec son verdict et sa preuve : commande, sortie, mesure.
- `aidd_docs/INSTALL.md` reprend ensuite les verdicts : tableau d'audit, versions, alternatives retenues.
- Les expériences marquées « VM de test » tournent sur une VM Debian jetable, avec Docker Engine et le plugin Compose, créée sur Hyperion (l'hyperviseur Proxmox d'Olympus).
- Ces expériences ne tournent jamais sur macOS : un PTY s'y comporte autrement, et l'agent ne vise que Linux.
- La création de la VM est tracée le jour même dans `Olympus/architecture-oikos.md`, et chaque geste en écriture sur l'hyperviseur annonce son retour arrière.

## 📊 Critères du test de charge

La base part du régime permanent : 24 h d'échantillons bruts et 30 jours d'agrégats à la minute.
Le sous-échantillonnage, la purge et la sauvegarde `VACUUM INTO` tournent pendant la mesure.

| Critère | Seuil d'échec |
| --- | --- |
| Erreurs d'écriture | au moins un `SQLITE_BUSY` ou un échantillon perdu |
| Cadence | un lot de 10 s non validé avant le tick suivant |
| Lectures UI | p95 au-dessus de 50 ms |

Une seconde passe à 1 000 conteneurs mesure la marge disponible ; elle informe sans faire échouer le spike.

## ⏳ Questions ouvertes

| Sujet | Hypothèse retenue | Ce qui la confirmerait |
| --- | --- | --- |
| Seuil de 50 ms | proposition, non dérivée d'une mesure | le ressenti de l'UI pendant le test de charge |
| Gabarit de la VM | 1 vCPU, 1 Go de RAM, aligné sur le plus petit serveur visé | les caractéristiques du plus petit VPS, non vérifiées |
| Sort de la VM après M0 | conservée comme serveur de test de M4 et M5, avec la stack Louise | décision au moment de fermer M0 |
| Credential agent | M0 tranche entre mTLS et clé signée | si le choix exige le design complet du contexte `fleet`, le détail part en M2 |

## 🚀 Prochaine étape

Créer la VM de test sur Hyperion ; les spikes sans VM peuvent démarrer en parallèle.
