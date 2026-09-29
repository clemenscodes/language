# Rust style

These rules are binding, always. They are not reminders and they are not
preferences. Code that breaks one of them is not finished.

The rules come from the plundercall repository and are the same in Igris. Where a
rule names a gate, the gate is listed in `docs/GATES.md`.

## Names

Every identifier is English. Every identifier states its full meaning. No
abbreviations, no single letters, no shortened forms.

```rust
struct KeyValue { }
let guest_state = guest.state();
let mac_address = tap.address;
```

An acronym is written out, always, wherever it appears.

```rust
struct DomainDrivenDesign { }
```

A proper name that is only ever written as letters is not an acronym and stays as
it is.

A name is what a reader gets instead of a comment, so it has to be enough on its
own.

## NEVER WRITE PARTIZIPIEN OR PROVERBS !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! EVER!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

A VARIABLE MAY NEVER EVER BE NAMED `written`, `taken`, `waiting` and such. Do not bother writing that garbage. it will be rejected and you will be flamed for that atrocious garbage.

**Every value is a noun for what it is, and never for what an action left behind.**
A method may be a verb in the present tense, `read_file`, `write_script`,
`refuse_instruction`, because a method acts. A value never acts, so it is never named
for an act that touched it. `reads`, `refuses`, `writes`, `written`, `found`,
`given`, `wanted`, `shown`, `kept` and `settled` name nothing, because each of them
says only that something happened and leaves out what the value is. The question that
settles it is whether the sentence "this value is a ..." can be finished with the name
alone. Where it needs a verb to finish, the name is wrong.

```rust
let text = fs::read_to_string(path)?;
let names = findings.len();
let addresses = location.try_addresses()?;
```

**This binds every language in this repository, not only Rust.** Nix and shell are
code of this repository and answer to this page.

Nothing in this repository speaks. A program does not say, tell, speak, ask or
answer. It requests and it responds. A name states a meaning, a type constrains
a value, a test proves a behaviour, and a gate refuses. People speak, ask and
answer.

Every word of an identifier belongs to the language of the context it is in, or the
identifier is wrong. Nothing is banned, because every word means what it means, and
a word whose meaning does not occur in this project is simply the wrong word for
what was named.

## Domain types, never primitives

A primitive never means a domain concept across a function boundary or in a
struct field. If a value means something, it is a named type, and that type is
what is passed, stored and returned.

```rust
fn bind(port: Port) -> Listener { }
struct Endpoint { host: Hostname, port: Port }
```

The primitive appears in exactly one location, inside the type that gives it
meaning, in its constructor and its accessor.

The test: if a caller can pass the wrong number for the concept and it still
compiles, the type is missing.

A `bool` that names a domain state is a primitive too. It becomes a two variant
enum, `DiskPresence::{Present, Absent}` rather than `disk_exists: bool`. The only
`bool` that stays is a flag on a clap `Args` struct, which is clap's interface.

The exception are primitives on which calculations have to be done. An `Offset` or
an `Address` may stay `usize`, because that keeps the code simple and avoids useless
conversions wherever calculations happen. A `Hostname` is never calculated, so it
must be a struct.

## Wrap every result and every intermediary

A computation's result is a named struct, even with one field. It costs nothing
and it is what lets the value implement `From`, `Display`, `IntoIterator` and
`FromStr`. This matters most for a `Vec` and for any foreign type, where wrapping
is the only way to implement a trait at all.

## No tuples

No plain tuples, no tuple structs, no newtype wrappers. A named struct with named
fields, always.

```rust
struct GuestMemoryRange { start: u64, length: u64 }
```

There is one exception and it is the only one. The argument of `From` or `TryFrom`
may be a tuple, because a constructor takes exactly one argument and a thing built
out of several values has no other way in. It is destructured in the signature, so
the parts are named where they are read.

```rust
impl<W, D> TryFrom<(W, D)> for Term {
    fn try_from((spelling, definition): (W, D)) -> Result<Self, Self::Error> { }
}
```

A tuple anywhere else, in a field, a return type, a variable or any other argument,
is the mistake this rule is about. A tuple that a std or third party API hands over,
such as the pair of `.enumerate()`, is destructured into named bindings on the spot
and never stored, returned or built by hand.

## Files and modules

Every file is `lib.rs`, `main.rs` or `mod.rs`. There is no fourth name. A concept
is a directory and the concept lives in that directory's `mod.rs`.

