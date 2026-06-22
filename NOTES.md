# Notes

Working notes and learner preferences. Keep terse.

## Learner profile
- 30+ years programming. Fluent: BASIC, FORTRAN, Pascal, C, Java, Python, Go.
- C background → already understands stack/heap, pointers, manual `malloc`/`free`, double-free. **Teach ownership by contrast with C (manual free) and Go (GC).**
- Skip programming fundamentals (variables, loops, functions-as-concepts). Focus on what is *distinctively Rust*.

## Teaching preferences
- **Project-first / learn-by-doing** (stated after Lesson 1). Build small useful utilities; introduce concepts *only when the code forces the question*. Do NOT front-load ownership/borrowing/lifetimes as standalone theory lessons — teach them in context as they arise.
- Prefers **short, hands-on** lessons; one tangible runnable win each.
- Wants the "why"/mental model briefly, then build. Keep theory callouts to a sentence or two inline.
- **Explain every new symbol the first time it appears** — don't assume Rust syntax/idioms are obvious just because the user is a veteran (e.g. `use`, `::`, `!` macros, `{var}` interpolation, format specs, closures `|x| …`). Prefers dense lines broken down piece-by-piece. (Stated re: Lesson 2 — "dumb it down just a bit more.") This is about *Rust-specific* unfamiliarity, not general programming — never condescend on fundamentals.
- Avoid heavy questionnaires up front — keep grounding lightweight and conversational. (User declined a multi-question form early on; conversational questions worked.)

