# Project Context

## Purpose
rust crate for bitrounding inspired by bitround.jl

## Tech Stack
- rust
- zarrs (https://crates.io/crates/zarrs) for Zarr format support

### Code Style
- Use `rustfmt` with default settings (line length 88)
- Follow standard Rust idioms and best practices
- Prefer explicit error types over unwrap()
- Use clippy lints

### Architecture Patterns
rust native for pure performance from metal
focus on linux and macos based on apple m series
take best rust practices from https://github.com/earth-mover/icechunk

### Testing Strategy
verify against test suite taken from bitround.jl

### Git Workflow
common sense, no git add ., use gh cli

#### Commit Convention for Archived Specs
When archiving a change with `openspec archive <id> --yes`:
1. Run `openspec archive` first (creates archive dir and updates specs/)
2. Stage changes: `gh auth login` then `git add openspec/specs/ openspec/changes/archive/`
3. Commit with message based on change type:
   - `feat(<capability>): <description>` - New features
   - `fix(<capability>): <description>` - Bug fixes
   - `perf(<capability>): <description>` - Performance improvements
   - `refactor(<capability>): <description>` - Code refactoring
   - `docs(<capability>): <description>` - Documentation changes
4. Create PR for review before merging

## Domain Context
Rust crate for bitround compression algorithm to reduce climate data size on disk.
Target workflow:
1. Open climate data (Zarr format via zarrs)
2. Apply bitround compression
3. Save back with compression
Compare against:
- Python implementation: https://github.com/zarr-developers/numcodecs/blob/main/numcodecs/bitround.py
- Information-based bit allocation: https://github.com/observingClouds/xbitinfo

## Important Constraints
verify against test suite taken from bitround.jl

## External Dependencies
read netcdf and zarr in rust

## Installation
### macOS
```bash
brew install rust
```
