#define CKB_VM_ASM_GENERATE_LABEL_TABLES 1
#include "cdefinitions_generated.h"

#define INT64_MIN 0x8000000000000000
#define UINT64_MAX 0xffffffffffffffff

#define ARG1 x0

/* rax is used both in Microsoft x64 and System V AMD64 ABI */
#define ARG_RET x0

#define MACHINE x1
#define TRACE x2

/*
 * INST_PC contains the current address of decoded Instruction in
 * Trace item, which is different from the RISC-V PC
 */
#define INST_PC x3
#define INST_ARGS x4
#define INST x5

/*
 * Rules to meet when considering register allocations:
 * * RD and TEMP1 cannot be %rcx to set aside %cl for shifts
 * * RS2r and TEMP1 cannot be %rax to allow using imul and idiv
 * * RS2r cannot be %rdx to allow using idiv
  */
#define RD_RS2s x6
#define RD RD_RS2s
#define RS1 x7
#define RS2r x8
#define RS2s RD_RS2s
#define IMMEDIATE x9
#define TEMP1 x10

#define PC_ADDRESS \
  [MACHINE], CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC

#define ZERO_ADDRESS \
  [MACHINE], CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS

#define SP_ADDRESS \
  [MACHINE], CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS + CKB_VM_ASM_REGISTER_SP * 8

#define REGISTER_ADDRESS(r) \
  [MACHINE, r, lsl 3]

#define CHECK_READ_BOUND_VERSION1(length) \
  ldr RS1, REGISTER_ADDRESS(RS1); \
  add RS1, RS1, IMMEDIATE; \
  mov TEMP1, RS1; \
  cmp TEMP1, CKB_VM_ASM_RISCV_MAX_MEMORY; \
  bhs .exit_out_of_bound; \
  add TEMP1, TEMP1, length; \
  cmp TEMP1, CKB_VM_ASM_RISCV_MAX_MEMORY; \
  beq 1f; \
  bhs .exit_out_of_bound; \
1:

#define CHECK_WRITE_PERMISSION(address_reg, temp_reg1, temp_reg2, length) \
  mov temp_reg1, address_reg; \
  lsr temp_reg1, temp_reg1, CKB_VM_ASM_RISCV_PAGE_SHIFTS; \
  cmp temp_reg1, CKB_VM_ASM_RISCV_PAGES; \
  bhs .exit_out_of_bound; \
  ldrb temp_reg2, [MACHINE, temp_reg1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FLAGS]; \
  and temp_reg2, temp_reg2, CKB_VM_ASM_MEMORY_FLAG_WXORX_BIT; \
  cmp temp_reg2, CKB_VM_ASM_MEMORY_FLAG_WRITABLE; \
  bne .exit_invalid_permission; \
  add temp_reg1, temp_reg1, 1; \
  lsl temp_reg1, temp_reg1, CKB_VM_ASM_RISCV_PAGE_SHIFTS; \
  mov temp_reg2, address_reg; \
  add temp_reg2, temp_reg2, length; \
  cmp temp_reg1, temp_reg2; \
  bhs 1f; \
  lsr temp_reg1, temp_reg1, CKB_VM_ASM_RISCV_PAGE_SHIFTS; \
  cmp temp_reg1, CKB_VM_ASM_RISCV_PAGES; \
  bhs .exit_out_of_bound; \
  ldrb temp_reg2, [MACHINE, temp_reg1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FLAGS]; \
  and temp_reg2, temp_reg2, CKB_VM_ASM_MEMORY_FLAG_WXORX_BIT; \
  cmp temp_reg2, CKB_VM_ASM_MEMORY_FLAG_WRITABLE; \
  bne .exit_invalid_permission; \
1:

#define ADDRESS_TO_SLOT_ADDRESS(r) \
  lsr r, r, 5; \
  and r, r, 8191; \
  mov x18, CKB_VM_ASM_TRACE_STRUCT_SIZE; \
  mul r, r, x18; \
  add r, r, MACHINE; \
  add r, r, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_TRACES

#define WRITE_RD_VALUE(v, temp_reg) \
  cmp RD, 0; \
  beq 1f; \
  mov temp_reg, v; \
  str temp_reg, REGISTER_ADDRESS(RD); \
1:

