# xorstring

`xorstring` is the implementation of
[`Malware related compile-time hacks with C++11` by LeFF](http://www.rohitab.com/discuss/topic/39611-malware-related-compile-time-hacks-with-c11/)
but for Rust Nightly (2018 edition).

This XOR encrypts byte-string literals at compile-time with a
XOR-cypher, then decrypts them at runtime. This circumvents simple
`.rodata`, `.data`, and `.text` string checks by anti-cheats such as
Valve Anti-Cheat, in that it can store plain-text internal data
without the anti-cheat noticing it.

## Why nightly?

Because of [`proc_macro_hygiene`](https://github.com/rust-lang/rust/issues/54727).

## Where is the under-the-hood implementation?

Here: [xorstring-procmacro](./xorstring-procmacro)

## Licence

The project is licensed under the [BSD 3-Clause Licence](./LICENCE).
