# AGENTS

Concise guidance for AI/code agents contributing to Carson: Rust
code, Prolog Reasoning, Ontologies, Knowledge graphs, Nix and
tests.

If .jj exists, you are in a jj version-controlled workspace; you must
use jj instead of git. Full reminder docs are `.agents/AGENTS-JJ.md`.

Any time you start a new logical piece of work, you should be on an
empty commit like this:

  @  prvolytw user@example.com 2025-09-19 16:56:38 default@ 15921e83
  │  (empty) (no description set)

Make regular commits with clear messages and rationale and inform the
user about the nature of those commits.

When you are finished you should be on a new empty commit (jj commit does this
in the act of specifying the commit message for the current commit, or if you
used `jj desc` you can use `jj new` to start a new commit on top of the current
one). JJ snapshots the entire working copy on every command; `@` is both your
live tree and the current commit (no staging area). Only use `jj new` when you
need a fresh empty `@` on top of a non-empty parent. To fold a small fix into an
existing change, create a new commit with `jj new`, describe it with `jj desc
-m ...`, and `jj squash -m ...` it into the target once verified. When using
restore/squash/rebase into an existing commit, re-set the intended description
with `jj desc -m ...` and spot-check with `jj log -r <rev> -T description` so
subjects/Change-Ids stay intact; prefer squashing only into dedicated fixup
commits instead of large aggregate changes.

For prompt/docs tweaks that must land on main, base the change on `main@origin`
(`jj new main@origin` → edit → `jj commit ...`), then rebase/return to your
work state (`jj edit` or `jj new`) after exporting the prompt commit.

This environment is editor-less: always pass a message flag when updating
descriptions or committing.

- Use `jj commit -m "…"` (or `--message`) to avoid launching an editor.
- Use `jj desc -m "…"` or `jj desc --stdin` (not `jj commit --stdin`).
- For multi-line messages in the shell, wrap them in `$'…'` so newlines stay
  within the same argument and do not split into separate commands.

Always use `--no-pager` with jj commands to avoid blocking on pager output.

Commit messages should begin upper case `Verb ...` or `pkg: Verb ...`
never `verb ...` or `pkg: verb ...`.

If the user asks about recent commits, they probably mean downstream commits
`jj log -r 'trunk()::'`, assuming they are rebased correctly against trunk.

## Quick Links
- README: `README.md`
- Design docs: `docs/`
- Tests: `tests/` (surfaced via `.#testing.evaluation` and `.#testing.builds`), `overlays/testing.nix` (extends `self.testing`).

## Priorities
- Documentation: keep expressions/workloads docs current; add small runnable examples.
- Tests first: prefer Nix eval/build probes; unit tests for Go.
- Stability: avoid bumping `flake.lock` or `overlays/llvm-latest.json` unless required.
- Changes: small, surgical diffs with clear rationale and adjacent tests.

## ADR Authoring (Agents)
- When asked to write or update an ADR, follow `docs/ADRs/PROCESS.md` and start
  from `docs/ADRs/TEMPLATE.md`.
- Treat invariants as part of the interface: if an ADR changes runtime
  correctness/cost boundaries, update `docs/DATA_INVARIANTS.md` (or a more
  specific invariant document) alongside the implementation.
- Always propose at least one validation hook that keeps the ADR true over time
  (unit tests, Nix eval/build probes, or P models under
  `theory-proving/models/hecaton/` when interleavings/retries are central).

## JJ Change Hygiene (Agents)
- It is OK (encouraged) to make frequent local commits for checkpoints and
  interpretability, but do not send those directly upstream.
- Before exporting/pushing, groom `trunk()::@` into task-sized commits (roughly
  one jj change per task), plus optional stand-alone enablement commits.
- Use `docs/tasks/TEMPLATE.md` “Finalization (JJ-ready history)” as the
  gate, and use `jj squash` with change IDs to consolidate fixups.

## Documentation (Diátaxis)
- Pick a doc type up front and avoid mixing: Tutorials (learning, first success), How-to guides (goal-oriented recipes), Reference (complete, accurate facts), Explanation (concepts/rationale).
- `README.md` should stay split by intent: quick tutorial-style first build, a how-to list for recurring tasks (cache setup, debug builds, try new LLVM), a compact reference/glossary, and a short explanation of Carson goals/philosophy.
- When adding docs elsewhere, label intent in the intro and keep pages single-purpose; cross-link instead of embedding long background in how-tos.
- Prefer short how-to pages for common workflows (perf-results, workload additions, cache usage, LLVM refresh) and keep tables for variants/params in reference docs (`EXPRESSIONS.md`, `WORKLOADS.md`).
- If you spot mixing or gaps, surface a concise recommendation in your summary to prompt humans to realign the docs; only restructure when in scope for the task.

