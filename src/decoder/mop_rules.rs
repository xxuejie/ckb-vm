use crate::{
    decoder::Decoder,
    instructions::{
        extract_opcode, instruction_length, set_instruction_length_n, Instruction, Itype, R4type,
        R5type, Rtype, Utype,
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
            let r0 = i0.rd();
            let r1 = i0.rs2();
            if !(i0.rd() == r0
                && i0.rs1() == r0
                && i0.rs2() == r1
                && r0 != r1
                && r0 != ZERO
                && r1 != ZERO)
            {
                return None;
            }

            let i1 = Rtype(insts[1]);
            if !(i1.rd() == r1 && i1.rs1() == r0 && i1.rs2() == r1) {
                return None;
            }

            let i2 = Rtype(insts[2]);
            let r2 = i2.rs2();
            if !(i2.rd() == r0
                && i2.rs1() == r0
                && i2.rs2() == r2
                && r0 != r2
                && r1 != r2
                && r2 != ZERO)
            {
                return None;
            }

            let i3 = Rtype(insts[3]);
            if !(i3.rd() == r2 && i3.rs1() == r0 && i3.rs2() == r2) {
                return None;
            }

            let i4 = Rtype(insts[4]);
            if !(i4.rd() == r1 && i4.rs1() == r1 && i4.rs2() == r2) {
                return None;
            }

            Some(Rtype::new(insts::OP_ADC, r0, r1, r2).0)
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
            let r0 = i0.rs1();
            let r1 = i0.rd();
            if !(i0.rd() == r1
                && i0.rs1() == r0
                && i0.rs2() == r1
                && r0 != r1
                && r0 != ZERO
                && r1 != ZERO)
            {
                return None;
            }

            let i1 = Rtype(insts[1]);
            let r3 = i1.rd();
            if !(i1.rd() == r3
                && i1.rs1() == r0
                && i1.rs2() == r1
                && r0 != r3
                && r1 != r3
                && r3 != ZERO)
            {
                return None;
            }

            let i2 = Rtype(insts[2]);
            let r2 = i2.rs2();
            if !(i2.rd() == r0
                && i2.rs1() == r1
                && i2.rs2() == r2
                && r0 != r2
                && r1 != r2
                && r2 != r3
                && r2 != ZERO)
            {
                return None;
            }

            let i3 = Rtype(insts[3]);
            if !(i3.rd() == r2 && i3.rs1() == r1 && i3.rs2() == r0) {
                return None;
            }

            let i4 = Rtype(insts[4]);
            if !(i4.rd() == r1 && i4.rs1() == r2 && i4.rs2() == r3) {
                return None;
            }

            Some(R4type::new(insts::OP_SBB, r0, r1, r2, r3).0)
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
            let r0 = i0.rd();
            let r1 = i0.rs1();
            let r2 = i0.rs2();
            if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r2 && r0 != r1 && r0 != r2) {
                return None;
            }

            let i1 = Rtype(insts[1]);
            let r3 = i1.rd();
            if !(i1.rd() == r3 && i1.rs1() == r1 && i1.rs2() == r2 && r0 != r3) {
                return None;
            }

            Some(R4type::new(insts::OP_WIDE_MUL, r0, r1, r2, r3).0)
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
            let r0 = i0.rd();
            let r1 = i0.rs1();
            let r2 = i0.rs2();
            if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r2 && r0 != r1 && r0 != r2) {
                return None;
            }

            let i1 = Rtype(insts[1]);
            let r3 = i1.rd();
            if !(i1.rd() == r3 && i1.rs1() == r1 && i1.rs2() == r2 && r0 != r3) {
                return None;
            }

            Some(R4type::new(insts::OP_WIDE_MULU, r0, r1, r2, r3).0)
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
            let r0 = i0.rd();
            let r1 = i0.rs1();
            let r2 = i0.rs2();
            if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r2 && r0 != r1 && r0 != r2) {
                return None;
            }

            let i1 = Rtype(insts[1]);
            let r3 = i1.rd();
            if !(i1.rd() == r3 && i1.rs1() == r1 && i1.rs2() == r2 && r0 != r3) {
                return None;
            }

            Some(R4type::new(insts::OP_WIDE_MULSU, r0, r1, r2, r3).0)
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
            let r0 = i0.rd();
            let r1 = i0.rs1();
            let r2 = i0.rs2();
            if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r2 && r0 != r1 && r0 != r2) {
                return None;
            }

            let i1 = Rtype(insts[1]);
            let r3 = i1.rd();
            if !(i1.rd() == r3 && i1.rs1() == r1 && i1.rs2() == r2 && r0 != r3) {
                return None;
            }

            Some(R4type::new(insts::OP_WIDE_DIV, r0, r1, r2, r3).0)
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
            let r0 = i0.rd();
            let r1 = i0.rs1();
            let r2 = i0.rs2();
            if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r2 && r0 != r1 && r0 != r2) {
                return None;
            }

            let i1 = Rtype(insts[1]);
            let r3 = i1.rd();
            if !(i1.rd() == r3 && i1.rs1() == r1 && i1.rs2() == r2 && r0 != r3) {
                return None;
            }

            Some(R4type::new(insts::OP_WIDE_DIVU, r0, r1, r2, r3).0)
        },
    ),
    (
        // auipc r0, upper
        // jalr RA, r0, lower
        &[insts::OP_AUIPC, insts::OP_JALR_VERSION1],
        1,
        |insts| {
            let i0 = Utype(insts[0]);
            let r0 = i0.rd();
            if !(i0.rd() == r0) {
                return None;
            }

            let i1 = Itype(insts[1]);
            if !(i1.rd() == RA && i1.rs1() == r0) {
                return None;
            }

            let fuze_imm = i0.immediate_s().wrapping_add(i1.immediate_s());
            Some(Utype::new_s(insts::OP_FAR_JUMP_REL, RA, fuze_imm).0)
        },
    ),
    (
        // lui r0, upper
        // jalr RA, r0, lower
        &[insts::OP_LUI, insts::OP_JALR_VERSION1],
        1,
        |insts| {
            let i0 = Utype(insts[0]);
            let r0 = i0.rd();
            if !(i0.rd() == r0) {
                return None;
            }

            let i1 = Itype(insts[1]);
            if !(i1.rd() == RA && i1.rs1() == r0) {
                return None;
            }

            let fuze_imm = i0.immediate_s().wrapping_add(i1.immediate_s());
            Some(Utype::new_s(insts::OP_FAR_JUMP_ABS, RA, fuze_imm).0)
        },
    ),
    (
        // lui r0, upper
        // addiw r0, r0, lower
        &[insts::OP_LUI, insts::OP_ADDIW],
        1,
        |insts| {
            let i0 = Utype(insts[0]);
            let r0 = i0.rd();
            if !(i0.rd() == r0) {
                return None;
            }

            let i1 = Itype(insts[1]);
            if !(i1.rd() == r0 && i1.rs1() == r0) {
                return None;
            }

            let fuze_imm = i0.immediate_s().wrapping_add(i1.immediate_s());
            Some(Utype::new_s(insts::OP_CUSTOM_LOAD_IMM, r0, fuze_imm).0)
        },
    ),
    (
        // add r0, r1, r0
        // sltu r2, r0, r1
        // add r3, r2, r4
        //
        // r0 != r1
        // r0 != r4
        // r2 != r4
        // r0 != x0
        // r2 != x0
        &[insts::OP_ADD, insts::OP_SLTU, insts::OP_ADD],
        2,
        |insts| {
            let i0 = Rtype(insts[0]);
            let r0 = i0.rd();
            let r1 = i0.rs1();
            if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r0 && r0 != r1 && r0 != ZERO) {
                return None;
            }

            let i1 = Rtype(insts[1]);
            let r2 = i1.rd();
            if !(i1.rd() == r2 && i1.rs1() == r0 && i1.rs2() == r1 && r2 != ZERO) {
                return None;
            }

            let i2 = Rtype(insts[2]);
            let r3 = i2.rd();
            let r4 = i2.rs2();
            if !(i2.rd() == r3 && i2.rs1() == r2 && i2.rs2() == r4 && r0 != r4 && r2 != r4) {
                return None;
            }

            Some(R5type::new(insts::OP_ADD3A, r0, r1, r2, r3, r4).0)
        },
    ),
    (
        // add r0, r1, r2
        // sltu r1, r0, r1
        // add r3, r1, r4
        //
        // r0 != r1
        // r0 != r4
        // r1 != r4
        &[insts::OP_ADD, insts::OP_SLTU, insts::OP_ADD],
        2,
        |insts| {
            let i0 = Rtype(insts[0]);
            let r0 = i0.rd();
            let r1 = i0.rs1();
            let r2 = i0.rs2();
            if !(i0.rd() == r0
                && i0.rs1() == r1
                && i0.rs2() == r2
                && r0 != r1
                && r0 != ZERO
                && r1 != ZERO)
            {
                return None;
            }

            let i1 = Rtype(insts[1]);
            if !(i1.rd() == r1 && i1.rs1() == r0 && i1.rs2() == r1) {
                return None;
            }

            let i2 = Rtype(insts[2]);
            let r3 = i2.rd();
            let r4 = i2.rs2();
            if !(i2.rd() == r3 && i2.rs1() == r1 && i2.rs2() == r4 && r0 != r4 && r1 != r4) {
                return None;
            }

            Some(R5type::new(insts::OP_ADD3B, r0, r1, r2, r3, r4).0)
        },
    ),
    (
        // add r0, r1, r2
        // sltu r3, r0, r1
        // add r3, r3, r4
        //
        // r0 != r1
        // r0 != r4
        // r3 != r4
        &[insts::OP_ADD, insts::OP_SLTU, insts::OP_ADD],
        2,
        |insts| {
            let i0 = Rtype(insts[0]);
            let r0 = i0.rd();
            let r1 = i0.rs1();
            let r2 = i0.rs2();
            if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r2 && r0 != r1 && r0 != ZERO) {
                return None;
            }

            let i1 = Rtype(insts[1]);
            let r3 = i1.rd();
            if !(i1.rd() == r3 && i1.rs1() == r0 && i1.rs2() == r1 && r3 != ZERO) {
                return None;
            }

            let i2 = Rtype(insts[2]);
            let r4 = i2.rs2();
            if !(i2.rd() == r3 && i2.rs1() == r3 && i2.rs2() == r4 && r0 != r4 && r3 != r4) {
                return None;
            }

            Some(R5type::new(insts::OP_ADD3C, r0, r1, r2, r3, r4).0)
        },
    ),
    (
        // add r0, r1, r2
        // sltu r3, r0, r1
        //
        // or
        //
        // add r0, r2, r1
        // sltu r3, r0, r1
        //
        // r0 != r1
        // r0 != x0
        &[insts::OP_ADD, insts::OP_SLTU],
        2,
        |insts| {
            let mut i0 = Rtype(insts[0]);
            if i0.rd() == i0.rs1() && i0.rd() != i0.rs2() {
                i0 = swap_operands(&i0);
            }
            let r0 = i0.rd();
            let r1 = i0.rs1();
            let r2 = i0.rs2();
            if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r2 && r0 != r1 && r0 != ZERO) {
                return None;
            }

            let i1 = Rtype(insts[1]);
            let r3 = i1.rd();
            if !(i1.rd() == r3 && i1.rs1() == r0 && i1.rs2() == r1) {
                return None;
            }

            Some(R4type::new(insts::OP_ADCS, r0, r1, r2, r3).0)
        },
    ),
    (
        // sub r0, r1, r2
        // sltu r3, r1, r2
        //
        // r0 != r1
        // r0 != r2
        &[insts::OP_SUB, insts::OP_SLTU],
        2,
        |insts| {
            let i0 = Rtype(insts[0]);
            let r0 = i0.rd();
            let r1 = i0.rs1();
            let r2 = i0.rs2();
            if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r2 && r0 != r1 && r0 != r2) {
                return None;
            }

            let i1 = Rtype(insts[1]);
            let r3 = i1.rd();
            if !(i1.rd() == r3 && i1.rs1() == r1 && i1.rs2() == r2) {
                return None;
            }

            Some(R4type::new(insts::OP_SBBS, r0, r1, r2, r3).0)
        },
    ),
];

