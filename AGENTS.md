# tpe-frontend-webapp

- Agents must work on a branch and open a PR (`gh pr create --fill`); never push directly to `main` or use the owner's bypass, even when authenticated as the owner.
- The `ci` check must pass for agent-authored work. Only the human owner's own work may use the direct-push exception.
- Never commit secrets, `.env` files, databases or `.DS_Store` (see `.gitignore`; CI rejects them).
- Before committing, run:

```sh
cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test
npm run lint && npm test
npm run lint:css
npx @stoplight/spectral-cli lint openapi.yaml
npm run typecheck && npm run test:ts
```