## Repo Layout
- `overlays/`: overlays, variants, consumers, workload defs (e.g., `packages.nix`, `stdenv-*.nix`).
- `tests/`: eval/build probes (e.g., `evaluation.nix`); aggregated as `self.testing.evaluation` / `self.testing.builds`.
- `tools/`: Go modules and commands under `cmd/`.
- `aws/`, `jenkins/`: infra configuration and pipelines.
- Root: `flake.nix`, `flake.lock`, `default.nix`, `README.md`.

## Coding Standards
- Shell: POSIX sh, `set -euo pipefail`, deterministic output; avoid `bash -l`.
- Docs: concise, actionable; link to sources when behavior isn’t obvious.

## Nix Gotchas
- Attribute existence checks allow computed names: use string attrpaths (`a ? "${name}"`) or `builtins.hasAttr name a`; `${name}` interpolation in `?` is valid.
- `passthru` visibility: attributes placed in `passthru` are also visible directly on the derivation (`drv.collect` and `drv.passthru.collect` both work). In user-facing docs/commands, prefer the direct form; in Nix internals/tests, using `drv.passthru.*` is acceptable for explicitness or to avoid collisions with output names.

## Nix Workflow
- In Codex CLI sessions, prefer MCP Nix tools (e.g. `mcp__nix__nix_eval`,
  `mcp__nix__nix_build`, `mcp__nix__nix_log`) over running `nix` directly unless
  the user explicitly asks for raw `nix` CLI commands.
- Precedence reminder: `or` (defaulting) binds before comparisons and before `||`.
  `x or null != null` parses as `(x or null) != null` (boolean), while
  `(x or null)` by itself may be an attrset and will fail if fed into `||`.
- Evaluate first:
  ```
  nix eval .#testing.evaluation
  ```
- Dry-run builds and probes (pick an attr from `.#testing.builds`):
  ```
  nix build .#testing.builds.<attr> --dry-run
  nix build .#<attr> --dry-run
  ```
- Inspect/compare:
  ```
  nix log <expr-or-path>
  nix-store -qR ./result
  nix-diff <drvA> <drvB>
  ```
- Partial aggregate collection when builds are incomplete:
  ```
  nix build .#<pkg>.toolchains.both.variants.perf-results --keep-going
  nix run .#<pkg>.toolchains.both.variants.perf-results.perf-aggregate.collect
  ```
  Exit codes: 0=complete, 1=partial results written, 2=no data available.
  Output: `perf-partial.csv` with coverage and drvPath hints for missing branches.
- Targeted builds/dev shells:
  ```
  nix build .#<attr>
  nix build -f . <attr>        # non-flake eval path
  nix develop .#<attr>         # enter dev shell for attr
  ```
- Built-in tests: use `.#testing.builds.<attr>` from the aggregated test set.
- Prefer CI/cache over local expensive builds.
- After adding new files that Nix needs (e.g. scripts referenced via `./…`), run `jj new` so the working copy snapshot includes them before evaluating or building with Nix.

### Capturing configure trees (for toolchain/debug)
- Use the `capture-config-tree` variant (see `overlays/stdenv-variants.nix`) to grab `config.log` and the post-configure tree without root access to daemon builds:
  ```
  nix build .#packages.aarch64-linux.gccNGPackages_git.libstdcxx.capture-config-tree
  tar -tzf result-configTree/configure-tree.tar.gz | head
  ```
  Outputs: `configure-exit-status` and `configure-tree.tar.gz` (contains `build/config.log` etc.).

## Where To Add Things
- New Tests: `tests/` also make an entry to the `.justfile`
- Workloads: `overlays/packages.nix` via `meta.workload` / `meta.stimulus` (see WORKLOADS.md).
- Variants/consumers/params: `overlays/stdenv-variants.nix`, `overlays/stdenv-consumers.nix`, `overlays/stdenv-params.nix`.
- Machine/compiler sets: `overlays/machine-package-sets.nix`, `overlays/compiler-package-sets.nix`.
- Nix tests: `tests/evaluation.nix` (eval assertions), `tests/default.nix` (aggregates builds/eval), `overlays/testing.nix` (merges helpers into `self.testing`).
- Go tools/services: `tools/` (see `tools/AGENTS.md`).