fn swap_operands(i: &Rtype) -> Rtype {
    Rtype::new(i.op(), i.rd(), i.rs2(), i.rs1())
}

pub struct MopDecoder<'a, M> {
    decoder: &'a mut Decoder,
    memory: &'a mut M,
    pc: u64,
    insts: Vec<Instruction>,
    sizes: Vec<u8>,

    opcodes: &'static [InstructionOpcode],
    current: usize,
}

impl<'a, M: Memory> MopDecoder<'a, M> {
    pub fn new(decoder: &'a mut Decoder, memory: &'a mut M, pc: u64) -> Self {
        Self {
            decoder,
            memory,
            pc,
            insts: Vec::with_capacity(MAX_INSTS_IN_MOP),
            sizes: Vec::with_capacity(MAX_INSTS_IN_MOP),
            opcodes: &[],
            current: 0,
        }
    }

    pub fn fetch(&mut self) -> Option<Instruction> {
        if self.current >= self.opcodes.len() {
            return None;
        }

        let mut total_size: u8 = self.sizes.iter().sum();
        while self.current >= self.insts.len() {
            let instruction = self
                .decoder
                .decode_raw(self.memory, self.pc + total_size as u64)
                .ok()?;
            let size = instruction_length(instruction);

            self.insts.push(instruction);
            self.sizes.push(size);
            total_size += size;
        }

        let instruction = self.insts[self.current];
        if extract_opcode(instruction) != self.opcodes[self.current] {
            return None;
        }
        self.current += 1;
        Some(instruction)
    }

