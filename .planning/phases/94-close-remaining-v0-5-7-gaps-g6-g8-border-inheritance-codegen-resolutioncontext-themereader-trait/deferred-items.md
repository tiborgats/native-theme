# Phase 94 — Deferred Items

## From Plan 94-02 (G7 ResolutionContext)

### Clippy dead-code warnings on 94-01's inheritance registries

**Observed during 94-02 execution (2026-04-20):**

```
error: fields `widget_name` and `kind` are never read
  --> native-theme/src/resolve/mod.rs:63:9
   |
61 | pub(crate) struct BorderInheritanceInfo { ... }

error: fields `widget_name` and `font_field` are never read
  --> native-theme/src/resolve/mod.rs:86:9
   |
84 | pub(crate) struct FontInheritanceInfo { ... }
```

**Cause:** Plan 94-01 (running concurrently in Wave 1) added these two
inventory structs (`BorderInheritanceInfo`, `FontInheritanceInfo`) as part
of its G6 border/font inheritance codegen migration. The structs are
declared in 94-01's RED phase but not yet consumed by any reader (the
consumer is an inverted drift test in `inheritance.rs::tests` that 94-01
will wire up in its GREEN phase).

**Scope:** Out of Plan 94-02's one-file-of-focus. `resolve/context.rs`
and the `ResolutionContext` type are unrelated to these registries;
94-02's changes compile cleanly.

**Disposition:** DEFERRED to Plan 94-01's GREEN phase. When 94-01
completes and consumes the registries via
`inventory::iter::<BorderInheritanceInfo>()` + matching reads for
`FontInheritanceInfo`, the dead_code warning self-resolves. No action
from 94-02 required.

**Verification path:** After 94-01 GREEN lands, re-run
`./pre-release-check.sh` — the clippy failure at the "Running clippy
(native-theme)" step should resolve and the release gate go green.