#define WRITE_RD(v) \
  cmp RD, 0; \
  beq 1f; \
  str v, REGISTER_ADDRESS(RD); \
1:

#define NEXT_INST \
  ldr INST, [INST_ARGS]; \
  add INST_ARGS, INST_ARGS, 8; \
  asr RD_RS2s, INST, 8; \
  uxtb RD_RS2s, RD_RS2s; \
  asr INST, INST, 32; \
  ldr TEMP1, [INST_PC]; \
  add INST_PC, INST_PC 8; \
  br INST_PC

#define DECODE_R \
  asr RS2r, INST, 8; \
  uxtb RS1, INST; \
  uxtb RS2r, RS2r

#define DECODE_I \
  uxtb RS1, INST; \
  asr IMMEDIATE, INST, 8

#define DECODE_S DECODE_I

#define DECODE_U

  // ######################### WIP checkpoint ##############################
#ifdef __APPLE__
.globl _ckb_vm_x64_execute
_ckb_vm_x64_execute:
#else
.globl ckb_vm_x64_execute
#endif
ckb_vm_x64_execute:
  mov ARG1, MACHINE
.p2align 3
.CKB_VM_ASM_LABEL_OP_CUSTOM_TRACE_END:
.prepare_trace:
  movq PC_ADDRESS, %rax
  mov %eax, %ecx
  shr $5, %eax
  andq $8191, %rax
  imul $CKB_VM_ASM_TRACE_STRUCT_SIZE, %eax
  lea CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_TRACES(MACHINE, %rax), TRACE
  movq CKB_VM_ASM_TRACE_OFFSET_ADDRESS(TRACE), %rdx
  cmp %rcx, %rdx
  jne .exit_trace
  movzbl CKB_VM_ASM_TRACE_OFFSET_LENGTH(TRACE), %edx
  cmp $0, %rdx
  je .exit_trace
  movq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_CYCLES(MACHINE), %rax
  addq CKB_VM_ASM_TRACE_OFFSET_CYCLES(TRACE), %rax
  cmp CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MAX_CYCLES(MACHINE), %rax
  ja .exit_max_cycles_exceeded
  movq %rax, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_CYCLES(MACHINE)
  addq %rdx, PC_ADDRESS
  lea CKB_VM_ASM_TRACE_OFFSET_INSTRUCTIONS(TRACE), INST_ARGS
  lea CKB_VM_ASM_TRACE_OFFSET_THREAD(TRACE), INST_PC
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ADDI:
.CKB_VM_ASM_LABEL_OP_RVC_ADDI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ADD:
.CKB_VM_ASM_LABEL_OP_RVC_ADD:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  addq REGISTER_ADDRESS(RS2r), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ADDIW:
.CKB_VM_ASM_LABEL_OP_RVC_ADDIW:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ADDW:
.CKB_VM_ASM_LABEL_OP_RVC_ADDW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  addq REGISTER_ADDRESS(RS2r), RS1
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_AND:
.CKB_VM_ASM_LABEL_OP_RVC_AND:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  andq REGISTER_ADDRESS(RS2r), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ANDI:
.CKB_VM_ASM_LABEL_OP_RVC_ANDI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  andq IMMEDIATE, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_AUIPC:
  DECODE_U
  movq PC_ADDRESS, RS1
  subq $4, RS1
  addq IMMEDIATE, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BEQ:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2s), RS2s
  cmpq RS2s, RS1
  je .i_branch_success
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BGE:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2s), RS2s
  cmpq RS2s, RS1
  jge .i_branch_success
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BGEU:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2s), RS2s
  cmpq RS2s, RS1
  jae .i_branch_success
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BLT:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2s), RS2s
  cmpq RS2s, RS1
  jl .i_branch_success
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BLTU:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2s), RS2s
  cmpq RS2s, RS1
  jb .i_branch_success
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BNE:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2s), RS2s
  cmpq RS2s, RS1
  jne .i_branch_success
  NEXT_INST
.i_branch_success:
  movq PC_ADDRESS, RS1
  subq $4, RS1
  addq IMMEDIATE, RS1
  movq RS1, PC_ADDRESS
  jmp .prepare_trace
.p2align 3
.CKB_VM_ASM_LABEL_OP_DIV:
  DECODE_R
  push RD
  movq $INT64_MIN, RD
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmp RD, RS1
  jne .div_branch1
  cmp $-1, RS2r
  jne .div_branch1
  jmp .div_branch3
