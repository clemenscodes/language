# Where the words live

There is one language per bounded context, and a context is a directory under a root,
`crates` unless the context names another. The contexts belong to the repository that
uses this library and live in that repository's own crate. `core/languages` is where
the library joins them with its own.

`core/languages/foreign` is one module per third party crate, each a flat list of the
words that crate brought along. A source is judged against the language of a foreign
crate when its own `Cargo.toml` names that crate, which the gate reads rather than
keeping a second list beside it. A crate that loses a dependency loses its words in
the same act. A repository hands over foreign languages of its own beside these, and a
context may name one it uses without a manifest, which is how a context of another
programming language gets the words of that language.

`core/languages/rust` is Rust's own words, which have no definitions here because
`fn` means nothing in a domain and is only allowed. It is the anti corruption layer,
and it exists so that the platform's vocabulary never has to enter the domain's.

Rust is shared by every context. A context's own language and its foreign languages
are not.

The words live in Rust and are not taken out of the book. A book is what a person
reads and it is never an input to a tool. Where the two are made to agree it is by
generating the page from the words, and never the other way round.

Adding a word is adding a term, with its definition, in the module of the context
that means it. There is no list of forbidden words, because nothing is forbidden.
Every word means what it means, and a word whose meaning does not occur in that
context is simply the wrong word for what was named.

## What is one word

A word is checked against a language in its own case and in its own plural, so
`Entry` covers `entries`, `Name` covers `NAMES` and `Resolve` covers `resolves`.
`Word::stem` is where that is decided, by the three endings English uses, and it
folds nothing else together. A word whose stem no language has is an error like any
other.

## What is not judged

A number is not a word, so a source that contains `20` is not asked to define it.

A `#[cfg(test)] mod tests` is not judged at all. `docs/RUST_STYLE.md` requires a test
to be named as a sentence, and a sentence is made of ordinary English, so the two
rules would otherwise contradict each other. What ships is judged and what proves it
is not.
