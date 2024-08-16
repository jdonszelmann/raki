//! raki
//!
//! `raki` is a RISC-V instruction decoder written in Rust.
//!
//! - Both 32/64bit support.
//! - Support `rv32/64imac`, `Zicsr`, `Zifencei` extensions.
//! - Implement Display trait for formatting.
//!
//! # Usage
//! Call the `decode` as u16/u32 method.
//! ```
//! use raki::{BaseIOpcode, Decode, Instruction, Isa, OpcodeKind};
//!
//! fn example() {
//!     let inst_bytes: u32 = 0b1110_1110_1100_0010_1000_0010_1001_0011;
//!     let inst: Instruction = match inst_bytes.decode(Isa::Rv32) {
//!         Ok(inst) => inst,
//!         Err(e) => panic!("decoding failed due to {e:?}"),
//!     };
//!
//!     assert_eq!(inst.opc, OpcodeKind::BaseI(BaseIOpcode::ADDI));
//!     println!("{inst}");
//! }
//! // --output--
//! // addi t0, t0, -276
//! ```

#![no_std]

mod decode;
mod instruction;

// re-export
pub use crate::decode::{Decode, DecodingError};
pub use crate::instruction::{
    a_extension::AOpcode, base_i::BaseIOpcode, c_extension::COpcode, m_extension::MOpcode,
    priv_extension::PrivOpcode, zicntr_extension::ZicntrOpcode, zicsr_extension::ZicsrOpcode,
    zifencei_extension::ZifenceiOpcode, InstFormat, Instruction, OpcodeKind,
};

/// Target isa.
#[derive(Copy, Clone)]
pub enum Isa {
    /// 32 bit architecture.
    Rv32,
    /// 64 bit architecture.
    Rv64,
}

/// RISC-V extensions
#[derive(Debug)]
enum Extensions {
    /// Base Integer Instruction Set
    BaseI,
    /// Integer Multiplication and Division
    M,
    /// Atomic Instructions
    A,
    /// Compressed Instructions
    C,
    /// Instruction-Fetch Fence
    Zifencei,
    /// Control and Status Register Instructions
    Zicsr,
    /// Base Counters and Timers
    Zicntr,
    /// Privileged Instructions
    Priv,
}

#[cfg(test)]
mod tests {
    #[test]
    fn inst_eq_test() {
        use crate::decode::Decode;
        use crate::instruction::{base_i::BaseIOpcode, InstFormat, Instruction, OpcodeKind};
        use crate::Isa;

        assert_eq!(
            0b1111_1111_1001_1111_1111_0000_0110_1111_u32.decode(Isa::Rv64),
            Ok(Instruction {
                opc: OpcodeKind::BaseI(BaseIOpcode::JAL),
                rd: Some(0),
                rs1: None,
                rs2: None,
                imm: Some(-8),
                inst_format: InstFormat::JFormat,
            })
        );

        assert_eq!(
            0b1111_1111_1001_1111_1111_0000_0110_1111_u32
                .decode(Isa::Rv64)
                .unwrap()
                .opc,
            OpcodeKind::BaseI(BaseIOpcode::JAL),
        )
    }
}