A `mod.rs` defines exactly one concept. One struct, or one enum, or one trait.
Its inherent impls, its trait impls and its tests belong in that same file. A
second struct means a second directory.

If two sibling modules share the first word of their name, that word is a module
and both belong inside it. Apply it again inside whatever that produces.

Every child of a directory is the same kind of thing as its siblings. Name what
the children are in one noun phrase. If the sentence needs "and also", something
belongs one level down, inside a container named for what it is.

There is no module called `util`, `utils`, `helpers`, `common`, `misc` or
`shared`. Those names mark a concept that was not found.

## A layer publishes, a module does not

A layer publishes the types that cross out of it, one `pub(crate) use` per type, in
the `mod.rs` of the layer and nowhere else. That is the layer's interface and it is
the only re-export there is.

```rust
mod error;
mod language;
mod port;

pub(crate) use error::Error;
pub(crate) use language::Language;
pub(crate) use port::Reader;
```

Every other module keeps its children to itself, and reaches no further than its own
child when it does publish. A re-export that passes a type up through one module
after another is the thing this forbids, because after two of them nobody can tell
where a type lives, and after four the crate has a flat surface that a caller reaches
with a glob. There is no glob import anywhere, and no prelude.

**The layer answers to that rule as well and reaches no further than its own child.**
`pub use archive::files::file::File` names where `File` lives, and then hands the
caller `crate::File`, which names nothing. The section after the next one sets out
what the layer publishes instead.

Where a crate is a library, `lib.rs` publishes what the domain layer publishes, one
`pub use` per name. That is the one location a name appears twice and it is not a
chain, because `lib.rs` is the edge of the crate rather than a module inside it.
Every other file answers to the rule above, and the layer under `lib.rs` publishes
with `pub` rather than `pub(crate)`, because what it publishes leaves the crate.

A private `use` is ordinary Rust and is used freely. What is forbidden is passing
somebody else's item along, not importing one for this module's own work.

## The qualifier goes on the type, not on the module

A type is given the visibility it needs. A `pub(crate)` type inside a private module
is ordinary and right, so a caller receives it, binds it and calls its methods while
being unable to name its path.

```rust
mod vocabulary;

pub(crate) struct Vocabulary { }
```

A module is opened only to keep its name in the caller's path. That is all of the
test. When a caller writes `languages::Rust`, the name reveals which sack the type
came out of, and the module is opened so that it can. When a caller would write
`Rust` alone, the name reveals nothing, the module was opened to flatten a type out
of it, and that is the mistake.

Where a caller cannot name what it needs and no namespace belongs in its path, the
caller is in the wrong location. The compiler is showing that this code reaches into
somebody else's insides, and the answer is to move it under whatever owns the thing,
or to let the owner publish it instead.

## A layer publishes concepts, not a heap of types

What crosses out of a layer is a concept, not the list of types that concept is made
of. So the layer opens each of its leading concepts as a module and lets the concept
publish its own types.

```rust
pub mod generation;
pub mod service;
pub mod system;

mod error;

pub use error::Error;
```

**The leading concept is a namespace, and the type at the head of it is named for that
concept.** `generation::Generation` and `service::Service` are right and they are not
a repetition. The path names the concept, the type names the thing, and the two words
being the same is precisely what reveals that this concept has one type at its head.

The error is the one thing a layer publishes as a bare type, because it belongs to no
concept and leaves every one of them.

Inside a concept the rule above still decides. A module is opened only where its name
earns a position in the path. The exception is a type that lives two below the
concept, because a module may publish no further than its own child, and then the
module between them is opened and the path grows by the one step the tree actually
has.

## Visibility

Every type is private until something outside it fails to compile. Then that type
widens by exactly one step, `pub(super)` first, `pub(crate)` next, bare `pub` only
at the crate edge where another crate consumes it. A module is opened only under
the rule above.

## Fields and accessors

Struct fields are private. Read access is a getter, named for the thing it returns
and nothing more. Write access is a setter or a builder.

## Calls

Assign a struct to a variable before passing it.

```rust
let tap_device = TapDevice { index: 0, mtu: 1500 };
launcher.attach(tap_device);
```

Assign any expression to a variable before passing it. A call receives plain
variables, never inline expressions, however short.

