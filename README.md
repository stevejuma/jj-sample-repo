# JJ Sample Repo

A small task-tracking CLI used as a GitButler and JJ sample repository.

The repository history is intentionally structured to include:

- parallel branches
- a small stacked branch chain
- a merged pull request on trunk
- open pull requests
- branch content that can conflict during a workspace merge

## Commands

List tasks:

```bash
cargo run -- list
```

List owners:

```bash
cargo run -- owners --limit 2
```

Override the environment banner:

```bash
cargo run -- --env staging
```

## Sample Scenarios

- `feature/bootstrap-cleanup` is intended to represent a merged maintenance PR.
- `feature/cloud-runner` and `feature/query-filtering` are intended to conflict in shared output code.
- `feature/cloud-observability` is stacked on top of `feature/cloud-runner`.
- `feature/export-json` is a parallel branch that adds data export behavior.

## Local Testing Notes

- If you apply both `feature/cloud-runner` and `feature/query-filtering`, they should compete for the header rendering in `src/output.rs`.
- `feature/cloud-observability` is intentionally stacked on `feature/cloud-runner` so you can test dependent PR and branch behavior.
- Keeping your local `trunk` stale after merging a remote PR is useful for fetch and integrate-upstream testing.

## Resetting To A Known Test State

From a fresh clone, run:

```bash
make reset
```

That will:

- fetch and prune remote refs
- force-refresh tags and reset local `trunk` to the shared `reset-base` anchor
- recreate the local tracking branches used by the sample PRs
- recreate a local-only branch named `local-only/scratch-pad` with no pull request
- refresh JJ state if `jj` is installed

This intentionally leaves local `trunk` behind `origin/trunk`, which gives you a stable fetch/integration scenario for testing.

To inspect the current state after reset:

```bash
make show-state
```

Remote `trunk` intentionally stays one commit ahead of `reset-base` so `make reset` recreates a stale local trunk for fetch testing.