## Tone & structure (IMPORTANT — applies to every lesson)
- **Professional, but encouraging.** No gamified/breezy labels — banned: "Your win", "The payoff", "Make it yours", "let the compiler teach you", "build with me", etc. Prefer neutral labels: "Lesson goals", "Outcome", "Optional extensions", "Check your understanding", "Connection", "Concept".
- **Every lesson opens with a `Lesson goals` section** — a short bulleted list of what the reader will be able to do by the end. This is where "what the lesson teaches" goes.
- **Subheadings state the task/concept**, not the method or vibe. Bad: "Count, like wc — the compiler is your feedback loop". Good: "Step 3 — Count lines, words, and characters".
- **Prefer bulleted/numbered lists over long prose** for structure and for recapping things already understood — but see the clarity rule below.
- **When introducing a NEW concept, err on the side of clearer, even if longer.** ⚠️ Concept-introduction bullets must be full, self-contained explanations: state *what it is*, *what it does here*, and *why/when it matters* — in complete sentences, not terse fragments that make the reader fill gaps. Never trade clarity of a new idea for skimmability. (Stated re: the error-handling lesson's Step 1 "what each change does" list being too terse.) This rule outranks "keep it short" wherever they conflict. Applies to every lesson, retroactively.

## Conventions
- **Lesson code lives in a directory prefixed with the lesson number**: `NNNN-<name>/` (e.g. `0001-rwc/`), aligning the cargo project to its lesson file `lessons/NNNN-*.html`. The cargo *package* name can stay a clean slug (e.g. `rwc`) — only the directory carries the number.
- **Every lesson links the shared assets**: `assets/style.css`, `assets/quiz.js`, and `assets/linenumbers.js` (the last auto-adds line numbers to all `<pre><code>` blocks via CSS counters — numbers aren't selectable, so copied code stays clean). Don't inline styles/scripts a future lesson would duplicate.
- **Shell/terminal blocks use `<pre class="shell">`** — they are NOT line-numbered (only source code is) and get a neutral gray accent instead of the rust accent. Rust source blocks stay plain `<pre>`.
- **Lesson pages link `assets/nav.js`** which injects the left-margin lesson index (fixed rail on wide screens, stacked block on narrow, hidden in print). **Single source of truth for the course outline: the `LESSONS`/`REFS` arrays in `nav.js`.** When adding a lesson, append it there — do not hand-edit each page's nav.
- **In build-along lessons, mark lines added/changed in a step** by wrapping the *whole line* (indentation included) in `<span class="add">…</span>`. `linenumbers.js` promotes it to a green-highlighted `.code-line.add`. Include a one-line legend (`.add-swatch`) the first time it's used. Only highlight what changed vs. the previously shown version — not the baseline first listing.

## Environment
- Rust 1.96.0, cargo 1.96.0, rustup 1.29.0 installed.
- Workspace is a private GitHub repo `charathram/Rust` (SSH). Commit identity: charathram / charath@pragmatik.tech.
- Existing scratch projects: `hello/` (hello world), `hello2/` (array of Tamil/Hindi greetings, `.iter()`, `&` reference).

## Course arc (PROJECT-FIRST)
> Full published syllabus: `lessons/000-syllabus.html` (linked at top of the left nav). Keep syllabus + `nav.js` + this arc in sync when lessons are added/reordered.

**Numbering (after the full renumber):** Orientation is Module 0, file `0000-ownership-and-moves.html`, NOT a numbered lesson. The numbered build sequence starts at Lesson 1. Lesson code dir = `000N-rwc/` aligned to lesson number.

- **Module 0 · Orientation primer** ✅ — `0000-ownership-and-moves.html`. Ownership model as a read-once primer (slimmed, no exercise). Set apart in nav (○ glyph) and syllabus (Module 0).
- **Lesson 1 · Build a CLI `rwc`** ✅ — `0001-build-rwc-cli.html`, project `0001-rwc/`. cargo, `env::args`, `fs::read_to_string`, `Result` via `expect`, string iterators; ownership shows up as `&path`.
- **Lesson 2 · Error handling** ✅ — `0002-error-handling.html`, project `0002-rwc/`. `main -> Result<(), Box<dyn Error>>`, `?`, `ok_or`, stderr, exit codes (1 vs panic's 101). std-only; `anyhow` deferred to the dependencies lesson.
- **Lesson 3 · Tests & the feedback loop** ✅ — `0003-tests.html`, project `0003-rwc/`. Extract pure `counts(&str) -> (usize,usize,usize)`, unit tests in `#[cfg(test)] mod tests` (`#[test]`, `assert_eq!`, Unicode-aware case), `cargo test`. std-only; integration testing (`assert_cmd`) flagged for later. **Module 1 complete.**
- **Lesson 4 · Borrowing & references** ← IN PROGRESS (Module 2) — `0004-borrowing.html`, project `0004-rwc/`. Feature driver: rwc counts *several* files + prints a grand total. Teaches shared `&` (recap `&text`; `&paths` so the Vec survives the loop), mutable `&mut` (a `add_into(&mut total, …)` accumulator), and the shared-XOR-mutable rule (+ no-dangling → lifetimes L9). C/Go contrast. Lesson 3 tests stay green as the refactor safety net; added an `add_into` test. std-only. **Authored + verified (4 tests pass, clippy clean, multi-file/error/no-arg paths checked); learner now working through it.**
- **Lesson 5 · Reading borrow-checker errors** ← NEXT — real `E03xx` errors hit pushing rwc further. Project `0005-rwc/`. (Module 2 batch continues: L6 structs/enums, L7 match, L8 strings/slices, L9 lifetimes.)

### Deferred testing topics (committed — do not drop)
> Flagged as Lesson 3 "optional extensions"; user explicitly asked that both be covered properly in future lessons. Anchor them to these steps when authoring:
- **Doc-tests** → **Module 2**, at the step where `rwc` gains `src/lib.rs` (≈ Lesson 6, structs/enums). Doc-tests only run for *library* crates, so this is the first lesson where a `///` example on `counts` actually runs under `cargo test`. Cover the `///` block, `#`-prefixed hidden setup lines, and fence annotations (`no_run`/`ignore`/`should_panic`/`compile_fail`). Tie back to the binary-crate caveat noted in Lesson 3.
- **`assert_cmd` integration tests** → **Module 3, Lesson 10** (Cargo/dependencies). Use it as the worked example of a *dev-dependency*: `tests/cli.rs` running the compiled binary as a subprocess, asserting on stdout/stderr/exit codes. Connects to the `tests/` dir + unit-vs-integration split introduced in Lesson 3.

### Batch-build plan (user asked to generate remaining lessons "properly, in module batches")
Author + verify one module at a time; user says "continue" between batches. Done: Module 0, Module 1 (L1–L3). Next batch: **Module 2** (L4 borrowing, L5 borrow-checker errors, L6 structs/enums, L7 match, L8 strings/slices, L9 lifetimes) — all anchored to growing rwc into a multi-file + stdin tool, verified with `cargo test`/`cargo run`. Then Modules 3–8 per syllabus. Each lesson: grounded citations, compiled code, glossary + nav + syllabus updates.

Full reworked arc (post-critique v2, 8 modules / **35 numbered lessons** + Module 0) lives in the syllabus: measurable outcomes, effort tags (S/M/L) + aggregate time (~50h lesson work, 70–90h total), per-module **prerequisites**, checkpoints with acceptance criteria, two **interleaved cumulative checkpoints** (after Modules 4 & 6), spaced-review steps, community touchpoints, reproducibility (toolchain pin), staged capstone. Notable structure: testing at L3; clap after structs; smart pointers/interior mutability, a borrow-checker-errors skill lesson, static-vs-dynamic dispatch, and an early lifetimes lesson (Module 2) all added; async/traits/serde/sqlx each split into two; Module 2 re-anchored to a real feature driver (rwc → multi-file + stdin).