## Testing
- Nix evaluation: `nix eval .#testing.evaluation`.
- Build probes: use `.#testing.builds.<attr>` (optionally narrowed) and `--dry-run` first.
- Build closure inventory: `nix-store -qR "$(nix eval --raw .#testing.buildsDrvPaths.drvPath)" | sort -u` walks the `.drv` closures of every build test in one shot (from `tests/default.nix`) without building them. Use it to spot unexpected dependencies or repeated derivations before/after changes.
- Local failure analysis: `nix log`, reproducible inputs, and dev shells.
- Add tests alongside changes; surface them via `tests/default.nix` (and `overlays/testing.nix` if new helpers are required).
- Go: `(cd tools && go test ./...)`.

## CI, Caching, Review
- Jenkins: `jenkins/precommit.*`; CI builds only when derivations change.
- Cache: prefer Carson binary cache; set `substituters` and `trusted-public-keys` as in `README.md`.
- Gerrit: push to `refs/for/main` (or `git review`); use imperative subjects (<=72 chars) and brief rationale/impact; reference touched attrs; include example commands (e.g., `nix build .#…`).

### Autonomous Commit Review Flow
- To inventory candidates with pending reviews, use `jj log --color=never --no-graph -T 'commit_id ++ " " ++ change_id ++ "\n"' -r 'main@origin::submit-cursor'` and inspect change IDs that still have exported review JSON.
- Use `python3 ~/review_comments_flatten.py --rev review-cursor --reviews-dir ~/tmp/reviews` (or `--commit <sha>`) to collect outstanding ChatGPT review comments for the target commit.
- Immediately jot down a one-line summary of how you will act on the surfaced comments (e.g. `update commit message`, `already handled in 8732e166`, `needs user input`) before touching the tree so intent stays explicit.
- Inspect the original change with `jj show review-cursor` and limit history to affected files via `jj log -r 'review-cursor::' <paths>` to understand later edits.
- Triage each comment: verify whether subsequent commits already addressed it; otherwise update code/tests and document fixes with new commits.
- When multiple `review-cursor+` candidates exist, follow the longest descendant chain before repeating the review loop.
- After resolving feedback, rerun the flatten script to confirm the comment list is empty, note the latest reviewed commit in your summary, and leave the working copy on a fresh empty commit; do not move `last-review-cursor` (user-managed).
- Run `tools/scripts/review_comments_flatten.py --rev review-cursor`; it already walks `jj evolog` to locate ancestor review archives when the current tip lacks one.

### Fix Application Workflow
- Execute the steps in **Autonomous Commit Review Flow** for each `review-cursor` tip; re-check `jj log -r submit-cursor::` so you understand which comments are already resolved downstream and avoid duplicating fixes.
- Before editing, outline the planned changes and create a review-fix commit with `jj new tip review-cursor`, making a multi-parent commit that references both the tip and the commit being addressed.
- Implement the fix, run validations, then update the integration bookmark with `jj bookmark set tip -r @` once the working copy commit contains the resolved changes.
- If further adjustments are needed, repeat the loop on a fresh empty commit derived from the updated tip.

### Avoiding Review-Loop Pitfalls
- Advance the cursor with `jj bookmark set review-cursor -r 'review-cursor+'`; `jj branch …` is unavailable in this workspace build.
- When referencing specific commits in revsets, use `commit_id(<sha-prefix>)` (for commit IDs) or `change_id(<id-prefix>)` (for change IDs); bare hex is interpreted as a change ID, and `commit:<sha>` is rejected.
- jj-specific docs live in `.agents/AGENTS-JJ.md`; refer to that path directly to avoid case-sensitive lookup errors.

### Review Fix Commit Messages
- When committing fixes for review comments, start the message with the normal imperative summary and immediately mention which comment is addressed (e.g. `web: Guard download cancel (review: cancel cleanup)`).
- Keep the review synopsis short (<=40 chars) so downstream tooling can scan the history.
- Follow the subject with a short body that justifies how the change resolves the feedback; prefer one or two sentences that cite the behaviour change or test coverage you verified.
- If a commit simultaneously resolves multiple comments, list the primary one in the subject and explain the rest in the body.

## Do / Don't
- Do: add tests; document new expressions/params; keep diffs narrow.
- Do: use `self.<pkg>` for uninstrumented binaries in init paths; keep init deterministic where possible.
- Do: implement logic generically (e.g. in `extensible-stdenv.nix`) rather than repeating it for each package or selector.
- Don't: bump global inputs casually; commit generated outputs; reformat unrelated files.

## Parameter Semantics
Parameters like `boltLite`, `lto`, `pgo`, etc. are recorded in `passthru.params` but only
**influence derivations that actually use them**. For example:
- `boltLite.on` vs `boltLite.off` affects BOLT-related derivations (`bolt-opt-instr`,
  `bolt-profdata-instr`, etc.) but NOT the base `hello-cpp` derivation.
