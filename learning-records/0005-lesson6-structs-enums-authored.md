# Lesson 6 authored — structs & enums (+ doc-tests, lib/bin split)

Authored `lessons/0006-structs-enums.html` and built+verified project `0006-rwc/` (Module 2, effort M). Triggered by the learner saying "Lesson 5 is done. Let's create lesson 6."

**Shape:** Build-along (back to the L1–L4 format after L5's diagnostics detour). Refactors the finished L5 tool under green tests. Five steps: (1) start `src/lib.rs` with a `Counts` struct; (2) add a `Source` enum; (3) make `main` a thin client; (4) document with doc-tests; (5) move tests over + run everything.

**What it teaches (three interlocking new things):**
- **structs** — `Counts { lines, words, chars }` replaces the `(usize,usize,usize)` tuple. `#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]`, `impl` block with an associated fn (`from_text`) and a `&mut self` method (`add`, replacing L4's free `add_into`). Named fields as self-documentation is the motivation.
- **enums** — `Source { Stdin, File(String) }` models input origin. `from_args` (empty → `[Stdin]`, else map paths via `.map(Source::File)` — variant-as-constructor), `name`, `read`. **This delivers stdin** (`echo x | rwc` → labelled `-`), i.e. the module-checkpoint stdin requirement, early.
- **doc-tests** — the deferred-from-L3 topic. Moving logic to `src/lib.rs` is what *enables* doc-tests (they run only for library crates — ties back to the L3 binary-crate caveat). Covers `///` runnable examples, hidden `#` setup lines, the turbofish `# Ok::<(),_>(())` trailer for `?`-using examples, and all four fence annotations. `no_run` is used **for real** on `Source::read` (reading stdin would hang the test run) — an authentic motivation, not a toy. `ignore`/`should_panic`/`compile_fail` covered in a `.compare` concept box + as optional extensions.

**Accuracy:** all terminal output is **real**, captured from the built project against `rustc`/`cargo` 1.96: `cargo test` = 6 unit + 5 doc-tests pass (doc-test line numbers 16/38/54/87/122 match the file; `Source::read` shows `- compile ... ok` for `no_run`); `cargo run` on two files + `echo | cargo run` for stdin; `cargo clippy` clean (had to drop the `|p| Source::File(p)` closure to bare `Source::File` to satisfy `redundant_closure` — turned into a teaching point).

**Key design decision — L7 re-anchor:** L6 needs a 2-arm `match` in `name`/`read` (you can't use an enum without eventually matching), and the clean-compile requirement forced stdin to actually work in L6 (an unconstructed `Stdin` variant would trip `dead_code`; constructing it means reading it means dispatching = match). So L6 introduces `match` minimally with a one-line forward-ref. To avoid making L7 redundant, **re-anchored L7** from the syllabus's original "dispatch on Source to read file/stdin" → "the same `match` in *depth*: exhaustiveness as a compile-time guarantee (add a variant, watch every match fail), guards, bindings, `if let`/`let else`." Propagated to `nav.js` + syllabus L7 description + NOTES arc. This is a stronger, more distinctively-Rust L7 anyway.

**Pedagogy aligned to NOTES:** neutral labels (Lesson goals / Outcome / Optional extensions / Check your understanding); every new symbol explained first-time (`pub`, `impl`, `&mut self`, `#[derive]`, `match`, `use rwc::`, `io::{self, Read}`, turbofish, `#`-hidden doc lines); struct-vs-enum "and vs or" framing; C/Go contrast (struct=record but owns fields; enum=tagged union, C unions lack safety, Go lacks sum types). 3 quizzes (struct-vs-enum, why-lib-for-doctests, no_run choice).

**Housekeeping:** `nav.js` LESSONS += L6; syllabus L5 → Complete, L6 → Next (linked), L7 desc rewritten, footer "Current ▸ Lesson 6"; glossary += struct, enum, derive, library-vs-binary crate, doc-test; NOTES arc updated (L5 ✅, L6 ✅ authored, L7 NEXT + re-anchor note).

**Next:** Lesson 7 — `match` in depth on the `Source` enum (exhaustiveness, guards, bindings, `if let`/`let else`). Then L8 strings/slices, L9 lifetimes to finish Module 2.