.div_branch1:
  cmp $0, RS2r
  jne .div_branch2
  movq $UINT64_MAX, RS1
  jmp .div_branch3
.div_branch2:
  MOV_RS1_TO_RAX
  cqo
  idivq RS2r
  MOV_RAX_TO_RS1
.div_branch3:
  pop RD
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_DIVU:
  DECODE_R
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmp $0, RS2r
  jne .divu_branch2
  WRITE_RD_VALUE($UINT64_MAX, RS2r)
  NEXT_INST
.divu_branch2:
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  movq REGISTER_ADDRESS(RS1), %rax
  xorq %rdx, %rdx
  divq RS2r
  mov %rax, RS2r
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  WRITE_RD(RS2r)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_DIVUW:
  DECODE_R
  movq REGISTER_ADDRESS(RS2r), RS2r
  mov RS2rd, RS2rd
  cmp $0, RS2r
  jne .divuw_branch2
  WRITE_RD_VALUE($UINT64_MAX, RS2r)
  NEXT_INST
.divuw_branch2:
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  movq REGISTER_ADDRESS(RS1), %rax
  mov %eax, %eax
  xorq %rdx, %rdx
  divq RS2r
  mov %rax, RS2r
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  movslq RS2rd, RS2r
  WRITE_RD(RS2r)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_DIVW:
  DECODE_R
  push RD
  movq $INT64_MIN, RD
  movq REGISTER_ADDRESS(RS1), RS1
  movslq RS1d, RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  movslq RS2rd, RS2r
  cmp RD, RS1
  jne .divw_branch1
  cmp $-1, RS2r
  jne .divw_branch1
  jmp .divw_branch3
.divw_branch1:
  cmp $0, RS2r
  jne .divw_branch2
  movq $UINT64_MAX, RS1
  jmp .divw_branch3
.divw_branch2:
  MOV_RS1_TO_RAX
  cqo
  idivq RS2r
  MOV_RAX_TO_RS1
.divw_branch3:
  pop RD
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_EBREAK:
  DECODE_U
  mov $CKB_VM_ASM_RET_EBREAK, ARG_RETd
  jmp .exit
.p2align 3
.CKB_VM_ASM_LABEL_OP_ECALL:
  DECODE_U
  mov $CKB_VM_ASM_RET_ECALL, ARG_RETd
  jmp .exit
.p2align 3
.CKB_VM_ASM_LABEL_OP_FENCE:
.CKB_VM_ASM_LABEL_OP_FENCEI:
.CKB_VM_ASM_LABEL_OP_RVC_NOP:
.CKB_VM_ASM_LABEL_OP_RVC_SLLI64:
.CKB_VM_ASM_LABEL_OP_RVC_SRAI64:
.CKB_VM_ASM_LABEL_OP_RVC_SRLI64:
  DECODE_U
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_JAL:
  DECODE_U
  movq PC_ADDRESS, RS1
  WRITE_RD(RS1)
  subq $4, RS1
  addq IMMEDIATE, RS1
  movq RS1, PC_ADDRESS
  jmp .prepare_trace
.p2align 3
.CKB_VM_ASM_LABEL_OP_JALR:
  DECODE_I
  movq PC_ADDRESS, TEMP1
  WRITE_RD(TEMP1)
  movq REGISTER_ADDRESS(RS1), TEMP1
  addq IMMEDIATE, TEMP1
  andq $-2, TEMP1
  movq TEMP1, PC_ADDRESS
  jmp .prepare_trace
.p2align 3
.CKB_VM_ASM_LABEL_OP_LB:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  movsbq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LBU:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  movzbq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LD:
.CKB_VM_ASM_LABEL_OP_RVC_LD:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $8, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  movq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LH:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $2, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  movswq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LHU:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $2, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  movzwq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LUI:
.CKB_VM_ASM_LABEL_OP_RVC_LI:
.CKB_VM_ASM_LABEL_OP_RVC_LUI:
.CKB_VM_ASM_LABEL_OP_CUSTOM_LOAD_IMM:
  DECODE_U
  WRITE_RD(IMMEDIATE)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LW:
