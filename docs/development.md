# Development Guide

## Development Environment

### Nix Flake (Recommended)

This project includes a Nix flake for reproducible development environments.

**Installation:**
```sh
# Install Nix (with flakes enabled)
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install

# Or using the official installer (requires manual flake config)
sh <(curl -L https://nixos.org/nix/install) --daemon
```

**The flake provides:**

- **Node.js** - Latest stable version for running the monorepo
- **Corepack** - Package manager manager (enables pnpm)
- **Python3** - Required by node-gyp for native module compilation
- **Alejandra** - Nix code formatter

**Usage:**
```sh
# Enter development shell with all dependencies
nix develop

# Or use direnv for automatic shell activation
echo "use flake" > .envrc && direnv allow
```

**Benefits:**
- Identical development environment across all machines
- No need to manually install Node.js, pnpm, or Python
- Automatic dependency management
- Works on Linux, macOS, and Windows (WSL)

### Manual Setup

If you prefer not to use Nix, ensure you have:
- Node.js 18+
- pnpm (via `corepack enable`)
- Python 3 (for native modules)

## Running Code

This template leverages [tsx](https://tsx.is) to allow execution of TypeScript files via NodeJS as if they were written in plain JavaScript.

To execute a file with `tsx`:

```sh
pnpm tsx ./path/to/the/file.ts
```

## Operations

**Building**

To build all packages in the monorepo:

```sh
pnpm build
```

**Testing**

To test all packages in the monorepo:

```sh
pnpm test
```

**Type Checking**

To check TypeScript types across all packages:

```sh
pnpm check
```

**Linting**

To lint code across all packages:

```sh
pnpm lint
```

**Linting with auto-fix**

To automatically fix linting issues:

```sh
pnpm lint --fix
```

## Pre-push Hook

The repository includes a git pre-push hook that runs the following checks:
- TypeScript compilation (`pnpm check`)
- Linting (`pnpm lint`)
- Build (`pnpm build`)

This ensures that broken code doesn't get pushed to the repository. The hook will prevent pushes if any of these checks fail.

## Code Generation

Some packages may include code generation steps:

```sh
pnpm codegen
```

This runs any build-time code generation across the monorepo.

## Package Dependencies

When adding dependencies, use the appropriate scope:

```sh
# Add to root workspace
pnpm add -w <package>

# Add to specific package
pnpm add --filter @openagentsinc/pylon-domain <package>

# Add dev dependency
pnpm add -D <package>
```

## Workspace Commands

Run commands across all packages:

```sh
# Run a script in all packages that have it
pnpm --recursive run <script>

# Run in parallel for speed
pnpm --recursive --parallel run <script>

# Run in specific package
pnpm --filter @openagentsinc/pylon-server <command>
```