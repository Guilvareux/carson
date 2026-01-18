# JJ for Git-Native Agents

## Mental Model
- Treat `jj` as a history editor layered on top of Git: it keeps its own operation log and change graph, then syncs with Git via `jj git import`/`export` (colocated repo state) and `jj git fetch`/`push` (remotes).
- Every change has a stable `change id`; each rewrite creates a new `commit id` for the same change. When you amend you are really creating a replacement commit that inherits the old change id so descendants auto-rebase.
- The "working copy" (`@`) is itself a commit that `jj` snapshots before and after every command; files are tracked automatically, so you normally just edit the tree and let `jj` notice.

## Addressing Revisions with Revsets
- Most commands take revsets: expressions that evaluate to one or more commits. They are Mercurial-style (`docs/revsets.md`) and can combine operators like `::`, `..`, `&`, `|`, etc.
- Single-letter shortcuts: `@` is the working-copy commit, `@-` its first parent, `@--` the grandparent, and so on. `@+` refers to the checked-out descendant if divergence exists.
- Short prefixes refer to change IDs by default; use `commit_id(<prefix>)` to select by commit ID when needed. Bookmarks (Git branches) are revsets too: `main`, `feature@origin`, `all:main` (allow multiple heads).
- Commands that expect exactly one revision will fail if the revset resolves to zero or multiple commits. Use `all:` when you intentionally target many (e.g. `jj abandon 'all:trunk() & ::@'`).

## High-Frequency Commands
- `jj status` (`jj st`): Like `git status`, but shows the current working-copy commit, its parents, conflicts, and a diff summary. Runs automatically before/after mutations, so it reflects the precise tree `jj` is tracking.
- `jj new <base>`: Create an empty child commit on top of the given parent(s) (default `@`) and check it out. Use it the way you might `git switch -c` followed by an empty commit: start new work, fork from an older revision, or stage conflict resolutions separate from the conflicted commit.
- `jj desc` (alias `jj describe`): Update the change description/metadata. Defaults to editing `@`, but accepts any revset. `-m` lets you script message updates without launching an editor.
- `jj edit <rev>`: Directly check out an existing change to rewrite it in place. Handy for quick edits, but remember it mutates that change immediately—best practice is still `jj new REV` then `jj squash` so you can review the delta first.
- `jj squash`: Move content between commits and optionally drop emptied sources. Without flags it squashes `@` into its parent; combine with `--from/--to` for explicit rewrites (see below).
- `jj abandon <rev>`: Drop commits while rebasing descendants onto the abandoned commit’s parents. Unlike `git reset --hard`, descendants are preserved; you can keep bookmarks with `--retain-bookmarks` or leave child content intact with `--restore-descendants`.

## Using `jj squash --from … --to …`
- General form: `jj squash --from <source> --to <destination>` moves the diff represented by the source revset onto the destination revision. If either side is omitted it defaults to `@`.
- `--to` is an alias for `--into`; whichever you use, the destination revision is rewritten with the source’s content applied on top. `jj` then abandons the (now empty) source change unless `--keep-emptied` is set.
- Typical patterns:
  - Amend an earlier change: `jj new <target>; …; jj squash --from @ --to <target>` rewrites `<target>` while keeping a reviewable staging commit.
  - Extract work upward: `jj squash --from feature1 --to feature0` merges one feature commit into another. Multiple `--from` values are allowed when you want to fold a stack into a single destination.
  - Move working-copy edits elsewhere: `jj squash --from @ --to @--` pushes your current edits into the grandparent so you can repurpose the current commit.
- When both source and destination have messages, `jj` prompts to reconcile them unless you pass `--use-destination-message` or `-m`.

## Conflict Markers and Resolution Workflow
- Conflicted commits are first-class: rebases and merges that hit conflicts still complete, and the conflicted state lives in the commit until you resolve it.
- When you materialize such a commit (`jj new <conflicted>` to stage a fix, or `jj edit <conflicted>`), files gain structured conflict markers:
  ```
  <<<<<<< Conflict 1 of 1
  %%%%%%% Changes from base to side #1
   apple
  -grape
  +grapefruit
   orange
  +++++++ Contents of side #2
  APPLE
  GRAPE
  ORANGE
  >>>>>>> Conflict 1 of 1 ends
  ```
  `%%%%%%%` sections are diffs to apply to the snapshot shown after `+++++++`. Multi-way conflicts add more diff sections; markers elongate (`<<<<<<<…`) when needed to avoid ambiguity.
- Resolve by editing the file to the final desired contents (apply the diffs to the snapshot, remove the markers). `jj` re-parses the file on the next snapshot, so there is no `git add` step—just ensure markers are gone or rewritten as partial resolutions.
- Recommended flow: `jj new <conflicted>` → edit files (or invoke `jj resolve --tool <name>` for a 3-way merge) → inspect with `jj diff` or `jj status` → `jj squash --from @ --to <conflicted>` to drop the staging commit and embed the resolution → optionally `jj desc <conflicted>` to update the message.

## Squash Safety Rules

1. **Always use `--no-pager`** with jj commands to avoid blocking on pager output.

2. **Always pass `-m "message"`** to `jj squash` to avoid launching an editor (terminal is editor-less).

3. **Use change IDs, not commit IDs** in `jj squash --from`. Change IDs are short alphanumeric (like `trnxvulk`); commit IDs are hex SHA (like `c64c77c2558c`). Using commit IDs can resurface old parallel commits and create divergence.

After squashes or rebases, check for divergence:
```bash
jj log -r 'trunk()::' -T 'change_id.short() ++ "\n"' --no-graph | sort | uniq -d
```

If any change IDs appear, inspect with `jj op show @` or trace with `jj --at-op <op-id>`.

## Practical Tips
- Use `jj log -r 'descendants(@)..::trunk()'` (or your favorite revset) to inspect the stack you are rewriting; revsets let you script complex selections instead of memorizing hashes.
- Because descendants auto-rebase, aggressive rewriting is safe. If you need to undo a rewrite entirely, `jj undo` reverts the last operation in the operation log.
- Remember to sync with Git via `jj git push`/`pull` once the change graph matches what you want exposed to collaborators.
