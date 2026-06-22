# Rust Resources

## Knowledge

- [Book: _The Rust Programming Language_ ("the Book") — official](https://doc.rust-lang.org/book/)
  The canonical, free, continuously-updated introduction maintained by the Rust project. Highest-trust primary source. Use for: ownership, borrowing, lifetimes, traits, error handling — nearly every core concept.
- [Ch. 4.1 — "What Is Ownership?"](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
  The ownership rules, move semantics, Copy vs heap types. Use for: the foundational mental model everything else builds on.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  Runnable, annotated examples for each language feature. Use for: "show me the syntax for X" with a runnable snippet.
- [Rustlings](https://github.com/rust-lang/rustlings)
  Official small exercises that fix-the-compiler-error, run locally. Use for: hands-on retrieval practice between lessons.
- [The standard library docs (`std`)](https://doc.rust-lang.org/std/)
  Authoritative API reference. Use for: method signatures, what a type can do, trait impls.
- [Book: _Command Line Applications in Rust_ (CLAiR) — Rust CLI working group](https://rust-cli.github.io/book/)
  Official guide to building CLIs (arg parsing with `clap`, exit codes, output). Use for: the CLI half of the mission. Start with the [15-minute tutorial](https://rust-cli.github.io/book/tutorial/index.html).
- [`cargo` Book](https://doc.rust-lang.org/cargo/)
  The build tool / package manager reference. Use for: project layout, dependencies, workspaces, profiles.
- [The Rust Book, Ch. 9 — Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
  `panic!` vs `Result`, the `?` operator, returning `Result` from `main`. Use for: the language mechanics of recoverable vs unrecoverable errors.
- [CLI Book — Error handling](https://rust-cli.github.io/book/tutorial/errors.html)
  CLI-specific error practice, including the `anyhow` crate and `.with_context(…)` for cause chains. Use for: ergonomic, user-facing errors in command-line tools.

## Wisdom (Communities)

- [users.rust-lang.org](https://users.rust-lang.org/)
  Official user forum. Beginners welcome, high-signal, experts answer. Use for: "is this idiomatic?", design feedback, stuck-on-a-borrow-checker questions.
- [r/rust](https://www.reddit.com/r/rust/)
  Active subreddit, well-moderated. Use for: ecosystem news, crate recommendations, broad discussion.
- [The Rust Community Discord](https://discord.gg/rust-lang)
  Real-time help channels (#beginners). Use for: quick interactive unblocking.

## Gaps
- Need a vetted, current web-backend resource (Axum tutorial or _Zero to Production in Rust_) before the web-backend lessons begin. To be filled when we reach that phase.
