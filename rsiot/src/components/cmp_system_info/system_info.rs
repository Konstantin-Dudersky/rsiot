use std::collections::HashMap;

/// Системная информация
#[derive(Debug, Default)]
pub struct SystemInfo {
    /// Returns the system hostname based off DNS.
    pub host_name: String,

    /// Returns the system long os version (e.g “MacOS 11.2 BigSur”).
    pub os_version: String,

    /// Информация о сетевых подключениях
    pub networks: HashMap<String, SystemInfoNetwork>,
}

/// Информация о сетевом подключении
#[derive(Debug, Default)]
pub struct SystemInfoNetwork {
    /// Name of network interface
    pub name: String,

    /// MAC address for network interface
    pub mac_address: String,
}
