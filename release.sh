#!/bin/bash
set -e

# Run tests

# Use changelogen to prepare release files without committing
npx changelogen --release --no-commit --no-tag

# Sync versions
PACKAGE_VERSION=$(grep -oP '"version": "\K[^"]+' package.json)
sed -i "s/^version = \".*\"/version = \"$PACKAGE_VERSION\"/" Cargo.toml

cargo test --verbose
cargo check --quiet
cargo fmt --all -- --check
cargo clippy -- -D warnings

# Stage all files
git add package.json CHANGELOG.md Cargo.toml Cargo.lock

# Create commit and tag
git commit -m "chore(release): v$PACKAGE_VERSION" package.json CHANGELOG.md Cargo.toml Cargo.lock
git tag "v$PACKAGE_VERSION"

# Push everything (this will trigger GitHub release creation in browser)
git push --follow-tags

# Publish to registries
cargo publish
npm publish