```rust
let virtual_base = guest.virtual_base(region);
let page_size = guest.page_size(region);
let translation = table.translate(virtual_base, page_size);
```

## A variable is a noun, a function is a verb

**Every variable is a noun, because a variable is named for what it is.** A binding, a
field and an argument all answer the question what is this, and the answer to that
question is a thing. A variable named with a verb is wrong, and a variable named with
an adjective is worse, because an adjective is not even a claim about a thing, it is
half of one.

**Every function is a verb, because a function is named for what it does.** The only
exception is a getter, which is a noun and returns the thing it names, and that is
all of the exception.

**A verb acts on a noun, and a word that only asserts something about an argument is
not one.** `describes`, `covers`, `mapped` and `readable` name no work and no thing.
They come about where a function was made to answer yes or no and a word was invented
for the question. The answer is to return what was asked about and let the caller
read it, so `block(address)` answers with the memory block or with nothing, and the
invented word is gone along with the second read it was doing. A verb in the present
tense stays right where the subject really does the thing, so an instruction `loads`,
a path `compiles` and a line `measures`.

**A preposition is no name.** `at`, `of`, `over` and `behind` state where one thing is
in relation to another and never what it is. `rest` answers nothing either, because
the rest of what is the question the name has to answer.

**Replacing a refused word with another empty one is not a rename.** `at` became
`where`, which is an adverb, and the code said exactly as little afterwards. A rename
is finished when the sentence "this value is a ..." can be finished with the name, and
until then the word that was there is as good as the word that replaced it.

**An adjective is allowed only in front of a noun.** `decrypted_asset` names which
asset and is a name. `decrypted` alone names which what, and is not.

```rust
let text = report.to_string();
let stated_place = header.tables();
let bare_name = candidate.chars().all(|letter| !letter.is_whitespace());
```

Where a value is the outcome of doing something, the noun for it is the doing.

**A name is never shadowed by a second value.** Two values under one name are two
things a reader has to keep apart without being told which is which, and the only
way out is to name the second one. Where no name comes to mind, the value has a
role in whatever is being computed and that role is its name, so a division has a
dividend, a divisor and a remainder rather than three shadows of `bytes`.

The words this repository refuses outright are `written`, `given`, `at`, `of`, `over`,
`wrote`, `number`, `found`, `one`, `other` and `later`. Each of them names nothing.
`found` reports only that a search ended. `one` and `other` report only that there are
two of something, and neither reports more inside a compound, so no name has one of
them in it either.

A number is refused as a name of its own. It counts and never states what the value is,
so `let four = ...` is wrong and the two operands of a bit operation are the value and
the `mask` it meets. An ordinal before a noun stays, because there the number is the
only true statement about the thing.

A refusal in this code is an error. The type is `Error`, the field is `error`, and
every name and every test name follows, so a test that proves one of them ends in
`is_an_error`. The hooks under `.claude` keep the word, because a hook really does
turn a write away.

Nothing here speaks, so `say` and `speak` are refused in every writing of them. A name
states a meaning, a type constrains a value, a test proves a behaviour and a gate
refuses. And a pointer points, so `aim` is refused as well.

## Functions

A free function named `verb_noun` is a design mistake. The noun is a type and the
verb is a method on it.

```rust
impl Persona {
    fn load(path: &Path) -> Self { }
}
```

A function whose name begins with `print` is a design mistake. Implement
`Display` and let the call site print.

A function's name states exactly what that function does, so a reader who never
opens the body knows what came out and what it was made of. A name that fits any
function fits none.

A name that is a noun is a getter and returns the thing it names. A name that is a
verb does work and the verb names which work. Nothing else is a function name, so
there is no `get_` prefix and no verb on something that only yields a field.

```rust
let term = entry.term();
let reason = ban.reason();
```

`of`, `get`, `make`, `build`, `run`, `do`, `handle`, `process`, `execute`, `apply`
and `perform` are banned outright. They name no work. Where one of them was
reached for, the work has a real verb and that verb is the name.

A name a trait dictates belongs to the trait, so `from`, `fmt`, `next` and
`into_iter` are named the way the trait declares them.

## Constructors

Every constructor is `From`, and where it can fail, `TryFrom`. There is no `new`,
no `of`, no `with_something` and no free constructor function. Rust already has the
trait for turning one value into another and this project uses it.

A constructor that needs several values takes one named struct made of them, and
that struct is what `From` receives.