    pub fn decode(&mut self) -> Result<Option<Instruction>, Error> {
        let head_instruction = self.decoder.decode_raw(self.memory, self.pc)?;

        self.insts.push(head_instruction);
        self.sizes.push(instruction_length(head_instruction));

        let rules: &[(
            &[InstructionOpcode],
            _,
            fn(&mut Self) -> Option<Instruction>,
        )] = &[
            (
                &[
                    insts::OP_ADD,
                    insts::OP_SLTU,
                    insts::OP_ADD,
                    insts::OP_SLTU,
                    insts::OP_OR,
                ],
                1..=u32::MAX,
                Self::check_adc,
            ),
            (
                &[
                    insts::OP_SUB,
                    insts::OP_SLTU,
                    insts::OP_SUB,
                    insts::OP_SLTU,
                    insts::OP_OR,
                ],
                1..=u32::MAX,
                Self::check_sbb,
            ),
            (
                &[insts::OP_MULH, insts::OP_MUL],
                1..=u32::MAX,
                Self::check_wide_mul,
            ),
            (
                &[insts::OP_MULHU, insts::OP_MUL],
                1..=u32::MAX,
                Self::check_wide_mulu,
            ),
            (
                &[insts::OP_MULHSU, insts::OP_MUL],
                1..=u32::MAX,
                Self::check_wide_mulsu,
            ),
            (
                &[insts::OP_DIV, insts::OP_REM],
                1..=u32::MAX,
                Self::check_wide_div,
            ),
            (
                &[insts::OP_DIVU, insts::OP_REMU],
                1..=u32::MAX,
                Self::check_wide_divu,
            ),
            (
                &[insts::OP_AUIPC, insts::OP_JALR_VERSION1],
                1..=u32::MAX,
                Self::check_far_jump_rel,
            ),
            (
                &[insts::OP_LUI, insts::OP_JALR_VERSION1],
                1..=u32::MAX,
                Self::check_far_jump_abs,
            ),
            (
                &[insts::OP_LUI, insts::OP_ADDIW],
                1..=u32::MAX,
                Self::check_custom_load_imm,
            ),
            (
                &[insts::OP_ADD, insts::OP_SLTU, insts::OP_ADD],
                2..=u32::MAX,
                Self::check_add3a,
            ),
            (
                &[insts::OP_ADD, insts::OP_SLTU, insts::OP_ADD],
                2..=u32::MAX,
                Self::check_add3b,
            ),
            (
                &[insts::OP_ADD, insts::OP_SLTU, insts::OP_ADD],
                2..=u32::MAX,
                Self::check_add3c,
            ),
            (
                &[insts::OP_ADD, insts::OP_SLTU],
                2..=u32::MAX,
                Self::check_adcs,
            ),
            (
                &[insts::OP_SUB, insts::OP_SLTU],
                2..=u32::MAX,
                Self::check_sbbs,
            ),
        ];

        for (opcodes, version, checker) in rules {
            if !version.contains(&self.decoder.version()) {
                continue;
            }

            self.opcodes = opcodes;
            self.current = 0;
            if let Some(inst) = checker(self) {
                let size = self.sizes[0..self.opcodes.len()].iter().sum();
                return Ok(Some(set_instruction_length_n(inst, size)));
            }
        }
        Ok(Some(head_instruction))
    }

