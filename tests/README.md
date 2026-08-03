# Tests layout

**`tests/original/` -> see `/test/` at the repo root.**

The original, unmodified upstream test suite (unit tests, MessagePack
fixtures, fuzz harness C sources, AVR build) lives at `test/` at the
repo root rather than `tests/original/`. It was **not moved or renamed**,
on purpose: `test/` is threaded through the original project's own
build tooling (`tools/unit.sh`, `test/unit/configure.py`,
`test/fuzz/Makefile`, `test/avr/Makefile`, CI config, etc.), and moving
it would have meant hand-editing that tooling -- which risks silently
changing behavior in code we're claiming is untouched. Leaving it at
its native path was the safer way to guarantee it's genuinely
byte-for-byte what shipped upstream.

The original C *source* (`src/mpack/`) was moved, to `reference-c/`,
because the port's Rust code needed the `src/` name at the repo root.
See `DECISIONS.md` for the full reasoning and the list of build scripts
that were updated to point at the new path.

**`tests/port/`** -- new tests written specifically for the Rust port
that aren't part of the original suite go here.
