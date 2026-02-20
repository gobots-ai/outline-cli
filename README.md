# outline-cli

A Rust CLI for the [Outline](https://www.getoutline.com/) Wiki API. Designed for both human users and AI agents — all output is JSON.

## Install

```sh
./install.sh
```

Builds a release binary and copies it to `/usr/local/bin/outline`.

### From source

```sh
cargo build --release
cp target/release/outline /usr/local/bin/
```

## Authentication

```sh
# Interactive
outline auth

# Non-interactive (CI/agents)
outline auth --token <TOKEN> --url https://your-instance.getoutline.com

# Check status
outline auth status
```

Credentials are stored in `~/.config/outline-cli/config.toml`.

## Usage

### Documents

```sh
outline documents list [--collection-id <ID>] [--offset <N>] [--limit <N>]
outline documents get <ID>
outline documents create --title <TITLE> --collection-id <ID> [--text <MARKDOWN>]
outline documents update <ID> [--title <TITLE>] [--text <MARKDOWN>]
outline documents delete <ID> [--permanent]
outline documents search <QUERY> [--collection-id <ID>] [--limit <N>]
outline documents export <ID>
```

### Collections

```sh
outline collections list [--offset <N>] [--limit <N>]
outline collections get <ID>
outline collections documents <ID>
```

## Output

All commands output JSON to stdout. Errors output JSON to stderr with a non-zero exit code:

```json
{"error": "not_authenticated", "message": "not authenticated — run `outline auth` to configure credentials"}
```

| Exit Code | Meaning |
|-----------|---------|
| 0 | Success |
| 2 | Not authenticated |
| 3 | API error |
| 4 | HTTP error |
| 5 | Config error |
| 6 | IO error |

## License

MIT
