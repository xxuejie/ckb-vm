use crate::{
    decoder::Decoder,
    instructions::{
        extract_opcode, instruction_length, set_instruction_length_n, Instruction, Itype, R4type,
        Rtype, Utype,
    },
    memory::Memory,
    Error,
};
use ckb_vm_definitions::{
    instructions::{self as insts, InstructionOpcode},
    registers::{RA, ZERO},
};

type Rule<'a> = (
    &'a [InstructionOpcode],
    u32,
    fn(&[Instruction]) -> Option<Instruction>,
);

const MAX_INSTS_IN_MOP: usize = 5;

const RULES: &[Rule] = &[
    (
        // add  r0, r0, r1
        // sltu r1, r0, r1
        // add  r0, r0, r2
        // sltu r2, r0, r2
        // or   r1, r1, r2
        //
        // r0 != r1
        // r0 != r2
        // r1 != r2
        // r0 != x0
        // r1 != x0
        // r2 != x0
        &[
            insts::OP_ADD,
            insts::OP_SLTU,
            insts::OP_ADD,
            insts::OP_SLTU,
            insts::OP_OR,
        ],
        1,
        |insts| {
            let i0 = Rtype(insts[0]);
            let i1 = Rtype(insts[1]);
            let i2 = Rtype(insts[2]);
            let i3 = Rtype(insts[3]);
            let i4 = Rtype(insts[4]);

            let r0 = i0.rd();
            let r1 = i0.rs2();
            let r2 = i2.rs2();

            if i0.rd() == r0
                && i0.rs1() == r0
                && i0.rs2() == r1
                && i1.rd() == r1
                && i1.rs1() == r0
                && i1.rs2() == r1
                && i2.rd() == r0
                && i2.rs1() == r0
                && i2.rs2() == r2
                && i3.rd() == r2
                && i3.rs1() == r0
                && i3.rs2() == r2
                && i4.rd() == r1
                && i4.rs1() == r1
                && i4.rs2() == r2
                && r0 != r1
                && r0 != r2
                && r1 != r2
                && r0 != ZERO
                && r1 != ZERO
                && r2 != ZERO
            {
                Some(Rtype::new(insts::OP_ADC, r0, r1, r2).0)
            } else {
                None
            }
        },
    ),
    (
        // sub  r1, r0, r1
        // sltu r3, r0, r1
        // sub  r0, r1, r2
        // sltu r2, r1, r0
        // or   r1, r2, r3
        //
        // r0 != r1
        // r0 != r2
        // r0 != r3
        // r1 != r2
        // r1 != r3
        // r2 != r3
        // r0 != x0
        // r1 != x0
        // r2 != x0
        // r3 != x0
        &[
            insts::OP_SUB,
            insts::OP_SLTU,
            insts::OP_SUB,
            insts::OP_SLTU,
            insts::OP_OR,
        ],
        1,
        |insts| {
            let i0 = Rtype(insts[0]);
            let i1 = Rtype(insts[1]);
            let i2 = Rtype(insts[2]);
            let i3 = Rtype(insts[3]);
            let i4 = Rtype(insts[4]);

            let r0 = i0.rs1();
            let r1 = i0.rd();
            let r2 = i2.rs2();
            let r3 = i1.rd();

            if i0.rd() == r1
                && i0.rs1() == r0
                && i0.rs2() == r1
                && i1.rd() == r3
                && i1.rs1() == r0
                && i1.rs2() == r1
                && i2.rd() == r0
                && i2.rs1() == r1
                && i2.rs2() == r2
                && i3.rd() == r2
                && i3.rs1() == r1
                && i3.rs2() == r0
                && i4.rd() == r1
                && i4.rs1() == r2
                && i4.rs2() == r3
                && r0 != r1
                && r0 != r2
                && r0 != r3
                && r1 != r2
                && r1 != r3
                && r2 != r3
                && r0 != ZERO
                && r1 != ZERO
                && r2 != ZERO
                && r3 != ZERO
            {
                Some(R4type::new(insts::OP_SBB, r0, r1, r2, r3).0)
            } else {
                None
            }
        },
    ),
    (
        // mulh r0, r1, r2
        // mul r3, r1, r2
        //
        // r0 != r1
        // r0 != r2
        // r0 != r3
        &[insts::OP_MULH, insts::OP_MUL],
        1,
        |insts| {
            let i0 = Rtype(insts[0]);
            let i1 = Rtype(insts[1]);

            let r0 = i0.rd();
            let r1 = i0.rs1();
            let r2 = i0.rs2();
            let r3 = i1.rd();

            if i0.rd() == r0
                && i0.rs1() == r1
                && i0.rs2() == r2
                && i1.rd() == r3
                && i1.rs1() == r1
                && i1.rs2() == r2
                && r0 != r1
                && r0 != r2
                && r0 != r3
            {
                Some(R4type::new(insts::OP_WIDE_MUL, r0, r1, r2, r3).0)
            } else {
                None
            }
        },
    ),
    (
        // mulhu r0, r1, r2
        // mul r3, r1, r2
        //
        // r0 != r1
        // r0 != r2
        // r0 != r3
        &[insts::OP_MULHU, insts::OP_MUL],
        1,
        |insts| {
            let i0 = Rtype(insts[0]);
            let i1 = Rtype(insts[1]);

            let r0 = i0.rd();
            let r1 = i0.rs1();
            let r2 = i0.rs2();
            let r3 = i1.rd();

            if i0.rd() == r0
                && i0.rs1() == r1
                && i0.rs2() == r2
                && i1.rd() == r3
                && i1.rs1() == r1
                && i1.rs2() == r2
                && r0 != r1
                && r0 != r2
                && r0 != r3
            {
                Some(R4type::new(insts::OP_WIDE_MULU, r0, r1, r2, r3).0)
            } else {
                None
            }
        },
    ),
    (
        // mulhsu r0, r1, r2
        // mul r3, r1, r2
        //
        // r0 != r1
        // r0 != r2
        // r0 != r3
        &[insts::OP_MULHSU, insts::OP_MUL],
        1,
        |insts| {
            let i0 = Rtype(insts[0]);
            let i1 = Rtype(insts[1]);

            let r0 = i0.rd();
            let r1 = i0.rs1();
            let r2 = i0.rs2();
            let r3 = i1.rd();

            if i0.rd() == r0
                && i0.rs1() == r1
                && i0.rs2() == r2
                && i1.rd() == r3
                && i1.rs1() == r1
                && i1.rs2() == r2
                && r0 != r1
                && r0 != r2
                && r0 != r3
            {
                Some(R4type::new(insts::OP_WIDE_MULSU, r0, r1, r2, r3).0)
            } else {
                None
            }
        },
    ),
    (
        // div r0, r1, r2
        // rem r3, r1, r2
        //
        // r0 != r1
        // r0 != r2
        // r0 != r3
        &[insts::OP_DIV, insts::OP_REM],
        1,
        |insts| {
            let i0 = Rtype(insts[0]);
            let i1 = Rtype(insts[1]);

            let r0 = i0.rd();
            let r1 = i0.rs1();
            let r2 = i0.rs2();
            let r3 = i1.rd();

            if i0.rd() == r0
                && i0.rs1() == r1
                && i0.rs2() == r2
                && i1.rd() == r3
                && i1.rs1() == r1
                && i1.rs2() == r2
                && r0 != r1
                && r0 != r2
                && r0 != r3
            {
                Some(R4type::new(insts::OP_WIDE_DIV, r0, r1, r2, r3).0)
            } else {
                None
            }
        },
    ),
    (
        // divu r0, r1, r2
        // rem r3, r1, r2
        //
        // r0 != r1
        // r0 != r2
        // r0 != r3
        &[insts::OP_DIVU, insts::OP_REMU],
        1,
        |insts| {
            let i0 = Rtype(insts[0]);
            let i1 = Rtype(insts[1]);

            let r0 = i0.rd();
            let r1 = i0.rs1();
            let r2 = i0.rs2();
            let r3 = i1.rd();

            if i0.rd() == r0
                && i0.rs1() == r1
                && i0.rs2() == r2
                && i1.rd() == r3
                && i1.rs1() == r1
                && i1.rs2() == r2
                && r0 != r1
                && r0 != r2
                && r0 != r3
            {
                Some(R4type::new(insts::OP_WIDE_DIVU, r0, r1, r2, r3).0)
            } else {
                None
            }
        },
    ),
    (
        // auipc r0, upper
        // jalr RA, r0, lower
        &[insts::OP_AUIPC, insts::OP_JALR_VERSION1],
        1,
        |insts| {
            let i0 = Utype(insts[0]);
            let i1 = Itype(insts[1]);

            let r0 = i0.rd();

            if i0.rd() == r0 && i1.rd() == RA && i1.rs1() == r0 {
                let fuze_imm = i0.immediate_s().wrapping_add(i1.immediate_s());
                Some(Utype::new_s(insts::OP_FAR_JUMP_REL, RA, fuze_imm).0)
            } else {
                None
            }
        },
    ),
    (
        // lui r0, upper
        // jalr RA, r0, lower
        &[insts::OP_LUI, insts::OP_JALR_VERSION1],
        1,
        |insts| {
            let i0 = Utype(insts[0]);
            let i1 = Itype(insts[1]);

            let r0 = i0.rd();

            if i0.rd() == r0 && i1.rd() == RA && i1.rs1() == r0 {
                let fuze_imm = i0.immediate_s().wrapping_add(i1.immediate_s());
                Some(Utype::new_s(insts::OP_FAR_JUMP_ABS, RA, fuze_imm).0)
            } else {
                None
            }
        },
    ),
    (
        // lui r0, upper
        // addiw r0, r0, lower
        &[insts::OP_LUI, insts::OP_ADDIW],
        1,
        |insts| {
            let i0 = Utype(insts[0]);
            let i1 = Itype(insts[1]);

            let r0 = i0.rd();

            if i0.rd() == r0 && i1.rd() == r0 && i1.rs1() == r0 {
                let fuze_imm = i0.immediate_s().wrapping_add(i1.immediate_s());
                Some(Utype::new_s(insts::OP_CUSTOM_LOAD_IMM, r0, fuze_imm).0)
            } else {
                None
            }
        },
    ),
];

