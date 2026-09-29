# LANGUAGE, how work is done here

This repository contains the library behind the gate of the ubiquitous language. It is
shared by plundercall and Igris, so a change here changes both. The rules below are the
rules of both.

## The two rules everything else follows from

**Never guess and never invent.** Every value has a source. The human stated it, a
tool requires it, or it was measured. A value taken from another repository is not a
source. It is another location where somebody once put it. Look it up. Where there is
nothing to look up, stop and report it.

**Nothing is created without approval.** Not a directory name, not a variable name,
not a version, not a default. Propose it, name what you chose and why, and wait. A
file that exists without approval is removed, not defended.

## Verification

Nothing is reported done because it compiles. It is done when it was run and the
output is in the answer. "It should work" is not a result. Where a gate is added, make
it fail on purpose once, then restore it, and report what it printed.

Where a claim turns out wrong, correct it plainly. Do not narrate the mistake at
length.

What is not proven is not put in a document. A document states what is true and
nothing about what was believed on the way. Where the behaviour changes, the page that
describes it changes in the same commit.

## Read your own code before anybody else does

A signature is a claim, not a mechanism. Before code is shown to anybody, every
signature and every name in it is read as a sentence and put to these questions, each
answered by putting in the simplest value.

What does this signature claim when it is read as a sentence, and is that sentence
true. What happens at the emptiest value there is, at `""`, at the empty list, at
zero. Can this type come about in a state in which it is not true. Does anything here
know something it has no business knowing. Is this named for what it is, or for how it
was built.

A doubt that can be named can be answered. So a doubt is answered and acted on before
the code is shown, and it is reported all the same, together with the answer.

## Code

`docs/RUST_STYLE.md` is the style of every language in this repository. It is read
before a name is chosen, not after a gate refuses one. Crates contain no comments.

Nothing in this repository speaks. A program requests and responds. A gate refuses. A
value never does. What is in hand after something did not work out is an `Error`.

**Errors are information and are never destroyed.** A failure is propagated so the
caller decides, or the design changes so the bad state cannot occur. It is never
turned into a default, a `false`, a silent return, `let _ =`, `2>/dev/null` or
`|| true`.

**Bash control flow is `if`.** `&&` and `||` are never used for control flow.

**No command in the foreground takes longer than 20 seconds, and nothing polls.**

## Where things live

The root is `crates`, `docs` and `nix`. `docs` is the book of the repository. The
library has its own book in `crates/language/docs` beside its `README.md`.

## Nix

The flake output structure is the directory structure. An output lives at
`nix/flake/<output>`. Every Nix file is a module, `<name>/default.nix`. Every
derivation is called through `callPackage`, and `runCommand` is not used. `inherit`
is used wherever it applies, and `with` is never used. The flake root is `self`.

## Tools

Every tool is in the devshell. `nix run` at a prompt is forbidden. Use what the
repository provides, `nix fmt`, `cargo fmt --all`, `nix flake check`, `nix develop`
and `moon run`. A moon task is named for the tool it runs, the parts separated by a
slash. Files are created and changed with the file tools, not with heredocs and not
with `sed -i`.

## Commits

Commits are signed with a hardware token that needs a physical touch. An agent does
not commit and does not push. Write the change, run the gates, report, and the human
commits. Staging is allowed, because a flake in a git repository cannot see an
unversioned file.
