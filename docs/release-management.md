# Release Management & CI/CD

This project uses **Changesets** for automated package versioning and publishing, integrated with GitHub Actions for continuous integration and deployment.

## Changesets Workflow

**Changesets** is a tool that manages versioning and publishing in monorepos by requiring explicit change documentation:

1. **Making Changes**: When you make changes that should trigger a new release:
   ```sh
   pnpm changeset
   ```
   This interactive CLI will:
   - Ask which packages have changed
   - What type of change (major, minor, patch)
   - Create a changeset file describing the changes

2. **Changeset Files**: Created in `.changeset/` directory, these markdown files:
   - Document what changed
   - Specify which packages need version bumps
   - Get committed with your code changes

3. **Version Bump**: On main branch, run:
   ```sh
   pnpm changeset-version
   ```
   This will:
   - Consume changeset files
   - Update package.json versions
   - Generate CHANGELOG.md files
   - Create a version commit

4. **Publishing**: After version bumps:
   ```sh
   pnpm changeset-publish
   ```
   This will:
   - Build all packages
   - Run tests to ensure quality
   - Publish changed packages to npm

## CI/CD Pipeline

The GitHub Actions workflows automate this process:

### **Check Workflow** (`.github/workflows/check.yml`)
Runs on every push and PR to `main`:

- **Build**: Runs code generation and verifies source state
- **Types**: Validates TypeScript compilation (`pnpm check`)  
- **Lint**: Checks code style and formatting (`pnpm lint`)
- **Test**: Runs the test suite (`pnpm vitest`)

### **Release Workflow** (`.github/workflows/release.yml`)
Runs on every push to `main` branch:

- **Automated Publishing**: Uses `changesets/action` to:
  - Check for pending changesets
  - Create "Version Packages" PR if changesets exist
  - Auto-publish packages when version PR is merged
  - Requires `NPM_TOKEN` secret for npm authentication

### **Snapshot Workflow** (`.github/workflows/snapshot.yml`)
Runs on PRs to create preview packages:

- **PR Previews**: Creates temporary package versions for testing
- **Uses pkg-pr-new**: Publishes snapshot versions for review

## Package Configuration

Current packages use the `@openagentsinc` scope:

- `@openagentsinc/pylon-domain` - Shared schemas and API contracts
- `@openagentsinc/pylon-server` - Backend server implementation  
- `@openagentsinc/pylon-cli` - Command-line interface

**Publishing Settings**:
- **License**: AGPL-3.0-or-later
- **Access**: Public (anyone can install)
- **Registry**: npm (registry.npmjs.org)

## Development Workflow

1. **Feature Development**:
   ```sh
   # Make your changes
   git checkout -b feature/my-feature
   # ... code changes ...
   
   # Add changeset for your changes
   pnpm changeset
   
   # Commit everything including changeset
   git add . && git commit -m "feat: add new feature"
   git push origin feature/my-feature
   ```

2. **Create PR**: The Check workflow runs automatically

3. **Review & Merge**: When PR is merged to main:
   - Release workflow checks for changesets
   - Creates "Version Packages" PR if needed

4. **Release**: Merge the "Version Packages" PR to publish

## Common Commands

```sh
# Check what packages have changed
pnpm changeset status

# Preview what versions would be published  
pnpm changeset-version --dry-run

# Manually trigger a release (local)
pnpm changeset-publish
```

## Troubleshooting

**CI Publishing Errors**: The CI requires proper authentication and package configuration:
- ✅ Updated package names to `@openagentsinc/pylon-*`
- ✅ Set access to "public" in changeset config
- ✅ Updated TypeScript paths and workspace references

**Authentication**: CI requires `NPM_TOKEN` secret in GitHub repository settings for automated publishing.

**Pre-push Hook**: If the pre-push hook fails, fix the issues locally before pushing:
- Run `pnpm check` to fix TypeScript errors
- Run `pnpm lint --fix` to fix linting issues
- Run `pnpm build` to ensure build succeeds

## Setup Requirements

To enable automated publishing:

1. **NPM Token**: 
   - Create npm access token at https://www.npmjs.com/settings/tokens
   - Add as `NPM_TOKEN` secret in GitHub repository settings

2. **Repository Settings**:
   - Ensure repository is public or has proper access permissions
   - Enable GitHub Actions in repository settings

3. **Branch Protection**:
   - Consider protecting `main` branch
   - Require PR reviews before merging
   - Require status checks to pass