```rust
struct Refused {
    word: Word,
    path: PathBuf,
    reason: Reason,
}

impl From<Refused> for Refusal {
    fn from(refused: Refused) -> Self { }
}
```

This is why every intermediary is a named struct. The struct is the argument list,
it states the meaning of what went in, and the conversion is the standard trait
rather than a name somebody invented.

A constructor that needs nothing is `From<()>`. Unit is the absence of an argument
rather than a tuple of values, so this is not the tuple the rule above forbids.

```rust
impl From<()> for Language {
    fn from(_: ()) -> Self { }
}
```

**Only `From` answers with `Self`, and only `From` takes one.** Before the
conversion the thing is not there yet, so no other function can be handed one or
hand one back. What the computation before it works on is whatever the type is made
of, and that is what those functions take and answer with.

The one exception is a `const` that has to exist at compile time. `From` is not
`const`, so a type that is a constant has a `const fn new` beside its conversions,
and that is the only location the name `new` appears.

## Traits

Derive every trait the type mechanically supports, in this order: `Clone`,
`Debug`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord`, `Copy`, `Default`.
`Copy` for anything cheaply bitwise copyable. `Hash` always accompanies `Eq`.
Where a derive gives the wrong default, write the impl.

If a standard trait exists for what you are doing, implement it.

| Concept | Trait |
|---|---|
| Human readable text | `Display` |
| Infallible conversion | `From` |
| Fallible conversion | `TryFrom` |
| Zero or empty value | `Default` |
| Equality | `PartialEq`, `Eq` |
| Ordering | `PartialOrd`, `Ord` |
| Iteration | `Iterator`, `IntoIterator` |
| Parsing from text | `FromStr` |
| Errors | `std::error::Error` |
| Indexing | `Index`, `IndexMut` |

Inside an `impl` block the type is `Self`, in return types, constructors,
associated calls and patterns alike.

## Errors

Every error is information, and destroying information is forbidden. A failure
carries the reason, the location and the context of what is broken. Turning it into a
default value, a `false`, a silent return or a discarded result makes the broken state
look like a success.

A `Result` is propagated with `?` or handled in a branch that converts or reports the
error. `let _ = ...` and `.ok()` on a `Result` are forbidden, and so are
`unwrap_or(false)`, `unwrap_or(true)` and `unwrap_or_default()` on one. Where a
missing file means absence rather than failure, `ErrorKind::NotFound` is matched
explicitly and every other kind is still an error.

A subprocess that exits with a status other than zero is a failure, and every status
is checked. Its standard error is inherited or forwarded, never sent to
`Stdio::null()`, because the tool states the reason it failed and that reason is the
information.

The only tolerated suppression is inside `Drop`, where `?` does not exist, and there
the error is still reported.

## Numbers

No type suffix on a numeric literal. Annotate the binding instead.

```rust
let queue_count: u32 = 0;
let buffer: Vec<u8> = vec![0; size];
```

No underscore inside a numeric literal. A number is one number and is spelled as
one, so `0b00000101` and `1000`, never `0b0000_0101` or `1_000`.

No `as` cast outside the body of a `From` or `TryFrom` impl. Use `From`, `Into`
or `TryFrom`.

## Hexadecimal

Hexadecimal this repository forms is in upper case, and the `x` of a `0x` in front of
it stays lower case because the `x` is a marker and not a digit.

```
0x0000000000006A90
```

Hexadecimal another tool produces and reads back again stays exactly as that tool
produces it, which for `nix` and `sha256sum` means lower case. Reading is forgiving
where writing is strict, so a type that reads hexadecimal somebody else produced
accepts either case.

## Composition

When every field of a struct comes from another struct unchanged, embed that
struct as a named field rather than copying its fields.

## Purity

Deciding and doing are two functions. A decision takes what it needs as an
argument, returns what it produced, reads no clock and no file, keeps no state
between calls, and yields the same result every time. Doing what that decision states
touches the world and decides nothing.

Which program runs, with which arguments, and which file has to appear afterwards
is a decision, and it is proven by a test with plain values. Spawning the program
and looking for the file is the doing, and it is a handful of lines with no branch
that a test would want to reach.

IO lives with the thing that owns it. `main.rs` parses the command line, delegates,
reports and sets the status.

The test is whether the decision can be called from a test with plain values and no
preparation. Where it cannot, the decision and the doing are still one function and
have to be cut apart.

## One function, one thing

A function prepares and then does its one thing. The preparation is whatever the
one thing needs, and it is what the lines above the last one are for. Where a
second thing begins, a second function begins with it.

**A function works on one value.** Where two different values appear under one
name, they are two things and the name hides it. That is a missing binding, not a
reason to fold them together.

**A parameter a function only passes on is forbidden.** Whoever takes it uses it.
Where a function needs an argument so that the one below it can have it, the
sequence belongs to whoever already has the argument.

**The sequence of steps belongs to whoever drives them.** A function that puts two
steps in order is a third step nobody asked for, and it is named for one of the two
while doing both.

**Two functions with the same body and the same signature are one operation**, and
at least one of the two names is untrue.

**A function whose body is one operator does not exist.** A name in front of an
operator the type already answers to adds a word and no meaning.

## A type is what it is, not a container for a value

A value between two states is not a type. Where a name for what a fold passes
along is what is wanted, the answer is what the fold passes along, named for what
it is.

A type invented for that purpose ends up with no body, because every operation it
would answer to turns out to be an operation of what it is made of, or of what
drives it.

Where a type really is missing, its word is usually already in the language of the
context.

## Tests

Tests come first. A behaviour is expressed as a failing test, the test is made to
pass, and the code that resulted is what stays.

A test lives inline in the file of the thing it proves, in a `#[cfg(test)] mod
tests` at the bottom, and nowhere else. There is no `tests` directory and no test
file.

A test module may use `use super::*;`.

A test contains no functions. No helper, no builder, no fixture, no shared setup.
Whatever a test needs, that test states, because a helper hides the arrangement
that the test exists to show.

Every test is three blocks in this order, the arrangement, the act and the
assertion, separated by exactly one blank line and by nothing else. Within a block
the lines follow one another with no blank line between them, so the three blocks
are clear at a glance.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_word_is_kept_in_lower_case() {
        let spelling = "Harbour";

        let word = Word::from(spelling);

        let text = word.to_string();
        assert_eq!(text, "harbour");
    }
}
```

