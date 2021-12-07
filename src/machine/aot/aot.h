#ifndef CKB_VM_AOT_COMMON_H_
#define CKB_VM_AOT_COMMON_H_

#define ERROR_INVALID_MEMORY_SIZE 0xFFFFFF00
#define ERROR_NOT_ENOUGH_LABELS 0xFFFFFF01
#define ERROR_INVALID_VALUE 0xFFFFFF02

typedef struct {
  dasm_State* d;
  void* labels[lbl__MAX];
  uint32_t npc;
  uint32_t version;
} AotContext;

#define REGISTER_ZERO 0
#define REGISTER_RA 1
#define REGISTER_SP 2
#define REGISTER_GP 3
#define REGISTER_TP 4
#define REGISTER_T0 5
#define REGISTER_T1 6
#define REGISTER_T2 7
#define REGISTER_S0 8
#define REGISTER_S1 9
#define REGISTER_A0 10
#define REGISTER_A1 11
#define REGISTER_A2 12
#define REGISTER_A3 13
#define REGISTER_A4 14
#define REGISTER_A5 15
#define REGISTER_A6 16
#define REGISTER_A7 17
#define REGISTER_S2 18
#define REGISTER_S3 19
#define REGISTER_S4 20
#define REGISTER_S5 21
#define REGISTER_S6 22
#define REGISTER_S7 23
#define REGISTER_S8 24
#define REGISTER_S9 25
#define REGISTER_S10 26
#define REGISTER_S11 27
#define REGISTER_T3 28
#define REGISTER_T4 29
#define REGISTER_T5 30
#define REGISTER_T6 31
#define REGISTER_TEMP1 32
#define REGISTER_TEMP2 33
#define REGISTER_TEMP3 34
#define REGISTER_TEMP4 35
#define REGISTER_TEMP5 36
#define MAXIMUM_REGISTER 36
#define INVALID_REGISTER (MAXIMUM_REGISTER + 1)
#define VALID_REGISTER(r) ((r) <= MAXIMUM_REGISTER)

typedef struct {
  uint64_t registers[32];
  uint64_t pc;
  uint64_t next_pc;
  uint8_t running;
  uint64_t cycles;
  uint64_t max_cycles;
  uint8_t chaos_mode;
  uint32_t chaos_seed;
  uint8_t reset_signal;
  uint8_t isa;
  uint32_t version;
  uint8_t flags[CKB_VM_ASM_RISCV_PAGES];
  uint8_t memory[CKB_VM_ASM_RISCV_MAX_MEMORY];
  uint8_t frames[CKB_VM_ASM_MEMORY_FRAMES];
  /* We won't access traces here */
  uint8_t _traces[CKB_VM_ASM_ASM_CORE_MACHINE_STRUCT_SIZE -
                  CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_TRACES];
} AsmMachine;

extern void inited_memory(uint64_t frame_index, AsmMachine* machine);

#define AOT_TAG_REGISTER 0x1
#define AOT_TAG_IMMEDIATE 0x2
#define AOT_TAG_NATIVE_REGISTER 0x3

typedef uint32_t riscv_register_t;
typedef int32_t native_register_t;

typedef struct {
  uint32_t tag;
  union {
    riscv_register_t reg;
    uint64_t i;
    native_register_t x64_reg;
  } value;
} AotValue;

int aot_value_is_riscv_register(AotValue v, riscv_register_t r)
{
  return (v.tag == AOT_TAG_REGISTER) && (v.value.reg == r);
}

#endif /* CKB_VM_AOT_COMMON_H_ */
