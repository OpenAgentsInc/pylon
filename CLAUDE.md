# Claude Project Instructions

## Package Management

When installing packages, always use pnpm with the `--save-exact` flag to ensure exact versions are pinned:

```bash
pnpm add --save-exact <package>
pnpm add -D --save-exact <dev-package>
```

This prevents version drift and ensures consistent builds across environments.