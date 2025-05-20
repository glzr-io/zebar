# Publish to the marketplace

Widget packs can be published to the Zebar marketplace, making them available for other users to download and use.

## Prerequisites

Before publishing, make sure you have:

1. Zebar installed.
2. Obtained an API token from [glzr.io/api-token](https://glzr.io/api-token).

## Publishing via CLI

With Zebar installed, you can publish your widget pack using the `publish` command.

**Basic usage:**

```bash
zebar publish --token your-api-token
```

**With additional metadata:**

```bash
zebar publish --token your-api-token \
  --pack-config ./my-pack/zpack.json \
  --version-override 1.2.0 \
  --commit-sha abc123def456 \
  --release-notes "Fixed performance issues and added new themes." \
  --release-url "https://github.com/username/pack-repo/releases/v1.2.0"
```

### Required arguments

- `--token <TOKEN>`: API token for authentication. The widget pack gets published under the account that this token belongs to.
  - Can also be set via the `ZEBAR_PUBLISH_TOKEN` environment variable.

### Optional arguments

- `--pack-config <PATH>`: Path to the pack config file (default: `./zpack.json`).
- `--version-override <VERSION>`: Override the version number in the pack config (must be a valid semver string, e.g., `1.0.0`).
- `--commit-sha <SHA>`: Commit SHA associated with this release (will be shown on the marketplace page).
- `--release-notes <NOTES>`: Release notes for this version (will be shown on the marketplace page).
- `--release-url <URL>`: URL to the release page (will be shown on the marketplace page).

## Support

If you encounter issues with publishing, you can:

- Join the [glzr.io Discord](https://discord.gg/ud6z3qjRvM) community.
- Open an issue on the [Zebar GitHub repository](https://github.com/glzr-io/zebar).
