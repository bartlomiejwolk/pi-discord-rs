# Gitflow Workflow

This fork follows the [Gitflow branching model](https://nvie.com/posts/a-successful-git-branching-model/) with one adaptation: `master` is **our** production branch, not an upstream tracking branch. Upstream changes are tracked separately via `upstream-master`.

## Branches

| Branch | Purpose | Tracks | Merges To |
|--------|---------|--------|-----------|
| `master` | **Our** production releases | — | — |
| `develop` | Integration branch for features | — | — |
| `upstream-master` | Pristine upstream tracker (`darkautism/agent-discord-rs`) | `upstream/master` | Nothing (FF-only) |
| `feature/*` | New features / non-urgent bugfixes | `develop` | `develop` |
| `hotfix/*` | Urgent production fixes | `master` | `master` + `develop` |
| `release/*` | Release preparation | `develop` | `master` + `develop` |

## Why `master` is ours

We cannot rely on upstream accepting our PRs, and we **need** the hotfixes/features to run. So `master` is our deployable production branch. If upstream merges a PR, we fast-forward `upstream-master`, then merge `upstream-master` into `master`.

## Workflow Rules

### Starting work

```bash
# New feature
git checkout develop
git pull origin develop
git checkout -b feature/my-feature

# Production hotfix
git checkout master
git pull origin master
git checkout -b hotfix/critical-fix
```

### Syncing upstream

```bash
# Fetch upstream
git fetch upstream

# Fast-forward the pristine tracker (never commits directly)
git checkout upstream-master
git merge --ff-only upstream/master
git push origin upstream-master

# If upstream accepted something we want in our production:
git checkout master
git merge --no-ff upstream-master -m "Merge upstream/master into master"
git push origin master

# Then sync develop
git checkout develop
git merge --no-ff master -m "Merge master into develop"
git push origin develop
```

### Finishing work

```bash
# Feature: rebase onto develop, push, open PR against develop
git checkout feature/my-feature
git rebase develop
git push -f origin feature/my-feature
# → Open PR: feature/my-feature → develop

# Hotfix: rebase onto master, push
git checkout hotfix/critical-fix
git rebase master
git push -f origin hotfix/critical-fix
# → Open PR: hotfix/critical-fix → master (upstream, optional)
# → Also merge locally: hotfix/critical-fix → master AND develop
```

## Current Branch State

```
upstream-master ── ee7487f  ← pristine upstream (darkautism)

master ─────────── f91285f  ← our production (hotfix merged)
    ↑
    │
develop ────────── e0c1944  ← our integration (feature + master merged)
    ↑
    ├── feature/session-lifecycle  → PR #2 (optional upstream)
    │
    └── hotfix/auth-cli  → merged to master + develop; PR #1 (optional upstream)
```

| Branch | Type | Status |
|--------|------|--------|
| `upstream-master` | Upstream tracker | At `ee7487f` |
| `master` | Production | Hotfix merged |
| `develop` | Integration | Feature + hotfix merged |
| `feature/session-lifecycle` | Feature | Merged to `develop`; PR #2 open |
| `hotfix/auth-cli` | Hotfix | Merged to `master` + `develop`; PR #1 open |