.CKB_VM_ASM_LABEL_OP_RVC_LW:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $4, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  movslq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LWU:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $4, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  mov CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1d
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_MUL:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  imul REGISTER_ADDRESS(RS2r), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_MULH:
  DECODE_R
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  movq REGISTER_ADDRESS(RS1), %rax
  imulq REGISTER_ADDRESS(RS2r)
  MOV_RDX_TO_RS2r
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  WRITE_RD(RS2r)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_MULHSU:
  DECODE_R
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  PUSH_RS1_IF_RAX
  PUSH_RS1_IF_RDX
  movq REGISTER_ADDRESS(RS1), %rax
  test %rax, %rax
  jns .mulhsu_branch1
  neg %rax
  mulq REGISTER_ADDRESS(RS2r)
  xor $-1, %rdx
  movq %rdx, TEMP1
  POP_RS1_IF_RDX
  POP_RS1_IF_RAX
  movq REGISTER_ADDRESS(RS1), %rax
  imulq REGISTER_ADDRESS(RS2r)
  test %rax, %rax
  setz %al
  movzbl %al, %eax
  addq %rax, TEMP1
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  WRITE_RD(TEMP1)
  NEXT_INST
.mulhsu_branch1:
  mulq REGISTER_ADDRESS(RS2r)
  movq %rdx, TEMP1
  POP_RS1_IF_RDX
  POP_RS1_IF_RAX
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_MULHU:
  DECODE_R
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  movq REGISTER_ADDRESS(RS1), %rax
  mulq REGISTER_ADDRESS(RS2r)
  movq %rdx, RS2r
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  WRITE_RD(RS2r)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_MULW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  imul RS2rd, RS1d
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_OR:
.CKB_VM_ASM_LABEL_OP_RVC_OR:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  orq REGISTER_ADDRESS(RS2r), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ORI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  orq IMMEDIATE, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_REM:
  DECODE_R
  push RD
  movq $INT64_MIN, RD
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmp RD, RS1
  jne .rem_branch1
  cmp $-1, RS2r
  jne .rem_branch1
  xorq RS1, RS1
  jmp .rem_branch3
.rem_branch1:
  cmp $0, RS2r
  jne .rem_branch2
  jmp .rem_branch3
.rem_branch2:
  MOV_RS1_TO_RAX
  cqo
  idivq RS2r
  MOV_RDX_TO_RS1