## How the tree is found

The rules above describe what a good module tree looks like. This describes how to
find it, because a tree is not designed once, it is derived, and the derivation is one
question asked over and over.

> Which concept uses this concept?

Ask it of every module. If there is an answer, the module belongs inside that
answer, and it does not belong where it was. Ask it again of what that produces,
until the question has no answer left anywhere.

The only modules that survive at the root of a layer are these two. What crosses
out of the layer, because only the layer publishes and a module keeps its children
to itself. And what two independent concepts share, because it belongs to neither
of them. Everything else has a user, and its user is its home.

### When the compiler refuses, that is the design at work

Moving a concept inward makes it unnameable from outside, because a module keeps
its children to itself and only the layer publishes. That refusal is not an
obstacle to work around by moving the type back to the root. It is the question
being asked a second time, by the compiler.

There are two honest answers and hoisting is neither of them. Either the type has
no business crossing out and the caller was in the wrong location, or the type should
not exist. Removing the type usually makes the code smaller, and that is the reason
the refusal is worth listening to rather than working around.

## Comments

Crates contain no comments. Not doc comments, not line comments, not module prose.
A name contains what a thing is, a type constrains what it is, and a test proves
what it does. Where none of those gives the meaning, the design is wrong and the
design is what changes. What a crate does for a reader lives in its `README.md` and,
where one file is not enough, in its own book.

## Reading beats searching

A rule that a search can check has already been checked, by the gate that runs the
search. What is left is what only reading finds, and naming is all of it.

Every name is read as the sentence it makes. A name that would fit any variable
fits none, so `done`, `of`, `one` and `made` are not names, they are
places where a name is missing, and they are worth no less attention than a wrong
type. A count is named for what it counts and a path for what it leads to.

The test is whether somebody who reads one line knows what it is and what it does
without a second line.

## Lints

No `#[allow(...)]` anywhere, for any lint. A lint that fires is the tool pointing out
that the code is wrong, so the code changes.

`clippy::module_inception` is fixed by not nesting. `clippy::too_many_arguments`
is fixed by composing the arguments into a named struct. `dead_code` is fixed by
removing the item or wiring it in.

If a lint genuinely cannot be satisfied, stop and ask.
