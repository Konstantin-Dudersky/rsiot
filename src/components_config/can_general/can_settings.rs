/// Конфигурация CAN-протокола
#[derive(Clone, Debug)]
pub struct CanSettings {
    /// Настройка скорости CAN
    pub bitrate: CanSettingsBitrate,

    /// Настройка скорости CAN-FD
    pub dbitrate: CanSettingsDbitrate,

    /// Used specially when running more than one application on the same node (host)
    pub mode_loopback: bool,

    /// Only listen for frames on the bus (no sending)
    pub mode_listen_only: bool,

    /// Make 3 samples instead of 1 during the bit time (on the 2 TQs before the Sample Point)
    pub mode_triple_sampling: bool,

    /// Just send the CAN message one time (skip retransmission in case of error)
    pub mode_one_shot: bool,

    /// Enable/Disable Bit Error reporting
    pub mode_berr_reporting: bool,

    /// CAN-FD протокол
    pub mode_fd: bool,

    /// When enabled, acknowledgement absence is ignored
    pub mode_presume_ack: bool,

    /// Enable non-ISO CAN FD (this is the first specification of CAN FD, called CAN FD 1.0 and it’s
    /// not compatible with ISO CAN FD)
    pub mode_fd_non_iso: bool,

    /// DLC remaining seven values from 9 to 15 used for CAN FD should be set to 8 for standard CAN.
    pub mode_cc_len8_dlc: bool,

    /// Transmitter Delay Compensation Value mode (automatic, manual or disabled - used in CAN FD)
    pub mode_tdcv_mode: bool,

    /// Automatic restart delay time, the time to wait before restart the CAN controller in case of
    /// a bus-off condition (a CAN node becomes Bus-Off when the the counter for transmission errors
    /// becomes greater or equal to 256, so the node is deactivated)
    pub restart_ms: Option<u32>,
}

/// Настройка скорости передачи CAN
#[derive(Clone, Debug)]
pub enum CanSettingsBitrate {
    /// Обычное задание скорости
    Standard {
        /// CAN interface’s bit rate (bps)
        bitrate: u32,

        /// Point in time period where the bus is read to get the current bit level
        sample_point: Option<f32>,
    },

    /// Пользовательская настройка скорости
    Custom {
        /// Time Quantum (1 TQ = 1 Clock tick)
        tq: u8,

        /// Compensates the propagation of physical delays between nodes
        prop_seg: u8,

        /// Used to compensate errors between signal edges and adjust the length of the bit
        phase_seg1: u8,

        /// Used to compensate errors between signal edges and adjust the length of the bit
        phase_seg2: u8,

        /// Synchronization jump width, that’s the maximum time by which the bit sampling period
        /// might be delayed or shortened during each cycle
        sjw: Option<u8>,
    },
}

/// Настройка скорости передачи CAN-FD
#[derive(Clone, Debug)]
pub enum CanSettingsDbitrate {
    /// Используется классический CAN
    None,

    /// Обычное задание скорости
    Standard {
        /// Data bit rate (used in CAN FD, which supports different bit rates for the arbitration phase
        /// and the data/payload phase)
        dbitrate: u32,

        /// Data Time Quantum (used in CAN FD)
        dsample_point: Option<f32>,

        /// Transmitter Delay Compensation Value (used in CAN FD)
        tdcv: Option<u8>,

        /// Transmitter Delay Compensation Offset (used in CAN FD)
        tdco: Option<u8>,

        /// Transmitter Delay Compensation Filter windows value (used in CAN FD)
        tdcf: Option<u8>,
    },

    /// Пользовательская настройка скорости
    Custom {
        /// Data Time Quantum (used in CAN FD)
        dtq: u8,

        /// Compensates the propagation of physical delays between nodes
        dprop_seg: u8,

        /// Used to compensate errors between signal edges and adjust the length of the bit
        dphase_seg1: u8,

        /// Used to compensate errors between signal edges and adjust the length of the bit
        dphase_seg2: u8,

        /// Synchronization jump width, that’s the maximum time by which the bit sampling period
        /// might be delayed or shortened during each cycle
        dsjw: Option<u8>,

        /// Transmitter Delay Compensation Value (used in CAN FD)
        tdcv: Option<u8>,

        /// Transmitter Delay Compensation Offset (used in CAN FD)
        tdco: Option<u8>,

        /// Transmitter Delay Compensation Filter windows value (used in CAN FD)
        tdcf: Option<u8>,
    },
}
