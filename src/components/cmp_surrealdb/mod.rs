//! Компонент для взаимодействия с базой данных SurrealDB
//!
//! # Настройка баз данных
//!
//! ## RocksDB
//!
//! Для сборки под Windows поставить через Visual Studio Installer:
//!
//! - поставил llvm через choco
//! - Visual Studio Build Tools:
//!   - MSVC v143 - VS 2022 C++
//!   - Windows 11 SDK
//!   - C++ CMake tools for Windows
//!
//! Для включения бинарника необходимо указать соответствующий флаг в `features`:
//!
//! ```toml
//! surrealdb-rocksdb = { version = "0.24.0-surreal.1", features = ["mt_static"] }
//! ```
//!
//! ## TiKV
//!
//! https://tikv.org/docs/7.1/deploy/install/test/#deploy-the-tikv-cluster-on-a-single-machine
//!
//! ``` sh
//! wget https://download.pingcap.org/tidb-latest-linux-amd64.tar.gz
//! wget http://download.pingcap.org/tidb-latest-linux-amd64.sha256
//!
//! sha256sum -c tidb-latest-linux-amd64.sha256
//!
//! # Extract the package.
//! tar -xzf tidb-latest-linux-amd64.tar.gz
//! ```

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use component::{COMPONENT_NAME, Cmp};
pub use config::{Config, ConfigConnection, RequestInputConfig, RequestStartConfig};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;

static DB: std::sync::LazyLock<surrealdb::Surreal<surrealdb::engine::any::Any>> =
    std::sync::LazyLock::new(surrealdb::Surreal::init);
