---
title: CLI reference
description: The complete oura command reference, generated from the binary's own --help output.
sidebar:
  order: 1
---

{/* Frontmatter + intro for the generated CLI reference. `just docs-gen-cli` copies this file
    to reference.md and appends one section per command captured from the `oura` binary's own
    `--help`. This underscore-prefixed fragment is ignored by Astro (never its own page). */}

The complete `oura` command reference below is generated from the binary's own `--help`, so it
always matches the installed CLI — a regenerate-and-diff check ([`just docs-gen-cli-check`]) fails
CI if it drifts. For a narrative tour of the commands, see [CLI usage](/guides/cli-usage/).

[`just docs-gen-cli-check`]: https://github.com/spxrogers/oura-toolkit/blob/main/justfile
