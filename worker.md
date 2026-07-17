# Worker Registry

## Current Goal

- Goal: Completed cross-platform AxShell detached-window memory reductions without changing GPUI renderer ownership.
- Coordinator: main thread
- Last Updated: 2026-07-17 10:25 CST

## Reuse Rules

- Reuse only when task context matches (`goal_id` or `task_slice`) and there is at least 1 strong + 1 weak signal, unless exact `owned_paths` + `deliverable` match allows direct reuse.
- Prefer reachable live agents over file-only records.
- Do not use wide `owned_paths` as the main reuse signal for research/chat/no-file tasks.
- Exclude `reuse_hint=do-not-reuse` rows from normal scoring unless the current request explicitly asks to resume them.
- Mark uncertain liveness as `suspected-stale` before replacement; promote to `stale` only after explicit checks.

## Agents

| name | agent_id | status | goal_id | task_slice | responsibility | owned_paths | workstream | execution_lane | worker_class | reuse_hint | deliverable | deliverable_kind | task_mode | dependency_boundary | session_id | reachability | last_heartbeat_at | last_checked_at | ttl_hint | progress_marker | overlap_keywords | notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |


## History

| name | agent_id | final_status | goal_id | task_slice | summary | closed_at | notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| coordinator | main-thread | completed | 20260717-memory | diagnosis-integration | Delivered a runtime- and source-backed explanation; no application code was changed. | 2026-07-17 09:00 CST | Coordination record retained for future deduplication. |
| runtime-profiler | /root/runtime_profiler | completed | 20260717-memory | runtime-memory-sampling | Sample footprint is 245.7 MiB, led by IOSurface and malloc; a separate installed AxShell process is also running. | 2026-07-17 09:00 CST | One-shot validation; no files changed. |
| lifecycle-reviewer | /root/lifecycle_reviewer | completed | 20260717-memory | source-lifecycle-audit | Identified unconditional full-process sysinfo sampling, embedded font baseline, and conditional cache/list retention paths. | 2026-07-17 09:00 CST | One-shot source review; no files changed. |
| sampler-worker | /root/sampler_worker | completed | 20260717-window-memory | selective-system-sampling | Replaced full sysinfo refresh with selective CPU/memory sampling and made the sampler lazy. | 2026-07-17 10:17 CST | Focused test and cargo check passed; one-shot source owner. |
| detached-init-worker | /root/detached_init_worker | completed | 20260717-window-memory | detached-init-slimming | Added initialization mode, skipped detached icon/local-directory prewarming, and released stale main workspace globals. | 2026-07-17 10:17 CST | Full test suite and cargo check passed; one-shot source owner. |
| renderer-reviewer | /root/renderer_reviewer | completed | 20260717-window-memory | renderer-boundary-validation | Confirmed per-window native presentation targets on all supported platforms and supplied manual verification guidance. | 2026-07-17 10:17 CST | No local files changed; one-shot reviewer. |
| coordinator | main-thread | completed | 20260717-window-memory | integration-and-tracking | Integrated sampler, detached initialization, lifecycle cleanup, platform-boundary review, and validation records. | 2026-07-17 10:25 CST | Full Rust suite, diff check, and tracking validator passed; target-platform GUI profiling remains manual. |
