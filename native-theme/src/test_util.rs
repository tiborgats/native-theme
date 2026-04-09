/// Mutex to serialize tests that manipulate environment variables.
/// Env vars are process-global state, so tests that call set_var/remove_var
/// must hold this lock to avoid races with parallel test execution.
pub(crate) static ENV_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());
