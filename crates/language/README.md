# language

The library behind the gate that turns away an identifier made of a word the context
it is in does not have.

A repository that uses it keeps its own contexts in a small crate of its own and hands
them to the library, which reads the sources, judges them and prints the books.

Its book is in `docs`, and it covers what the library reads, how a repository uses it,
what it is made of and where the words live.
