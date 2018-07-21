//! Contract ABI helpers.

use error::Error;
use ethabi::{Bytes, RawLog, TopicFilter};
use linker::Linker;
use std::path::PathBuf;

/// Context for all loaded contracts.
pub struct ContractContext {
    /// List of sources, as indexed by a source map.
    pub source_list: Option<Vec<PathBuf>>,
}

/// Contract functions generated by parables_build.
pub trait ContractFunction {
    /// Output types of the function.
    type Output;

    /// Encodes the input for the function.
    fn encoded(&self, linker: &Linker) -> Result<Bytes, Error>;

    /// Decodes the given bytes output for the contract function.
    fn output(&self, output_bytes: Bytes) -> Result<Self::Output, Error>;
}

/// Helpers for building log filters.
pub trait LogFilter {
    fn wildcard_filter(&self) -> TopicFilter;
}

/// Log parsing implementation.
pub trait ParseLog {
    /// Type of the parsed log.
    type Log;

    /// Function to parse log.
    fn parse_log(&self, log: RawLog) -> Result<Self::Log, Error>;
}

pub trait Constructor {
    /// Name of the constructor item, used for linking.
    const ITEM: &'static str;

    /// Access the code to deploy for this constructor.
    const BIN: &'static str;

    /// Access the source map for the type this constructor is associated with.
    const SOURCE_MAP: Option<&'static str>;

    /// Access the runtime code being deployed.
    const RUNTIME_BIN: Option<&'static str>;

    /// Access the runtime source map for the type this constructor is associated with.
    const RUNTIME_SOURCE_MAP: Option<&'static str>;
}
