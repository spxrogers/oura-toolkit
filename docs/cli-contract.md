# CLI contract

The scripting contract for the `oura` binary (#21). Changes to any of this are breaking
changes for users' scripts and require a deliberate decision.

## Exit codes

| code | meaning | example |
|------|---------|---------|
| `0` | success | data returned, auth flow completed |
| `1` | runtime error | Oura API 5xx, network failure, I/O error |
| `2` | usage error | unknown flag/command, missing argument, bare `oura` |
| `4` | authentication required | no stored tokens/credentials, or a refresh rejected by the token endpoint — an auth flow will fix it |

Scripting example:

```sh
oura sleep list --json > sleep.json
case $? in
  0) ;;                                  # data in sleep.json
  4) oura auth login && retry ;;         # auth is fixable non-interactively? then retry
  *) echo "oura failed" >&2; exit 1 ;;
esac
```

## Streams

- **stdout** — results only (tables, plain lines, JSON). `oura mcp` is stricter: stdout is
  the JSON-RPC transport and carries nothing else.
- **stderr** — everything meant for humans: prose, progress, prompts, errors, hints.

## Output formats (data commands)

- **TTY default:** human-readable aligned table.
- **Piped default:** stable tab-separated lines, no header — safe for `cut`/`awk`.
- **`--json`:** pretty JSON from the command's data model. Never a default.
- **Color:** only on a TTY; disabled by `--no-color` or a non-empty `NO_COLOR` env var
  ([no-color.org](https://no-color.org)).

## Error style

One line to stderr: `oura: <what failed>: <why>` — followed by a `hint:` line when the fix
is known:

```
oura: fetching sleep data: not authenticated (no tokens stored)
hint: run `oura auth login`
```

No backtraces for expected errors.
