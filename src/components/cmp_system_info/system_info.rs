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

    /// Returns CPU’s usage.
    pub cpu_usage: Vec<f32>,

    /// Temperature of components
    pub temperatures: HashMap<String, f32>,

    /// Информация о памяти
    pub memory: SystemInfoMemory,

    /// Информация о дисках
    pub disks: HashMap<String, SystemInfoDisk>,
}

/// Информация о сетевом подключении
#[derive(Debug, Default)]
pub struct SystemInfoNetwork {
    /// Name of network interface
    pub name: String,

    /// MAC address for network interface
    pub mac_address: String,
}

/// Информация о потребленной памяти
#[derive(Debug, Default)]
pub struct SystemInfoMemory {
    /// Доступный размер оперативной памяти, \[MB\]
    pub total_memory_mb: f32,

    /// Используемый размер оперативной памяти, \[MB\]
    pub used_memory_mb: f32,

    /// Общий размер swap, \[MB\]
    pub total_swap_mb: f32,

    /// Используемый размер swap, \[MB\]
    pub used_swap_mb: f32,
}

/// Информация о диске
#[derive(Debug, Default)]
pub struct SystemInfoDisk {
    /// Название
    pub name: String,

    /// Занятое пространство, \[GB\]
    pub used_space_gb: f32,

    /// Общее пространство, \[GB\]
    pub total_space_gb: f32,
}
