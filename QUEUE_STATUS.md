# Queue status — 2026-10-19

This file records what was shipped, what is blocked/failed, and what is rejected for this repository. It complements `AUDIT_FINDINGS.md`.

## Shipped
- No code changes; audit docs.

## Blocked / failed
- Remaining 12 GTK4 widgets; no downstream app.

## Rejected / deferred
- Widget expansion until a GTK app adopts it.

## Policy

Any future feature or upgrade must check `AUDIT_FINDINGS.md` and this file before implementation. A blocked item is retried only when its external dependency is available; a rejected item needs new user evidence or a design change to reopen.