    fn check_adc(&mut self) -> Option<Instruction> {
        let i0 = Rtype(self.fetch()?);
        let r0 = i0.rd();
        let r1 = i0.rs2();
        if !(i0.rd() == r0
            && i0.rs1() == r0
            && i0.rs2() == r1
            && r0 != r1
            && r0 != ZERO
            && r1 != ZERO)
        {
            return None;
        }

        let i1 = Rtype(self.fetch()?);
        if !(i1.rd() == r1 && i1.rs1() == r0 && i1.rs2() == r1) {
            return None;
        }

        let i2 = Rtype(self.fetch()?);
        let r2 = i2.rs2();
        if !(i2.rd() == r0
            && i2.rs1() == r0
            && i2.rs2() == r2
            && r0 != r2
            && r1 != r2
            && r2 != ZERO)
        {
            return None;
        }

        let i3 = Rtype(self.fetch()?);
        if !(i3.rd() == r2 && i3.rs1() == r0 && i3.rs2() == r2) {
            return None;
        }

        let i4 = Rtype(self.fetch()?);
        if !(i4.rd() == r1 && i4.rs1() == r1 && i4.rs2() == r2) {
            return None;
        }

        Some(Rtype::new(insts::OP_ADC, r0, r1, r2).0)
    }

    fn check_sbb(&mut self) -> Option<Instruction> {
        let i0 = Rtype(self.fetch()?);
        let r0 = i0.rs1();
        let r1 = i0.rd();
        if !(i0.rd() == r1
            && i0.rs1() == r0
            && i0.rs2() == r1
            && r0 != r1
            && r0 != ZERO
            && r1 != ZERO)
        {
            return None;
        }

        let i1 = Rtype(self.fetch()?);
        let r3 = i1.rd();
        if !(i1.rd() == r3
            && i1.rs1() == r0
            && i1.rs2() == r1
            && r0 != r3
            && r1 != r3
            && r3 != ZERO)
        {
            return None;
        }

        let i2 = Rtype(self.fetch()?);
        let r2 = i2.rs2();
        if !(i2.rd() == r0
            && i2.rs1() == r1
            && i2.rs2() == r2
            && r0 != r2
            && r1 != r2
            && r2 != r3
            && r2 != ZERO)
        {
            return None;
        }

        let i3 = Rtype(self.fetch()?);
        if !(i3.rd() == r2 && i3.rs1() == r1 && i3.rs2() == r0) {
            return None;
        }

        let i4 = Rtype(self.fetch()?);
        if !(i4.rd() == r1 && i4.rs1() == r2 && i4.rs2() == r3) {
            return None;
        }

        Some(R4type::new(insts::OP_SBB, r0, r1, r2, r3).0)
    }

