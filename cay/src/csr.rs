//! Beagle/Apex CSR offsets used by the bring-up sequence, resolved from
//! libedgetpu's beagle_csr_offsets.h.

// System control unit (32-bit CSRs).
pub const SCU_CTRL_0: u32 = 0x1a30c;
pub const SCU_CTRL_2: u32 = 0x1a314;
pub const SCU_CTRL_3: u32 = 0x1a318;
pub const SCU_CTR_7: u32 = 0x1a33c;
pub const OMC0_00: u32 = 0x1a000;
pub const GCBB_CREDIT0: u32 = 0x1907c;

// Thermal / MBIST / bus-error interrupt enables (32-bit CSRs), programmed by
// BeagleTopLevelInterruptManager during bring-up.
pub const OMC0_D4: u32 = 0x1a0d4;
pub const OMC0_D8: u32 = 0x1a0d8;
pub const RAMBIST_CTRL_1: u32 = 0x1a704;
pub const SLV_ABM_EN: u32 = 0x1a500;
pub const MST_ABM_EN: u32 = 0x1a600;
pub const SLV_ERR_RESP_ISR_MASK: u32 = 0x1a558;
pub const MST_ERR_RESP_ISR_MASK: u32 = 0x1a658;

// Scalar core and tile config (64-bit CSRs).
pub const SCALAR_CORE_RUN_CONTROL: u32 = 0x44018;
pub const IDLE_REGISTER: u32 = 0x4a000;
pub const TILECONFIG0: u32 = 0x48788;
pub const DEEP_SLEEP: u32 = 0x40020;

// USB ML config (64-bit CSRs).
pub const DESCR_EP: u32 = 0x4c148;
pub const MULTI_BO_EP: u32 = 0x4c160;
pub const OUTFEED_CHUNK_LENGTH: u32 = 0x4c058;

// Host-interface-block error status (64-bit CSRs). Non-zero after a run means
// the accelerator faulted during execution.
pub const HIB_ERROR_STATUS: u32 = 0x486f0;
pub const HIB_FIRST_ERROR_STATUS: u32 = 0x48700;

// Interrupt controls (64-bit CSRs).
pub const FATAL_ERR_INT_CONTROL: u32 = 0x4c060;
pub const TOP_LEVEL_INT_CONTROL: [u32; 4] = [0x4c070, 0x4c080, 0x4c090, 0x4c0a0];

// Scalar-core run controls, written to move the core to run/halt (64-bit).
pub const SCALAR_CORE_RUN_CONTROLS: [u32; 5] = [0x44018, 0x44158, 0x44198, 0x441d8, 0x44218];

// Tile run controls (64-bit).
pub const TILE_RUN_CONTROLS: [u32; 10] = [
    0x400c0, 0x40150, 0x40110, 0x40250, 0x40298, 0x402e0, 0x40328, 0x40190, 0x401d0, 0x40210,
];
