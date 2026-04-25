/// Bytes per mebibyte
pub const BYTES_PER_MIB: u64 = 1024 * 1024;

/// Bytes per gibibyte (GiB).
pub const BYTES_PER_GIB: u64 = 1024 * 1024 * 1024;

/// Name of the environment variable that overrides the directory used to
/// locate bundled native libraries for smolvm.
///
/// If set, smolvm checks this directory before falling back to paths relative
/// to the current executable. This is primarily used by embedded runtimes.
pub const ENV_SMOLVM_LIB_DIR: &str = "SMOLVM_LIB_DIR";

/// Name of the environment variable that controls libkrun's log level.
///
/// Accepted values are integer levels understood by libkrun
/// (`0 = off`, `1 = error`, `2 = warn`, `3 = info`, `4 = debug`).
pub const ENV_SMOLVM_KRUN_LOG_LEVEL: &str = "SMOLVM_KRUN_LOG_LEVEL";

/// Number of extra vsock ports an external runner has requested.
///
/// When set to a positive integer `N`, the launcher reads
/// [`ENV_SMOLVM_VSOCK_PORT_PREFIX`]`{0..N}` and registers each
/// `<port>:<host_unix_socket_path>` entry with libkrun (listen=false:
/// the guest dials, libkrun forwards to the host socket).
pub const ENV_SMOLVM_VSOCK_PORT_COUNT: &str = "SMOLVM_VSOCK_PORT_COUNT";

/// Prefix for the indexed vsock-port entries (e.g. `SMOLVM_VSOCK_PORT_0`).
///
/// Each entry's value is `<vsock_port>:<host_unix_socket_path>`. Reserved
/// smolvm control ports (`AGENT_CONTROL`, `SSH_AGENT`, `DNS_FILTER`) are
/// rejected to keep external runners from clobbering them.
pub const ENV_SMOLVM_VSOCK_PORT_PREFIX: &str = "SMOLVM_VSOCK_PORT_";