.rem_branch3:
  pop RD
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_REMU:
  DECODE_R
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmp $0, RS2r
  jne .remu_branch2
  movq REGISTER_ADDRESS(RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.remu_branch2:
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  movq REGISTER_ADDRESS(RS1), %rax
  xorq %rdx, %rdx
  divq RS2r
  mov %rdx, RS2r
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  WRITE_RD(RS2r)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_REMUW:
  DECODE_R
  movq REGISTER_ADDRESS(RS2r), RS2r
  mov RS2rd, RS2rd
  cmp $0, RS2r
  jne .remuw_branch2
  movq REGISTER_ADDRESS(RS1), RS1
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.remuw_branch2:
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  movq REGISTER_ADDRESS(RS1), %rax
  mov %eax, %eax
  xorq %rdx, %rdx
  divq RS2r
  mov %rdx, RS2r
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  movslq RS2rd, RS2r
  WRITE_RD(RS2r)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_REMW:
  DECODE_R
  push RD
  movq $INT64_MIN, RD
  movq REGISTER_ADDRESS(RS1), RS1
  movslq RS1d, RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  movslq RS2rd, RS2r
  cmp RD, RS1
  jne .remw_branch1
  cmp $-1, RS2r
  jne .remw_branch1
  xorq RS1, RS1
  jmp .remw_branch3
.remw_branch1:
  cmp $0, RS2r
  jne .remw_branch2
  jmp .remw_branch3
.remw_branch2:
  MOV_RS1_TO_RAX
  cqo
  idivq RS2r
  MOV_RDX_TO_RS1
.remw_branch3:
  pop RD
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SB:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  CHECK_WRITE_PERMISSION(RS1, TEMP1, RS2r, 1)
  movq REGISTER_ADDRESS(RS2s), RS2s
  mov RS2sb, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SD:
.CKB_VM_ASM_LABEL_OP_RVC_SD:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  CHECK_WRITE_PERMISSION(RS1, TEMP1, RS2r, 8)
  movq REGISTER_ADDRESS(RS2s), RS2s
  movq RS2s, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SH:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  CHECK_WRITE_PERMISSION(RS1, TEMP1, RS2r, 2)
  movq REGISTER_ADDRESS(RS2s), RS2s
  mov RS2sh, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLL:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq REGISTER_ADDRESS(RS2r), %rcx
  shl %cl, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLLI:
.CKB_VM_ASM_LABEL_OP_RVC_SLLI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq IMMEDIATE, %rcx
  shl %cl, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLLIW:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq IMMEDIATE, %rcx
  shl %cl, TEMP1
  movslq TEMP1d, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLLW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq REGISTER_ADDRESS(RS2r), %rcx
  and $0x1F, %ecx
  shl %cl, TEMP1
  movslq TEMP1d, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLT:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmpq RS2r, RS1
  setl RS1b
  movzbl RS1b, RS1d
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLTI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  cmpq IMMEDIATE, RS1
  setl RS1b
  movzbl RS1b, RS1d
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLTIU:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  cmpq IMMEDIATE, RS1
  setb RS1b
  movzbl RS1b, RS1d
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLTU:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmpq RS2r, RS1
  setb RS1b
  movzbl RS1b, RS1d
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SRA:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq REGISTER_ADDRESS(RS2r), %rcx
  sar %cl, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SRAI:
.CKB_VM_ASM_LABEL_OP_RVC_SRAI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq IMMEDIATE, %rcx
  sar %cl, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SRAIW:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq IMMEDIATE, %rcx
  sar %cl, TEMP1d
  movslq TEMP1d, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SRAW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq REGISTER_ADDRESS(RS2r), %rcx
  and $0x1F, %ecx
  sar %cl, TEMP1d
  movslq TEMP1d, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SRL:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq REGISTER_ADDRESS(RS2r), %rcx
  shr %cl, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SRLI:
.CKB_VM_ASM_LABEL_OP_RVC_SRLI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq IMMEDIATE, %rcx
  shr %cl, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SRLIW:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq IMMEDIATE, %rcx
  shr %cl, TEMP1d
  movslq TEMP1d, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SRLW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq REGISTER_ADDRESS(RS2r), %rcx
  and $0x1F, %ecx
  shr %cl, TEMP1d
  movslq TEMP1d, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SUB:
.CKB_VM_ASM_LABEL_OP_RVC_SUB:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  subq REGISTER_ADDRESS(RS2r), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SUBW:
.CKB_VM_ASM_LABEL_OP_RVC_SUBW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  subq REGISTER_ADDRESS(RS2r), RS1
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SW:
.CKB_VM_ASM_LABEL_OP_RVC_SW:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  CHECK_WRITE_PERMISSION(RS1, TEMP1, RS2r, 4)
  movq REGISTER_ADDRESS(RS2s), RS2s
  mov RS2sd, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_XOR:
.CKB_VM_ASM_LABEL_OP_RVC_XOR:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  xorq REGISTER_ADDRESS(RS2r), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_XORI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  xorq IMMEDIATE, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_ADDI16SP:
  DECODE_I
  addq IMMEDIATE, SP_ADDRESS
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_ADDI4SPN:
  DECODE_U
  movq SP_ADDRESS, RS1
  addq IMMEDIATE, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_BEQZ:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  cmpq $0, RS1
  je .rvc_branch_success
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_BNEZ:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  cmpq $0, RS1
  jne .rvc_branch_success
  NEXT_INST
.rvc_branch_success:
  movq PC_ADDRESS, RS1
  subq $2, RS1
  addq IMMEDIATE, RS1
  movq RS1, PC_ADDRESS
  jmp .prepare_trace
.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_EBREAK:
  DECODE_U
  mov $CKB_VM_ASM_RET_EBREAK, ARG_RETd
  jmp .exit
.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_J:
  DECODE_U
  movq PC_ADDRESS, RS1
  subq $2, RS1
  addq IMMEDIATE, RS1
  movq RS1, PC_ADDRESS
  jmp .prepare_trace
.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_JAL:
  DECODE_U
  movq PC_ADDRESS, RS1
  movq RS1, (CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS + 1 * 8)(MACHINE)
  subq $2, RS1
  addq IMMEDIATE, RS1
  movq RS1, PC_ADDRESS
  jmp .prepare_trace
.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_JALR:
  DECODE_S
  movq PC_ADDRESS, %rcx
  movq %rcx, (CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS + 1 * 8)(MACHINE)
  movq REGISTER_ADDRESS(RS1), %rcx
  andq $-2, %rcx
  movq %rcx, PC_ADDRESS
  jmp .prepare_trace
.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_JR:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), %rcx
  andq $-2, %rcx
  movq %rcx, PC_ADDRESS
  jmp .prepare_trace
