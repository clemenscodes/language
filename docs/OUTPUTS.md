# The outputs

Every output of the flake is in this table, and the `outputs` gate refuses a flake and a
table that do not state the same thing.

| Output | Kind | What it is |
|---|---|---|
| `book` | package | the book of the repository, rendered |
| `language-book` | package | the book of the library, rendered |
| `statix` | check | the gate that refuses a Nix file with a known smell |
| `deadnix` | check | the gate that refuses a Nix binding nothing reads |
| `format` | check | the gate that refuses Nix that is not formatted |
| `rustfmt` | check | the gate that refuses Rust that is not formatted |
| `comment` | check | the gate that refuses a comment in a crate |
| `test` | check | the gate that refuses a function beside the tests and a decision inside a main |
| `word` | check | the gate that refuses a word this repository refuses |
| `punctuation` | check | the gate that refuses a sign prose in this repository does without |
| `book` | check | the gate that refuses a book that does not build or leaves a page out |
| `language` | check | the gate that refuses the library when it does not build or a test of it fails |
| `outputs` | check | the gate that refuses this table when it and the flake disagree |
| `pipeline` | check | the gate that refuses a check no moon task builds and a task input that misses a source |
| `default` | shell | the devshell, with every tool the repository uses |
