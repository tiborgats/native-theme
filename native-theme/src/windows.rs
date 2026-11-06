//! The running Windows version, for the generation table
//! ([`crate::pipeline::diagnose_platform_support`]).

/// The running Windows version, from the `windows-version` crate's safe
/// `OsVersion::current()` (`RtlGetVersion`), for the generation table:
/// Windows 11 still reports major version 10, and its build tells the
/// generations apart.
pub(crate) fn os_version() -> crate::pipeline::OsVersion {
    let v = ::windows_version::OsVersion::current();
    crate::pipeline::OsVersion::new(v.major, v.minor, v.build)
}
