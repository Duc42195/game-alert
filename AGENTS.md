# AGENTS.md — game-alert

The single rulebook for every person and every AI tool in this repo. Other AI config files only point here.

## Profile
- What: Phần mềm chạy nền (Rust, build Windows) phát hiện tiến trình game đã đăng ký (Roblox), chặn tiến trình và gửi cảnh báo qua Discord
- Goal: Chạy ổn định nền trên Windows: phát hiện đúng tiến trình game, chặn (kill + sau này overlay màn hình), gửi cảnh báo qua Discord webhook
- Stack: Rust (sysinfo), build target Windows
- Research project: no
- Task id prefix: `GA` · Git host: GitHub · Default branch: `main` · External tracker: none
- Gate command: ``
- Python: `python3` (only `/plan-check` needs it)

## Where things live (read the relevant one before you act)
| Need | Where |
|---|---|
| What is planned, who owns it, how far, reviewed or not | `plan.csv` (source of truth for tasks) |
| Why the system is the way it is | `.agents/wiki/decisions-log.md` |
| Lessons, gotchas, open questions, working flow | `.agents/wiki/` |
| Who is who | `.agents/roles.md` |

## Working rules
1. Work from `plan.csv`. Pick a task assigned to you (or unassigned) and set its `status` cell to `in-progress`. `bash .agents/tools/plan.sh list` prints the plan with a done-percentage and marks late tasks.
2. Branch `<type>/<task-id>-<slug>`. Commit `type(scope): subject (<task-id>)`, imperative, lowercase.
3. Close a task with `/done <task-id>`. It commits, opens the MR/PR, updates the row in `plan.csv`. It never merges.
4. A change is reviewed by someone other than its author before merge. The reviewer sets the `review` cell to `approved` or `changes`.
5. The gate (``) must pass before `/done` commits.
6. Git and the MR/PR state are the truth. If `plan.csv` disagrees, fix the CSV. Never mark a task done that is not done.
7. No secrets in the repo. Tokens live in environment variables or `.agents/state/` (git-ignored).
   For `game-alert`: secrets (vd `DISCORD_WEBHOOK_URL`) nằm trong `.agents/state/secrets.env`
   (ưu tiên, riêng của project) hoặc `~/secret.env` (fallback, secrets chung của máy) —
   format `KEY=VALUE`, không commit. `build.rs` đọc 2 file này lúc compile và nhúng giá trị
   vào binary qua `env!()` trong `src/settings.rs`; thiếu secret thì `cargo build` báo lỗi
   rõ ràng ngay lúc đó.
8. When you edit `plan.csv`, keep one task per row and the header as it is; separate ids in `depends` with `;`.

## Knowledge capture (hard rule)
In the same session it happens, append (never delete, never rewrite history):
- a decision → `.agents/wiki/decisions-log.md`
- an error you hit and fixed → `.agents/wiki/learnings.md` as symptom · root cause · fix · lesson
- an open question opened, changed or closed → `.agents/wiki/open-questions.md`
- a process gotcha → `.agents/wiki/working-process.md`
To reverse an earlier call, append a superseding entry that points to the old one.

## Decision integrity
- One topic, one current decision. Before recording one, read `decisions-log.md` for the same `Topic:`. If it is already decided, add a new entry with `Supersedes: <date — title>`; never edit or delete the old entry.
- Every decision has a `Topic:` line, and topics must fit together: if a new decision conflicts with another topic, resolve it before recording it.
- If `Research project` is `yes`: the chosen result of an experiment is a decision with a `Result:` line (run-id, config, metrics, artifact path); a better run supersedes it. Every table, number and figure in a report, paper or wiki page carries its run-id and must equal the chosen result's. If it does not, regenerate it from that run.

## Reading the team record
- Who does what: `owner` column, `.agents/roles.md`.
- How far: `status`, `start`, `end`, `mr`, `updated` columns; `bash .agents/tools/plan.sh list [--owner NAME] [--status STATUS]`.
- Is it right: `reviewer` and `review` columns, plus a green gate.
- Who blocks whom: `depends` column.
