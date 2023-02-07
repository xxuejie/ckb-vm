use crate::instructions::{b, i, m, rvc, Instruction, InstructionFactory, Register};
use crate::memory::Memory;
use crate::{Error, ISA_B, ISA_MOP, RISCV_MAX_MEMORY, RISCV_PAGESIZE};

mod mop_rules;

const RISCV_PAGESIZE_MASK: u64 = RISCV_PAGESIZE as u64 - 1;
const INSTRUCTION_CACHE_SIZE: usize = 4096;

pub struct Decoder {
    factories: Vec<InstructionFactory>,
    mop: bool,
    version: u32,
    // use a cache of instructions to avoid decoding the same instruction twice, pc is the key and the instruction is the value
    instructions_cache: [(u64, u64); INSTRUCTION_CACHE_SIZE],
    decodes: usize,
}

impl Decoder {
    pub fn new(mop: bool, version: u32) -> Decoder {
        Decoder {
            factories: vec![],
            mop,
            version,
            instructions_cache: [(RISCV_MAX_MEMORY as u64, 0); INSTRUCTION_CACHE_SIZE],
            decodes: 0,
        }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn add_instruction_factory(&mut self, factory: InstructionFactory) {
        self.factories.push(factory);
    }

    // This method is used to decode instruction raw bits from memory pointed
    // by current PC. Right now we support 32-bit instructions and RVC compressed
    // instructions. In future version we might add support for longer instructions.
    //
    // This decode method actually leverages a trick from little endian encoding:
    // the format for a full 32 bit RISC-V instruction is as follows:
    //
    // WWWWWWWWZZZZZZZZYYYYYYYYXXXXXX11
    //
    // While the format for a 16 bit RVC RIST-V instruction is one of the following 3:
    //
    // YYYYYYYYXXXXXX00
    // YYYYYYYYXXXXXX01
    // YYYYYYYYXXXXXX10
    //
    // Here X, Y, Z and W stands for arbitrary bits.
    // However the above is the representation in a 16-bit or 32-bit integer, since
    // we are using little endian, in memory it's actually in following reversed order:
    //
    // XXXXXX11 YYYYYYYY ZZZZZZZZ WWWWWWWW
    // XXXXXX00 YYYYYYYY
    // XXXXXX01 YYYYYYYY
    // XXXXXX10 YYYYYYYY
    //
    // One observation here, is the first byte in memory is always the least
    // significant byte whether we load a 32-bit or 16-bit integer.
    // So when we are decoding an instruction, we can first load 2 bytes forming
    // a 16-bit integer, then we check the 2 least significant bits, if the 2 bitss
    // are 0b11, we know this is a 32-bit instruction, we should load another 2 bytes
    // from memory and concat the 2 16-bit integers into a full 32-bit integers.
    // Otherwise, we know we are loading a RVC integer, and we are done here.
    // Also, due to RISC-V encoding behavior, it's totally okay when we cast a 16-bit
    // RVC instruction into a 32-bit instruction, the meaning of the instruction stays
    // unchanged in the cast conversion.
    fn decode_bits<M: Memory>(&self, memory: &mut M, pc: u64) -> Result<u32, Error> {
        // when the address is not the last 2 bytes of an executable page,
        // use a faster path to load instruction bits
        if pc & RISCV_PAGESIZE_MASK < RISCV_PAGESIZE_MASK - 1 {
            let mut instruction_bits = memory.execute_load32(pc)?;
            if instruction_bits & 0x3 != 0x3 {
                instruction_bits &= 0xffff;
            }
            Ok(instruction_bits)
        } else {
            let mut instruction_bits = u32::from(memory.execute_load16(pc)?);
            if instruction_bits & 0x3 == 0x3 {
                instruction_bits |= u32::from(memory.execute_load16(pc + 2)?) << 16;
            }
            Ok(instruction_bits)
        }
    }

    pub fn decode_raw<M: Memory>(&mut self, memory: &mut M, pc: u64) -> Result<Instruction, Error> {
        // self.decodes += 1;
        if pc as usize >= RISCV_MAX_MEMORY {
            return Err(Error::MemOutOfBound);
        }
        let instruction_bits = self.decode_bits(memory, pc)?;
        for factory in &self.factories {
            if let Some(instruction) = factory(instruction_bits, self.version) {
                return Ok(instruction);
            }
        }
        Err(Error::InvalidInstruction {
            pc,
            instruction: instruction_bits,
        })
    }

    // Macro-Operation Fusion (also Macro-Op Fusion, MOP Fusion, or Macrofusion) is a hardware optimization technique found
    // in many modern microarchitectures whereby a series of adjacent macro-operations are merged into a single
    // macro-operation prior or during decoding. Those instructions are later decoded into fused-µOPs.
    //
    // - https://riscv.org/wp-content/uploads/2016/07/Tue1130celio-fusion-finalV2.pdf
    // - https://en.wikichip.org/wiki/macro-operation_fusion#Proposed_fusion_operations
    // - https://carrv.github.io/2017/papers/clark-rv8-carrv2017.pdf
    pub fn decode_mop<M: Memory>(&mut self, memory: &mut M, pc: u64) -> Result<Instruction, Error> {
        let mut mop_decoder = mop_rules::MopDecoder::new(self, memory, pc);
        match mop_decoder.decode()? {
            Some(i) => Ok(i),
            None => self.decode_raw(memory, pc),
        }
    }

    pub fn decode<M: Memory>(&mut self, memory: &mut M, pc: u64) -> Result<Instruction, Error> {
        let instruction_cache_key = {
            // according to RISC-V instruction encoding, the lowest bit in PC will always be zero
            let addr = pc >> 1;
            // Here we try to balance between local code and remote code. At times,
            // we can find the code jumping to a remote function(e.g., memcpy or
            // alloc), then resumes execution at a local location. Previous cache
            // key only optimizes for local operations, while this new cache key
            // balances the code between a 8192-byte local region, and certain remote
            // code region. Notice the value 12 and 8 here are chosen by empirical
            // evidence.
            (addr & 0xFF) | (addr >> 12 << 8)
        } as usize
            % INSTRUCTION_CACHE_SIZE;
        let cached_instruction = self.instructions_cache[instruction_cache_key];
        if cached_instruction.0 == pc {
            return Ok(cached_instruction.1);
        }
        let instruction = if self.mop {
            self.decode_mop(memory, pc)
        } else {
            self.decode_raw(memory, pc)
        }?;
        self.instructions_cache[instruction_cache_key] = (pc, instruction);
        Ok(instruction)
    }

    pub fn reset_instructions_cache(&mut self) {
        self.instructions_cache = [(RISCV_MAX_MEMORY as u64, 0); INSTRUCTION_CACHE_SIZE];
    }
}

pub fn build_decoder<R: Register>(isa: u8, version: u32) -> Decoder {
    let mut decoder = Decoder::new(isa & ISA_MOP != 0, version);
    decoder.add_instruction_factory(rvc::factory::<R>);
    decoder.add_instruction_factory(i::factory::<R>);
    decoder.add_instruction_factory(m::factory::<R>);
    if isa & ISA_B != 0 {
        decoder.add_instruction_factory(b::factory::<R>);
    }
    decoder
}

// impl Drop for Decoder {
    // fn drop(&mut self) {
        // println!("Decode raw: {}", self.decodes);
    // }
// }
