// Axel '0vercl0k' Souchet - May 26th 2024
#![doc = include_str!("../README.md")]
mod addr_space;
mod builder;
mod error;
mod guid;
mod misc;
mod modules;
mod pdbcache;
mod pe;
mod stats;
mod symbolizer;

pub use addr_space::AddrSpace;
pub use builder::Builder;
pub use error::{Error, Result};
pub use guid::Guid;
pub use modules::{Module, Modules};
pub use pdbcache::format_symcache_path;
pub use pe::{PdbId, PeId, SymcacheEntry};
pub use stats::Stats;
pub use symbolizer::{download_from_symsrv, Symbolizer};
