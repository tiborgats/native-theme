/// Mutex to serialize tests that manipulate environment variables.
/// Env vars are process-global state, so tests that call set_var/remove_var
/// must hold this lock to avoid races with parallel test execution.
///
/// NOTE: All ENV_MUTEX usages were removed in Phase 72-01. This module
/// is kept temporarily and will be removed in Phase 72-02.
#[allow(dead_code)]
pub(crate) static ENV_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());
