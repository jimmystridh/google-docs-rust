# Google Docs Skill (Rust Port)

Rust port of the original Google Docs/Drive/Sheets skill with the same command surface and JSON output patterns.

## What is included

- `docs_manager` CLI: Google Docs operations (read, structure, edit, markdown, tables, images)
- `drive_manager` CLI: Google Drive file/folder operations
- `sheets_manager` CLI: Google Sheets data + formatting/chart/protection operations
- Shared OAuth token flow using:
  - `~/.claude/.google/client_secret.json`
  - `~/.claude/.google/token.json`

## Repository layout

- `src/bin/docs_manager.rs`
- `src/bin/drive_manager.rs`
- `src/bin/sheets_manager.rs`
- `src/auth.rs` shared OAuth logic
- `src/google_api.rs` shared Google HTTP client
- `scripts/*` runnable wrappers
- `references/*` operation guides and troubleshooting
- `examples/sample_operations.md` end-to-end examples

## Build

```bash
cargo build --release
```

## Usage

Use wrapper scripts (recommended):

```bash
scripts/docs_manager --help
scripts/drive_manager --help
scripts/sheets_manager --help
```

Compatibility wrappers are also included for existing references:

```bash
scripts/docs_manager.rb --help
scripts/drive_manager.rb --help
scripts/sheets_manager.rb --help
```

## Auth setup

1. Create Google Cloud OAuth Desktop credentials.
2. Save client secret JSON to:
   - `~/.claude/.google/client_secret.json`
3. Complete auth once:

```bash
scripts/docs_manager auth <code>
# or
scripts/sheets_manager auth <code>
```

Drive commands use the same shared token and do not have a separate `auth` command.

## Validation

This port was validated with:

```bash
cargo check --offline
cargo clippy --offline --all-targets --all-features
```

## License

MIT (same as original)
