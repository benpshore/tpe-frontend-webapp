# tpe-frontend-webapp

tpe-frontend-webapp

## Checks

Run these before opening a PR. CI runs the same ones.

```sh
cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test
npm run lint && npm test
npm run lint:css
npx @stoplight/spectral-cli lint openapi.yaml
npm run typecheck && npm run test:ts
```

The owner (`benpshore`) may push their own work directly to `main` without a PR or passing CI. CI still runs after a push; releases retain their CI gates. Force-push, deletion, and merge commits remain blocked for everyone.

Other accounts must use a squash PR with passing, up-to-date `ci`. Agents must use PRs and CI even when authenticated as the owner (see `AGENTS.md`). GitHub checks the pushing account, so enforcing a human/agent boundary requires separate credentials.
