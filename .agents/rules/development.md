# Abyss Project Development Rules

## 1. Code Cleanliness: Strictly Zero Comments by Default
- **Default Invariant:** Never write comments, docstrings, or inline explanations inside code unless explicitly instructed by the user.
- **When Explicitly Requested:** Keep comments extremely short, minimal, and exclusively in English.
- Code must be self-explanatory through clean naming, idiomatic constructs, and strong types.

## 2. Git Commit Invariants
- **High Frequency:** Commit changes with high frequency at logical checkpoints.
- **Commit Style:** Always use concise, single-line English commit messages (e.g. `feat(parser): add slice syntax`, `refactor(std): upgrade File to accept Str`).

## 3. Parallel `std` & `core` Evolution
- Never write dirty workarounds or ad-hoc low-level logic in compiler or tooling code when a proper abstraction belongs in `std` or `core`.
- Continuously enhance and enrich `std` and `core` in parallel to keep the codebase clean, high-level, and reusable.

## 4. Proactive Skill & Knowledge Updates
- Whenever introducing a new language feature, standard library API, or architectural invariant, autonomously update the `abyss-lang` skill so that future turns benefit immediately.

## 5. Comprehensive & Negative Testing Invariant
- **Immediate Test Coverage:** Every new feature, syntax addition, or refactoring must immediately update existing tests or introduce dedicated comprehensive test files.
- **Edge-Case & Negative Testing:** Test suites must rigorously probe edge cases, boundary conditions, and invalid inputs. For syntax and type restrictions, write negative test cases in `tests/errors/<name>.a` accompanied by expected `.err` output.
- **Full-Codebase Unification:** When introducing a new feature or compiler refactor, update the entire compiler source code and all existing tests so that the entire codebase is unified, modernized, and passes all tests (201+ tests green).
