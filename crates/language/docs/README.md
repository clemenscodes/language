# The language gate

The gate turns away an identifier made of a word the context it is in does not have.

It takes Rust sources, takes every identifier apart into the words it is made of,
and reports the words that none of the languages of that source has. Three kinds of
language are merged for one source, the one of its own context, Rust's own
vocabulary, and one for each third party crate its `Cargo.toml` names.

```
vocabulary crates/harbour/src/lib.rs
[LANGUAGE] crates/harbour/src/lib.rs writes Zonk.
```

A word it reports is either the wrong word for what was named, or a word that
belongs in that context's language and is not there yet. It is almost always the
first, because a value named `made` or `taken` is a value named after nothing, and a
past tense is an error outright.

The status is one when anything was an error and zero when nothing was.

This crate is a library. The contexts are not part of it, because every repository
has its own. A repository that uses the library has a crate of its own, conventionally
`crates/vocabulary`, whose `main.rs` builds the contexts of that repository and hands
them over.