- The base derivation `hello-cpp` will have the same `drvPath` in both `boltLite.on` and
  `boltLite.off` branches because BOLT parameters don't affect how `hello-cpp` is built.
- Only when you access a BOLT consumer (like `.bolt-opt-instr`) will the parameter
  actually change the derivation.

This means when verifying fanout correctness:
- Check derivations that ARE affected by the parameter (e.g., `hello-cpp.bolt-opt-instr`)
- Don't expect different `drvPath`s for derivations that don't use the parameter

## Debugging Dimension Issues

When aggregate consumers (like `perf-aggregate`) produce fewer branches than expected,
or when fanouts seem to lose dimensions, use this diagnostic pattern:

```nix
nix eval --impure --expr '
let 
  pkgs = import ./. {};
  getDims = pkg: builtins.attrNames (pkg.__fanoutContext or pkg.passthru.__fanoutContext or {});
in {
  step1 = getDims pkgs.hello-cpp.toolchains.both;
  step2 = getDims pkgs.hello-cpp.toolchains.both.variants;
  step3 = getDims pkgs.hello-cpp.toolchains.both.variants.<fanout>.both;
  # ... trace each step to find where dimensions are lost
}
'
```

**Key invariant**: Each step should **add** to the dimension set, never lose dimensions.
If `step3` has fewer dimensions than `step2`, context is being lost rather than merged.

See `docs/failure-modes/0006-fanout-context-dimension-loss.md` for detailed diagnostics
and root cause patterns.

## Task Execution Workflow

Tasks are organized under `docs/tasks/` in directories named after their parent ADR
(e.g., `docs/tasks/ADR0021/`). Each task directory contains a `README.md` index and
individual task files (`T1.md`, `T2.md`, etc.).

### Creating a New ADR (Numbering)

When creating a new ADR under `docs/ADRs/NNNN-<slug>.md`, pick the **lowest ADR
number that has never been used in any revision** (avoid reusing numbers from
deleted/abandoned ADR files).

One simple way is to scan the full `jj` history for any ADR paths and then pick
the next unused number:

```bash
jj --no-pager log -r '::' --no-graph --name-only docs/ADRs \
  | rg -o 'docs/ADRs/[0-9]{4}' | rg -o '[0-9]{4}' | sort -u
```

### Finding the Next Task

1. **Identify the active ADR task directory**:
   ```bash
   ls docs/tasks/
   ```

2. **Read the README.md to understand task dependencies and status**:
   ```bash
   cat docs/tasks/<ADR>/README.md
   ```

3. **Find the first task with status "Not Started" whose dependencies are "Done"**:
   - Check the Task Index table in README.md
   - Verify all listed dependencies have `Status: Done` in their task files

4. **Read the task file thoroughly before starting**:
   ```bash
   cat docs/tasks/<ADR>/<task>.md
   ```

### Task File Structure

Each task file contains these sections:

| Section | Purpose |
|---------|---------|
| **Status** | Current state: `Not Started`, `In Progress`, `Done`, `Blocked` |
| **Owner** | Who is working on it (can be `TBD` or agent session ID) |
| **Related ADR** | Parent ADR reference |
| **Depends On** | Other tasks that must be Done first |
| **Objective** | What the task accomplishes |
| **Deliverables** | Concrete outputs to produce |
| **Implementation Notes** | Code snippets, patterns, guidance |
| **Key Considerations** | Gotchas, edge cases, design constraints |
| **Acceptance Criteria** | Checkboxes that must all be checked for Done |
| **Validation** | Commands to verify the implementation works |
| **Open Questions** | Unresolved design decisions (escalate to user if blocking) |