    fn check_wide_mul(&mut self) -> Option<Instruction> {
        let i0 = Rtype(self.fetch()?);
        let r0 = i0.rd();
        let r1 = i0.rs1();
        let r2 = i0.rs2();
        if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r2 && r0 != r1 && r0 != r2) {
            return None;
        }

        let i1 = Rtype(self.fetch()?);
        let r3 = i1.rd();
        if !(i1.rd() == r3 && i1.rs1() == r1 && i1.rs2() == r2 && r0 != r3) {
            return None;
        }

        Some(R4type::new(insts::OP_WIDE_MUL, r0, r1, r2, r3).0)
    }

    fn check_wide_mulu(&mut self) -> Option<Instruction> {
        let i0 = Rtype(self.fetch()?);
        let r0 = i0.rd();
        let r1 = i0.rs1();
        let r2 = i0.rs2();
        if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r2 && r0 != r1 && r0 != r2) {
            return None;
        }

        let i1 = Rtype(self.fetch()?);
        let r3 = i1.rd();
        if !(i1.rd() == r3 && i1.rs1() == r1 && i1.rs2() == r2 && r0 != r3) {
            return None;
        }

        Some(R4type::new(insts::OP_WIDE_MULU, r0, r1, r2, r3).0)
    }

    fn check_wide_mulsu(&mut self) -> Option<Instruction> {
        let i0 = Rtype(self.fetch()?);
        let r0 = i0.rd();
        let r1 = i0.rs1();
        let r2 = i0.rs2();
        if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r2 && r0 != r1 && r0 != r2) {
            return None;
        }

        let i1 = Rtype(self.fetch()?);
        let r3 = i1.rd();
        if !(i1.rd() == r3 && i1.rs1() == r1 && i1.rs2() == r2 && r0 != r3) {
            return None;
        }

        Some(R4type::new(insts::OP_WIDE_MULSU, r0, r1, r2, r3).0)
    }

    fn check_wide_div(&mut self) -> Option<Instruction> {
        let i0 = Rtype(self.fetch()?);
        let r0 = i0.rd();
        let r1 = i0.rs1();
        let r2 = i0.rs2();
        if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r2 && r0 != r1 && r0 != r2) {
            return None;
        }

        let i1 = Rtype(self.fetch()?);
        let r3 = i1.rd();
        if !(i1.rd() == r3 && i1.rs1() == r1 && i1.rs2() == r2 && r0 != r3) {
            return None;
        }

        Some(R4type::new(insts::OP_WIDE_DIV, r0, r1, r2, r3).0)
    }

    fn check_wide_divu(&mut self) -> Option<Instruction> {
        let i0 = Rtype(self.fetch()?);
        let r0 = i0.rd();
        let r1 = i0.rs1();
        let r2 = i0.rs2();
        if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r2 && r0 != r1 && r0 != r2) {
            return None;
        }

        let i1 = Rtype(self.fetch()?);
        let r3 = i1.rd();
        if !(i1.rd() == r3 && i1.rs1() == r1 && i1.rs2() == r2 && r0 != r3) {
            return None;
        }

        Some(R4type::new(insts::OP_WIDE_DIVU, r0, r1, r2, r3).0)
    }

    fn check_far_jump_rel(&mut self) -> Option<Instruction> {
        let i0 = Utype(self.fetch()?);
        let r0 = i0.rd();
        if !(i0.rd() == r0) {
            return None;
        }

        let i1 = Itype(self.fetch()?);
        if !(i1.rd() == RA && i1.rs1() == r0) {
            return None;
        }

        let fuze_imm = i0.immediate_s().wrapping_add(i1.immediate_s());
        Some(Utype::new_s(insts::OP_FAR_JUMP_REL, RA, fuze_imm).0)
    }

    fn check_far_jump_abs(&mut self) -> Option<Instruction> {
        let i0 = Utype(self.fetch()?);
        let r0 = i0.rd();
        if !(i0.rd() == r0) {
            return None;
        }

        let i1 = Itype(self.fetch()?);
        if !(i1.rd() == RA && i1.rs1() == r0) {
            return None;
        }

        let fuze_imm = i0.immediate_s().wrapping_add(i1.immediate_s());
        Some(Utype::new_s(insts::OP_FAR_JUMP_ABS, RA, fuze_imm).0)
    }

    fn check_custom_load_imm(&mut self) -> Option<Instruction> {
        let i0 = Utype(self.fetch()?);
        let r0 = i0.rd();
        if !(i0.rd() == r0) {
            return None;
        }

        let i1 = Itype(self.fetch()?);
        if !(i1.rd() == r0 && i1.rs1() == r0) {
            return None;
        }

        let fuze_imm = i0.immediate_s().wrapping_add(i1.immediate_s());
        Some(Utype::new_s(insts::OP_CUSTOM_LOAD_IMM, r0, fuze_imm).0)
    }

    fn check_add3a(&mut self) -> Option<Instruction> {
        let i0 = Rtype(self.fetch()?);
        let r0 = i0.rd();
        let r1 = i0.rs1();
        if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r0 && r0 != r1 && r0 != ZERO) {
            return None;
        }

        let i1 = Rtype(self.fetch()?);
        let r2 = i1.rd();
        if !(i1.rd() == r2 && i1.rs1() == r0 && i1.rs2() == r1 && r2 != ZERO) {
            return None;
        }

        let i2 = Rtype(self.fetch()?);
        let r3 = i2.rd();
        let r4 = i2.rs2();
        if !(i2.rd() == r3 && i2.rs1() == r2 && i2.rs2() == r4 && r0 != r4 && r2 != r4) {
            return None;
        }

        Some(R5type::new(insts::OP_ADD3A, r0, r1, r2, r3, r4).0)
    }

    fn check_add3b(&mut self) -> Option<Instruction> {
        let i0 = Rtype(self.fetch()?);
        let r0 = i0.rd();
        let r1 = i0.rs1();
        let r2 = i0.rs2();
        if !(i0.rd() == r0
            && i0.rs1() == r1
            && i0.rs2() == r2
            && r0 != r1
            && r0 != ZERO
            && r1 != ZERO)
        {
            return None;
        }

        let i1 = Rtype(self.fetch()?);
        if !(i1.rd() == r1 && i1.rs1() == r0 && i1.rs2() == r1) {
            return None;
        }

        let i2 = Rtype(self.fetch()?);
        let r3 = i2.rd();
        let r4 = i2.rs2();
        if !(i2.rd() == r3 && i2.rs1() == r1 && i2.rs2() == r4 && r0 != r4 && r1 != r4) {
            return None;
        }

        Some(R5type::new(insts::OP_ADD3B, r0, r1, r2, r3, r4).0)
    }

    fn check_add3c(&mut self) -> Option<Instruction> {
        let i0 = Rtype(self.fetch()?);
        let r0 = i0.rd();
        let r1 = i0.rs1();
        let r2 = i0.rs2();
        if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r2 && r0 != r1 && r0 != ZERO) {
            return None;
        }

        let i1 = Rtype(self.fetch()?);
        let r3 = i1.rd();
        if !(i1.rd() == r3 && i1.rs1() == r0 && i1.rs2() == r1 && r3 != ZERO) {
            return None;
        }

        let i2 = Rtype(self.fetch()?);
        let r4 = i2.rs2();
        if !(i2.rd() == r3 && i2.rs1() == r3 && i2.rs2() == r4 && r0 != r4 && r3 != r4) {
            return None;
        }

        Some(R5type::new(insts::OP_ADD3C, r0, r1, r2, r3, r4).0)
    }

    fn check_adcs(&mut self) -> Option<Instruction> {
        let mut i0 = Rtype(self.fetch()?);
        if i0.rd() == i0.rs1() && i0.rd() != i0.rs2() {
            i0 = swap_operands(&i0);
        }
        let r0 = i0.rd();
        let r1 = i0.rs1();
        let r2 = i0.rs2();
        if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r2 && r0 != r1 && r0 != ZERO) {
            return None;
        }

        let i1 = Rtype(self.fetch()?);
        let r3 = i1.rd();
        if !(i1.rd() == r3 && i1.rs1() == r0 && i1.rs2() == r1) {
            return None;
        }

        Some(R4type::new(insts::OP_ADCS, r0, r1, r2, r3).0)
    }

    fn check_sbbs(&mut self) -> Option<Instruction> {
        let i0 = Rtype(self.fetch()?);
        let r0 = i0.rd();
        let r1 = i0.rs1();
        let r2 = i0.rs2();
        if !(i0.rd() == r0 && i0.rs1() == r1 && i0.rs2() == r2 && r0 != r1 && r0 != r2) {
            return None;
        }

        let i1 = Rtype(self.fetch()?);
        let r3 = i1.rd();
        if !(i1.rd() == r3 && i1.rs1() == r1 && i1.rs2() == r2) {
            return None;
        }

        Some(R4type::new(insts::OP_SBBS, r0, r1, r2, r3).0)
    }
}
