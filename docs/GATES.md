# The gates

A gate is a check that refuses. It is not advice and there is no flag that turns one
off. Every gate is a check of the flake, and `moon run :ci` builds each of them as a
task of its own, so a gate is built again only when a file it reads has changed.

A gate that has never been seen to fail is not known to be a gate. So a new gate is
made to fail on purpose once and then restored, and what it printed is recorded on
this page.

## What each one refuses

| Gate | What it refuses | What it reads |
|---|---|---|
| `statix` | a Nix file with a known smell | every `.nix` file |
| `deadnix` | a Nix binding nothing reads | every `.nix` file |
| `format` | Nix that is not formatted | every `.nix` file |
| `rustfmt` | Rust that is not formatted | `Cargo.toml` and `crates` |
| `comment` | a comment in a crate | every `.rs` file under `crates` |
| `test` | a function beside the tests of a test module, and a decision inside a `main.rs` | every `.rs` file under `crates` |
| `word` | a refused verb, a banned word, hexadecimal in lower case that this repository formed, and a name of Rust that names nothing, names an act or names a position | `crates` except the vocabularies under `crates/language/src/core/languages`, `docs` except `RUST_STYLE.md`, `nix` and `README.md` |
| `punctuation` | an em dash, an en dash, an arrow, a set membership, an ellipsis or a box drawing line anywhere, and a semicolon or a separator line in the prose of a Markdown page | `.moon`, `crates`, `docs`, `nix` and every file at the root that people edit |
| `book` | a book that does not build, a page it names that is not there, and a page it leaves out of its summary | both books |
| `language` | the library when it does not build or a test of it fails | the manifests, the lock file, the nextest profiles and the source of the library |
| `outputs` | a flake output nobody recorded, and a row naming one that is gone | `docs/OUTPUTS.md` and the flake |
| `pipeline` | a check of the flake that no moon task builds, a task naming a check the flake does not have, and a gate whose task does not list a directory the gate reads | `moon.yml`, `.moon/tasks` and `nix/flake/checks/default.nix` |

The vocabularies are exempt from `word`, because a definition has to be able to contain
a word in order to define it.

## Seen to refuse

Every gate was made to fail on 2026-09-29, in a copy of the tree with one fault per
gate, and every one of them refused. This is what each printed.

`statix` refused a `let` with no bindings.

```
[02] Warning: Useless let-in expression
```

`deadnix` refused a `let` binding nothing reads.

```
Unused let binding: unused
```

`format` refused `{a=1;}`.

```
Error: unexpected changes detected, --fail-on-change is enabled
```

`rustfmt` refused a `pub use` with two spaces too many.

```
-pub use   core::Languages;
+pub use core::Languages;
```

`comment` refused a line comment in `lib.rs`.

```
[COMMENT] a crate contains no comment.
./crates/language/src/lib.rs:5:// the edge of the crate
```

`test` refused a function beside the tests of a test module, and in a second copy a
decision inside a `main.rs`.

```
[TEST] ./crates/language/src/application/command_line/outcome/mod.rs keeps 1 functions beside its tests.
[TEST] a decision inside a main. No test can call a main, so nothing decides there.
./crates/probe/src/main.rs:3:    if flag > 1 {
```

`word` refused a sentence in `docs/README.md` with a banned word. The line is not
quoted here, because this page would then be refused as well.

`punctuation` refused a semicolon in the same sentence.

```
[PUNCTUATION] prose has no semicolon and no separator line.
```

`book` refused a page the summary does not name.

```
> ./docs/probe.md
[BOOK] . leaves a page out of its summary, or names one twice.
```

`language` refused a test whose expectation was changed from 21 to 22.

```
FAIL core::languages::foreign::tests::every_language_this_library_adopts_from_outside_has_a_name_of_its_own
  left: 21
 right: 22
```

`outputs` refused a row for an output the flake does not have.

```
> ghost package
[OUTPUTS] the flake and docs/OUTPUTS.md do not agree.
```

`pipeline` refused a task that no longer lists `README.md`, which its gate reads.

```
word reads README.md and its task lists no README.md
```

## Running one by hand

```
nix flake check
nix build --no-link --print-build-logs .#checks.x86_64-linux.word
moon run :ci
```
