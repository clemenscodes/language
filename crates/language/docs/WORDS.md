# Where the words live

There is one language per bounded context, and a context is a directory under a root,
`crates` unless the context names another. The contexts belong to the repository that
uses this library and live in that repository's own crate. `core/languages` is where
the library joins them with its own.

The library knows no third party crate. A repository hands over one foreign language
per third party crate it uses, each a flat list of the words that crate brought along,
and `core/languages/foreign` is only the type they are made of. A source is judged
against the language of a foreign crate when its own `Cargo.toml` names that crate,
which the gate reads rather than keeping a second list beside it. A crate that loses a
dependency loses its words in the same act. A context may also name a foreign language
it uses without a manifest, which is how a context of another programming language
gets the words of that language.

`core/languages/rust` is the language of Rust itself, which has no definitions here
because `fn` means nothing in a domain and is only allowed. It is the anti corruption
layer, and it exists so that the platform's vocabulary never has to enter the
domain's.

It has two lists. `WORDS` are the words of Rust on their own, the keywords and the
primitive types, and each of them is allowed wherever it occurs, inside any
identifier. `NAMES` are identifiers of the standard library and of the compiler, such
as `is_empty`, `as_bytes`, `Vec` and `should_panic`, and each of them is allowed only
as the identifier it is. So `text.is_empty()` passes on the words of Rust alone,
while `fn is`, `fn is_port` and `fn has_files` pass only in a context whose language
has `is`, `port`, `has` and `files`.

A word that only the standard library uses is not a word every domain may use, and
that is why the two are kept apart. A list of single words let `is` through because
`is_empty` exists, and with it every `fn is` any crate defined.

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

## What is one name

An identifier is one name, and a name is the words it is made of in their order,
taken apart the same way a source is. Two writings of the same words are the same
name, so `as_ref` and `AsRef` are one name, while `is_empty` and `empty_is` are two.
A name has no plural, so the name `file` does not let `files` through.

A source is judged name by name. A name its language admits is let through, and its
words are not judged at all. Every other name is taken apart and each of its words
has to be in the language. A language admits a name with `try_admit_names`, which a
foreign language can use as well as Rust does.

## Where the names of Rust come from

`NAMES` was measured from the source of the standard library that the toolchain of
this repository ships, 1.98.1. It is every identifier that `core`, `alloc` and `std`
declare as public and stable, every macro the compiler builds in, every public module
and every public re-export, whose words the earlier list of single words of Rust
already admitted. So no identifier passes now that did not pass before.

Eight names come from the compiler rather than from that source. `std` and `core` are
the crates every source can name, `main` is the function a program starts with, `repr`,
`should_panic` and `ignore` are attributes, `feature` is the key of a `cfg`, and `b`
is the prefix of a byte literal.

`is` was left out, although the standard library declares it for `Any` and for
`Error`, because a lone `is` is the word this split exists to keep out of a domain.

`WORDS` keeps six more words that are neither keywords nor primitive types, `divide`,
`dividend`, `divisor`, `lib`, `tests` and `zeroes`. The earlier list had them, and no
identifier of the standard library contains them.

## What is not judged

A number is not a word, so a source that contains `20` is not asked to define it.

A `#[cfg(test)] mod tests` is not judged at all. `docs/RUST_STYLE.md` requires a test
to be named as a sentence, and a sentence is made of ordinary English, so the two
rules would otherwise contradict each other. What ships is judged and what proves it
is not.
