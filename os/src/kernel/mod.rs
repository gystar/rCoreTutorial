pub mod condvar;

use crate::process::*;
use alloc::sync::Arc;
pub use condvar::Condvar;
use spin::Mutex;
