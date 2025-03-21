#!/bin/bash
set -e

PACKAGE_VERSION=$(grep -oP '"version": "\K[^"]+' package.json)

# Use changelogen to prepare release files without committing
npx changelogen --release --no-commit --no-tag

# Sync versions
sed -i "s/^version = \".*\"/version = \"$PACKAGE_VERSION\"/" Cargo.toml

# Run tests
cargo test --verbose
cargo check --quiet
cargo fmt --all -- --check
cargo clippy -- -D warnings

# Stage all files 
git add package.json CHANGELOG.md Cargo.toml Cargo.lock

# Tag and commit
git tag "v$PACKAGE_VERSION"
git commit -m "chore(release): v$PACKAGE_VERSION" package.json CHANGELOG.md Cargo.toml Cargo.lock

# Push everything (this will trigger GitHub release creation in browser)
git push --follow-tags

# Publish to registries
npm publish
cargo publish