### Execution Flow

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. CLAIM TASK                                                   │
│    - Update Status: "Not Started" → "In Progress"               │
│    - Update Owner: "TBD" → describe your session                │
│    - Commit: "docs: Claim task T<n> for ADR <m>"                │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ 2. SPIKE (Early Checkpoint)                                     │
│    - Implement the SIMPLEST possible version first              │
│    - Goal: validate core assumptions, not full implementation   │
│    - Test one basic case end-to-end                             │
│    - If spike fails: STOP, document blockers, consult user      │
│    - If spike succeeds: proceed to full implementation          │
│    - Commit: "<scope>: Spike for T<n> - <what was validated>"   │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ 3. IMPLEMENT                                                    │
│    - Follow Implementation Notes                                │
│    - Make incremental commits with clear messages               │
│    - Reference task ID in commits: "overlays: Add X (ADR0021/T1)"│
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ 4. VALIDATE                                                     │
│    - Run each Validation command                                │
│    - Check each Acceptance Criteria checkbox                    │
│    - Run `nix eval .#testing.evaluation` if Nix changes         │
│    - Run `go test ./...` if Go changes                          │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ 5. COMPLETE TASK                                                │
│    - Check all Acceptance Criteria boxes: [ ] → [x]             │
│    - Update Status: "In Progress" → "Done"                      │
│    - Add Outcome section summarizing what was delivered         │
│    - Commit: "docs: Complete task T<n> for ADR <m>"             │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ 6. UPDATE README                                                │
│    - Update task status in README.md Task Index table           │
│    - Commit: "docs: Update ADR <m> task index"                  │
│    - Run `jj new` to start fresh for next task                  │
└─────────────────────────────────────────────────────────────────┘
```

### The Spike Checkpoint

The spike is a critical early validation step. Before investing in full
implementation, prove the approach works with the simplest possible test:

**What makes a good spike:**
- Tests ONE core assumption (e.g., "can I access `__cartesianProduct` from a consumer?")
- Minimal code - often just a few lines in a test file or REPL
- Fast feedback - should complete in under 5 minutes
- Documents findings - what worked, what didn't, any surprises

**Spike success criteria:**
- The core mechanism works as expected
- No fundamental blockers discovered
- Complexity estimate is reasonable

**Spike failure actions:**
1. Document what failed and why in the task file
2. Update Status to `Blocked` if fundamental issue
3. Consult user before proceeding with workarounds
4. Do NOT proceed to full implementation

**Example spike for T1 (perf-aggregate consumer):**
```nix
# In tests/evaluation.nix, add minimal test:
spike-perf-aggregate = let
  drv = pkgs.hello-cpp.toolchains.both.variants.perf-results;
  hasCombined = drv ? __combinedBranches || drv ? __cartesianProduct;
  branches = drv.__combinedBranches or drv.__cartesianProduct or {};
in if hasCombined && builtins.length (builtins.attrNames branches) > 0
   then "spike-ok: ${toString (builtins.length (builtins.attrNames branches))} branches"
   else throw "spike-failed: no combined branches found";
```

Then run: `nix eval .#testing.evaluation.spike-perf-aggregate`

### Status Transitions

- `Not Started` → `In Progress`: When you begin work (claim the task)
- `In Progress` → `Done`: When all acceptance criteria pass
- `In Progress` → `Blocked`: When you hit an unresolvable issue (add reason)
- `Blocked` → `In Progress`: When blocking issue is resolved
- Any → `Not Started`: If abandoning work (explain why, clear Owner)

### Task Completion Checklist

Before marking a task Done:

- [ ] All Acceptance Criteria checkboxes are `[x]`
- [ ] All Validation commands pass
- [ ] Tests added and passing (`nix eval .#testing.evaluation` or `go test`)
- [ ] Implementation commits have clear messages referencing the task
- [ ] No Open Questions remain (or explicitly deferred with user agreement)
- [ ] Outcome section added describing what was delivered

### Adding an Outcome Section

When completing a task, add an `## Outcome` section at the end:

```markdown
## Outcome

- Added `perf-aggregate` consumer to `overlays/stdenv-consumers.nix`
- Created `collect_aggregate.py` in `overlays/packages/perf-summary/`
- Evaluation test added: `testing.evaluation.perf-aggregate-basic`
- Commits: abc1234, def5678
```

### Handling Blockers

If a task cannot be completed:

1. Update Status to `Blocked`
2. Add a `## Blocked` section explaining the issue:
   ```markdown
   ## Blocked
   
   - **Reason**: T3 depends on machine-package-sets infrastructure that
     doesn't expose individual machine variants at package level
   - **Needs**: User input on whether to extend machine-package-sets.nix
     or create new infrastructure
   - **Blocked since**: 2025-12-19
   ```
3. Inform the user and await guidance
4. Do NOT proceed to dependent tasks

### Commit Message Format for Tasks

```
<scope>: <imperative summary> (ADR<n>/T<m>)

<body explaining what and why>

Refs: docs/tasks/ADR<n>/T<m>.md
```

Example:
```
overlays: Add perf-aggregate consumer (ADR0021/T1)

Introduces a new consumer that walks __cartesianProduct or
__combinedBranches to collect performance data into a unified
manifest for downstream aggregation.

Refs: docs/tasks/ADR0021/T1.md
```

## Go Tools Note
- For coordinator/worker specifics and testing patterns under `tools/`, see `tools/AGENTS.md`.
