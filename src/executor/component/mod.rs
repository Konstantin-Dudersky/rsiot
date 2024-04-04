#[cfg(not(feature = "single-thread"))]
mod multi_thread;
#[cfg(not(feature = "single-thread"))]
pub use multi_thread::{Component, IComponent, IComponentProcess};

#[cfg(feature = "single-thread")]
pub use single_thread::{Component, IComponent, IComponentProcess};
#[cfg(feature = "single-thread")]
mod single_thread;
