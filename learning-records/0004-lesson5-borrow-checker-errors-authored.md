# Lesson 5 authored — reading borrow-checker errors

Authored `lessons/0005-borrow-checker-errors.html` and set up project `0005-rwc/` (Module 2, effort S). Triggered by the learner saying "Lesson 4 is now done."

**Shape (deliberately different from L1–L4):** This is a *diagnostics* lesson, not a build-along — there is no new feature and no net code change. `0005-rwc/` is a clean, clippy-clean copy of the finished L4 tool; the learner breaks it four times on purpose, reads each error, and restores the baseline. The deliverable is the reading skill, not a feature.

**What it teaches:**
- Error *anatomy* as the transferable skill — code → summary → primary span (`^^^`) → secondary labels (`----`) → `help`. Framed up front in a `.compare` box so every later error is skimmable.
- Four errors, each provoked in `rwc` and tied back to a Lesson 4 rule: **E0596** (took `&mut` of a non-`mut` binding), **E0382** (dropped `&` in `for path in paths` → use-after-move via implicit `.into_iter()`; the centerpiece, since L4 quietly avoided it), **E0502** (`peek = &total` held across `&mut total`), **E0499** (two live `&mut`).
- **NLL** ("a borrow lives until its *last use*") — explains why the "...later used here" labels matter and why removing/reordering a later use fixes things. This is the unifying mental model.
- Tooling: `rustc --explain E0XXX` + the online Rust Error Index as the authoritative fallback.

**Accuracy:** all four error blocks are **real `rustc` 1.96 output** captured against the actual file (header included) in a scratch copy, so the inline line numbers (35/29/23/30/37/36/38) match exactly what the learner will see. Decision note: chose E0596 over E0384 for the "forgot `mut`" case because the natural trigger in this code is `&mut` of an immutable binding (E0596), not double-assignment (E0384).

**Pedagogy calls aligned to NOTES:** professional/neutral labels ("Lesson goals", "Outcome", "Optional extensions"); new-concept bullets are full self-contained sentences (clarity > brevity); C/Go contrast retained ("a data race is no error in C; Rust refuses to compile it"). 3 quiz questions (last-use semantics, the `.into_iter()` move, the E0596 fix).

**Housekeeping:** `nav.js` LESSONS array += L5; syllabus L4 → Complete, L5 → Next (linked, description rewritten to the four concrete codes); NOTES course arc updated (L4 ✅, L5 ✅ authored, L6 now NEXT with the doc-tests/`src/lib.rs` commitment re-flagged).

**Next:** Lesson 6 — structs & enums (`Counts` struct replacing the `(usize,usize,usize)` tuple; `Source` enum for file-vs-stdin). First lesson with `src/lib.rs`, so **doc-tests are due here** per the deferred-topics commitment.
