# language

The library behind the gate of the ubiquitous language, shared by plundercall and
Igris. Each repository that uses it keeps its own contexts and hands them over.

## Working here

```
nix develop
moon run :ci
nix build .#book .#language-book
```

`nix develop` opens the devshell with every tool the repository uses. `moon run :ci`
builds every gate, and `docs/GATES.md` lists what each of them refuses. The library
has its own book in `crates/language`.

`CLAUDE.md` contains the rules for anybody who contributes, human or agent.
