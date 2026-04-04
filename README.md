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