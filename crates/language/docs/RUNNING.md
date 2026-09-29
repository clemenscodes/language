# Using it

A repository depends on the library through Cargo, pinned to a tag.

```toml
[workspace.dependencies]
language = { git = "https://github.com/clemenscodes/language", tag = "<tag>" }
```

The crate of that repository builds its contexts and hands them to the command line.
Its `main.rs` parses, delegates and returns the status, and decides nothing. Where the
contexts and the foreign languages come from is that repository's own design.

```rust
fn main() -> io::Result<ExitCode> {
    let line = CommandLine::parse();
    let languages = Languages::try_from((contexts, foreign));
    line.try_respond(languages)
}
```

`try_respond` takes the languages or the error that kept them from coming about. An
error is printed to the error stream and ends the process with a failure. An error of
the streams themselves is returned, so the process ends with it rather than losing it.

The library publishes six names.

| Name | What it is |
|---|---|
| `Language` | a vocabulary, built with `try_define` for a term and its definition and `try_admit` for a word without one, and joined with another by `\|` |
| `Context` | one directory and its language, from `(directory, language)` under `crates`, from `(root, directory, language)` under any root, and from `(context, names)` with the names of foreign languages it may also use |
| `Foreign` | the words of one third party crate, from `(name, language)` |
| `Languages` | every language of a repository, from `(contexts, foreign)`, joined with Rust's own words |
| `CommandLine` | the arguments, parsed by clap, and `try_respond` |
| `Error` | why a language could not come about |

## The gate

With sources and nothing else it is the gate. It takes the sources to judge as
arguments and finds nothing by itself, so which files are checked is the caller's
decision.

Which language a source is checked against follows from its path. A context is a
directory under its root, and the gate merges that context's language with Rust's own
vocabulary, with every foreign language the context names, and with one language for
every third party crate the source's `Cargo.toml` names.

## The press

With `--book` it prints the ubiquitous language book of a context out of the
vocabulary. With no directory it prints every book, with directories it prints those.

```
vocabulary --book
vocabulary --book crates/harbour
vocabulary --book crates/harbour crates/ship
```

A book is the vocabulary as a page and nothing else, so nobody edits those pages by
hand. A directory no context covers is an error and no book comes out of it.

## Here

```
moon run language:cargo/nextest
moon run language:nix/build/language
```
