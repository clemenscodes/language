# What it is made of

The crate is laid out by layer. Every module is private, and a layer publishes the
types that cross out of it in its own `mod.rs`, which `docs/RUST_STYLE.md` sets out
in full. `lib.rs` is the edge of the crate and publishes the six names a repository
uses.

```
core            the domain and the ports it is driven through
  language      a vocabulary, and the rule a source either satisfies or does not
    vocabulary  a set of words, with the difference and the union of two
      word      one word, kept verbatim, compared without regard to case
      source    a Rust file taken apart into words, past strings, comments and tests
      term      a spelling and its definition
        spelling     the words one term is made of, and its identity
        definition   what a term means
  languages     the shared language, the contexts of a repository and the foreign languages
    context     one directory under a root, and the words that directory means
    foreign     one third party crate, and the words it brought with it
    rust        the words of Rust itself
  book          one context's words as a page, and where that page belongs
  dependencies  the crates a manifest names
  port          the seams a source is read through and a book goes out through

application     what the presentation drives
  command_line  the sources as arguments, parsed by clap, and the response to them
    service
      gate      reads a source and yields its errors or lets it through
      press     prints the book of a context out of its vocabulary
    outcome     what the process exits with

infra           the world facing edge
  file_system   the adapter that reads a file and writes one
```

`core` publishes `Language`, `Error`, `Context`, `Foreign` and `Languages` beyond the
crate, and `Book`, `Dependencies`, `Reader` and `Writer` within it. `application`
publishes `CommandLine`. `infra` publishes no type at all, only the two functions that
yield the adapter it chose, so nobody outside it can name `FileSystem`.

A `Source` gives the words a text contains. A `Language` gives the words it allows.
The difference of the two is the errors of the source, which is why `Vocabulary`
implements `Sub`, and why two languages are joined with `BitOr` rather than with a
name somebody invented.

`Language` is a `Specification` over a `Source`, so the rule of this library is
one method that decides whether a source is in the language.

## The two things it does

The gate reads and judges. The press reads nothing and writes, taking the terms of
a context and printing them as that context's page. They share the languages and
nothing else, which is why they are two services beside each other rather than one
with a flag inside it.

A `Book` is a page and the path it belongs at, and it is `Display`, so what the press
hands the port is the file itself. The path follows from the context rather than from
an argument, because a book that could go anywhere would be a book the gate
could not find again.

## What follows from the rules

A term without a definition is not a term, a spelling without a word is not a
spelling, and an empty word is not a word, so every conversion that can fail is
`TryFrom` and the error names what could not be made.

A `Language` has no identity and two languages are joined into a third, so it is a
value object. A `Term` keeps its spelling when its definition changes, so it is an
entity identified by that spelling.

`Languages` comes about from the contexts and the foreign languages of a repository.
It adds Rust's own words and nothing else, and the only
way that can fail is a vocabulary of the library that is not a vocabulary, which is
why the one error it can end in is the `Error` of a `Language`.

The gate writes nothing. It responds with `Error::Language`, which names the source
and the words it contains that its language does not have, and the command line
prints that to the error stream and decides the exit code. The adapter reads the file,
and that fails with an `io::Error`, which the gate translates into `Error::Content`,
so the infrastructure never knows a word of the domain. The press fails the same way
through `Error::Book`, and the command line prints both to the error stream and yields
`Outcome::Error`, because the status of a process that did not do its work is the same
whichever of the two stopped it.

Each of the two services has an error of its own, and every variant of it is the
thing that was not there rather than the act that failed. The gate has two, a source
not in its language being an `Error::Language` and a file whose bytes could not be
read an `Error::Content`. The press has three, a book that never came to be being an
`Error::Book`, a context that yields no term at all an `Error::Vocabulary`, and a
directory no context covers an `Error::Directory`.
