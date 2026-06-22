# Lesson 3 complete — tests & the feedback loop

The learner finished Lesson 3 (`0003-tests.html`) independently, completing **Module 1** (a correct, error-handling, *tested* `rwc` CLI).

**What they built:** Extracted a pure `counts(&str) -> (usize, usize, usize)` from `main`, then a `#[cfg(test)] mod tests` suite (ASCII, Unicode-aware, empty-input). `cargo test` → 3 passed; `cargo clippy` clean; `cargo run` output unchanged.

**Signal on their level (informs future lesson pitch):**
- Carried the Lesson 2 error handling forward verbatim (`match` on `args().nth(1)` + `exit(2)`, `map_err` context) instead of reverting to the lesson's trimmed `main`. Reads the listings as guides, not gospel — good.
- Wrote *original* test cases and hand-predicted the counts correctly (incl. `\n` counted as a char). Solid grasp of the `.chars()` vs `.len()` byte/char distinction.
- Asked sharp meta-questions before doing the work: unit-vs-integration test conventions, and how doc-tests work. Wants the *why* and the ecosystem context, not just the mechanics.

**Commitments captured (so they aren't dropped):** doc-tests → Module 2 (when `src/lib.rs` appears; binary crates skip doc-tests); `assert_cmd` integration tests → Module 3 Lesson 10 (as the dev-dependency example). Recorded in `NOTES.md` "Deferred testing topics".

**What this means going forward:** Can pitch slightly denser; keep explaining Rust-specific syntax on first appearance (per [[MISSION.md]] / NOTES teaching prefs), but no need to belabor general programming. Tests now exist as the refactor safety net for Module 2 (borrowing). Next: author + start Lesson 4.