pub struct MopDecoder<'a, M> {
    decoder: &'a mut Decoder,
    memory: &'a mut M,
    pc: u64,
}

impl<'a, M: Memory> MopDecoder<'a, M> {
    pub fn new(decoder: &'a mut Decoder, memory: &'a mut M, pc: u64) -> Self {
        Self {
            decoder,
            memory,
            pc,
        }
    }

    pub fn decode(&mut self) -> Result<Option<Instruction>, Error> {
        let head_instruction = self.decoder.decode_raw(self.memory, self.pc)?;

        let mut insts = Vec::with_capacity(MAX_INSTS_IN_MOP);
        let mut sizes = Vec::with_capacity(MAX_INSTS_IN_MOP);

        insts.push(head_instruction);
        sizes.push(instruction_length(head_instruction));

        for rule in RULES {
            if let Some(inst) = self.check_rule(rule, &mut insts, &mut sizes)? {
                return Ok(Some(inst));
            }
        }

        Ok(None)
    }

    #[inline(always)]
    pub fn check_rule(
        &mut self,
        rule: &Rule,
        insts: &mut Vec<Instruction>,
        sizes: &mut Vec<u8>,
    ) -> Result<Option<Instruction>, Error> {
        let (opcodes, version, f) = rule;
        if self.decoder.version() < *version {
            return Ok(None);
        }
        let loaded_length = insts.len();
        // Fast path checking loaded instruction's opcodes
        for i in 0..std::cmp::min(loaded_length, opcodes.len()) {
            if extract_opcode(insts[i]) != opcodes[i] {
                return Ok(None);
            }
        }
        {
            let mut total_size: u8 = sizes.iter().sum();
            // Decode new instructions when necessary
            for i in loaded_length..opcodes.len() {
                let instruction = self
                    .decoder
                    .decode_raw(self.memory, self.pc + total_size as u64)?;
                insts.push(instruction);
                let size = instruction_length(instruction);
                sizes.push(size);
                total_size += size;

                if extract_opcode(instruction) != opcodes[i] {
                    return Ok(None);
                }
            }
        }
        // Testing against the matching function
        Ok(f(&insts[0..opcodes.len()]).map(|inst| {
            let size = sizes[0..opcodes.len()].iter().sum();
            set_instruction_length_n(inst, size)
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_insts_in_mop() {
        let max_insts = RULES
            .iter()
            .map(|(opcodes, _, _)| opcodes.len())
            .max()
            .unwrap();
        assert_eq!(max_insts, MAX_INSTS_IN_MOP)
    }
}