.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_LDSP:
  DECODE_U
  movq SP_ADDRESS, RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $8, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  movq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_LWSP:
  DECODE_U
  movq SP_ADDRESS, RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $4, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  movslq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_MV:
  DECODE_R
  movq REGISTER_ADDRESS(RS2r), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_SDSP:
  DECODE_S
  movq SP_ADDRESS, RS1
  addq IMMEDIATE, RS1
  CHECK_WRITE_PERMISSION(RS1, TEMP1, RS2r, 8)
  movq REGISTER_ADDRESS(RS2s), RS2s
  movq RS2s, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_SWSP:
  DECODE_S
  movq SP_ADDRESS, RS1
  addq IMMEDIATE, RS1
  CHECK_WRITE_PERMISSION(RS1, TEMP1, RS2r, 4)
  movq REGISTER_ADDRESS(RS2s), RS2s
  mov RS2sd, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1)
  NEXT_INST
/* RS2r is used as a temporary register here */
.CKB_VM_ASM_LABEL_OP_VERSION1_JALR:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS2r
  movq PC_ADDRESS, TEMP1
  WRITE_RD(TEMP1)
  addq IMMEDIATE, RS2r
  andq $-2, RS2r
  movq RS2r, PC_ADDRESS
  jmp .prepare_trace
.p2align 3
.CKB_VM_ASM_LABEL_OP_VERSION1_RVC_JALR:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq PC_ADDRESS, %rcx
  movq %rcx, (CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS + 1 * 8)(MACHINE)
  andq $-2, TEMP1
  movq TEMP1, PC_ADDRESS
  jmp .prepare_trace
.p2align 3
.CKB_VM_ASM_LABEL_OP_VERSION1_LB:
  DECODE_I
  CHECK_READ_BOUND_VERSION1($1)
  movsbq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_VERSION1_LBU:
  DECODE_I
  CHECK_READ_BOUND_VERSION1($1)
  movzbq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_VERSION1_LD:
.CKB_VM_ASM_LABEL_OP_VERSION1_RVC_LD:
  DECODE_I
  CHECK_READ_BOUND_VERSION1($8)
  movq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_VERSION1_LH:
  DECODE_I
  CHECK_READ_BOUND_VERSION1($2)
  movswq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_VERSION1_LHU:
  DECODE_I
  CHECK_READ_BOUND_VERSION1($2)
  movzwq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_VERSION1_LW:
.CKB_VM_ASM_LABEL_OP_VERSION1_RVC_LW:
  DECODE_I
  CHECK_READ_BOUND_VERSION1($4)
  movslq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_VERSION1_LWU:
  DECODE_I
  CHECK_READ_BOUND_VERSION1($4)
  mov CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1d
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_VERSION1_RVC_LDSP:
  DECODE_U
  movq SP_ADDRESS, RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $8, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  ja .exit_out_of_bound
  movq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_VERSION1_RVC_LWSP:
  DECODE_U
  movq SP_ADDRESS, RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $4, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  ja .exit_out_of_bound
  movslq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.exit_out_of_bound:
  mov $CKB_VM_ASM_RET_OUT_OF_BOUND, ARG_RETd
  jmp .exit
.p2align 3
.exit_max_cycles_exceeded:
  mov $CKB_VM_ASM_RET_MAX_CYCLES_EXCEEDED, ARG_RETd
  jmp .exit
.exit_invalid_permission:
  mov $CKB_VM_ASM_RET_INVALID_PERMISSION, ARG_RETd
  jmp .exit
.p2align 3
.exit_trace:
.CKB_VM_ASM_LABEL_OP_UNLOADED:
  DECODE_U
  mov $CKB_VM_ASM_RET_DECODE_TRACE, ARG_RETd
  jmp .exit
.exit:
  retq
