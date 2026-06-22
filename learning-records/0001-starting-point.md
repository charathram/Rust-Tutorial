# Starting point: veteran polyglot, new to Rust specifically

**Established prior knowledge:** 30+ years of programming; fluent in C, Java, Python, Go (and BASIC/FORTRAN/Pascal). This means stack/heap, pointers, manual `malloc`/`free`, double-free, and GC are all already understood — they are *anchors* to teach against, not things to teach. Programming fundamentals (control flow, functions, data structures) can be assumed.

**Rust-specific level:** beginner. Has written `hello` (hello world) and `hello2` (array of strings, `.iter()`, a `&` reference) — so has *seen* references and iteration but has no model of ownership yet.

**Why it matters for future sessions:** Pitch lessons at "experienced engineer learning a new memory model," not "new programmer." Lead with the *distinctively Rust* idea and contrast it to C (manual free) and Go (GC). The borrow checker — not syntax — is the real learning curve, so the early arc (ownership → borrowing → slices → error handling) is the priority before the CLI/web mission tracks.

**Evidence:** self-reported experience; observed `hello2/src/main.rs`. Ownership understanding not yet demonstrated — Lesson 0001 issued; await exercise/quiz evidence before recording it as learned.
