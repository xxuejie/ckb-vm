#define CKB_VM_ASM_RISCV_MAX_MEMORY 4194304
#define CKB_VM_ASM_RISCV_PAGE_SHIFTS 12
#define CKB_VM_ASM_RISCV_PAGE_SIZE 4096
#define CKB_VM_ASM_RISCV_PAGE_MASK 4095
#define CKB_VM_ASM_RISCV_PAGES 1024

#define CKB_VM_ASM_MAXIMUM_TRACE_ADDRESS_LENGTH 64

#define CKB_VM_ASM_RET_DECODE_TRACE 1
#define CKB_VM_ASM_RET_ECALL 2
#define CKB_VM_ASM_RET_EBREAK 3
#define CKB_VM_ASM_RET_DYNAMIC_JUMP 4
#define CKB_VM_ASM_RET_MAX_CYCLES_EXCEEDED 5
#define CKB_VM_ASM_RET_OUT_OF_BOUND 6
#define CKB_VM_ASM_RET_INVALID_PERMISSION 7

#define CKB_VM_ASM_REGISTER_SP 2

#define CKB_VM_ASM_MEMORY_FLAG_FREEZED 1
#define CKB_VM_ASM_MEMORY_FLAG_EXECUTABLE 2
#define CKB_VM_ASM_MEMORY_FLAG_WXORX_BIT 2
#define CKB_VM_ASM_MEMORY_FLAG_WRITABLE 0

#define CKB_VM_ASM_TRACE_STRUCT_SIZE 296
#define CKB_VM_ASM_TRACE_OFFSET_ADDRESS 0
#define CKB_VM_ASM_TRACE_OFFSET_LENGTH 8
#define CKB_VM_ASM_TRACE_OFFSET_CYCLES 16
#define CKB_VM_ASM_TRACE_OFFSET_INSTRUCTIONS 24
#define CKB_VM_ASM_TRACE_OFFSET_THREAD 160

#define CKB_VM_ASM_ASM_CORE_MACHINE_STRUCT_SIZE 6620448
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS 0
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC 256
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_CYCLES 272
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MAX_CYCLES 280
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FLAGS 288
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY 1312
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_TRACES 4195616

#define CKB_VM_ASM_OP_UNLOADED 0
#define CKB_VM_ASM_OP_ADD 1
#define CKB_VM_ASM_OP_ADDI 2
#define CKB_VM_ASM_OP_ADDIW 3
#define CKB_VM_ASM_OP_ADDW 4
#define CKB_VM_ASM_OP_AND 5
#define CKB_VM_ASM_OP_ANDI 6
#define CKB_VM_ASM_OP_AUIPC 7
#define CKB_VM_ASM_OP_BEQ 8
#define CKB_VM_ASM_OP_BGE 9
#define CKB_VM_ASM_OP_BGEU 10
#define CKB_VM_ASM_OP_BLT 11
#define CKB_VM_ASM_OP_BLTU 12
#define CKB_VM_ASM_OP_BNE 13
#define CKB_VM_ASM_OP_DIV 14
#define CKB_VM_ASM_OP_DIVU 15
#define CKB_VM_ASM_OP_DIVUW 16
#define CKB_VM_ASM_OP_DIVW 17
#define CKB_VM_ASM_OP_EBREAK 18
#define CKB_VM_ASM_OP_ECALL 19
#define CKB_VM_ASM_OP_FENCE 20
#define CKB_VM_ASM_OP_FENCEI 21
#define CKB_VM_ASM_OP_JAL 22
#define CKB_VM_ASM_OP_JALR 23
#define CKB_VM_ASM_OP_LB 24
#define CKB_VM_ASM_OP_LBU 25
#define CKB_VM_ASM_OP_LD 26
#define CKB_VM_ASM_OP_LH 27
#define CKB_VM_ASM_OP_LHU 28
#define CKB_VM_ASM_OP_LUI 29
#define CKB_VM_ASM_OP_LW 30
#define CKB_VM_ASM_OP_LWU 31
#define CKB_VM_ASM_OP_MUL 32
#define CKB_VM_ASM_OP_MULH 33
#define CKB_VM_ASM_OP_MULHSU 34
#define CKB_VM_ASM_OP_MULHU 35
#define CKB_VM_ASM_OP_MULW 36
#define CKB_VM_ASM_OP_OR 37
#define CKB_VM_ASM_OP_ORI 38
#define CKB_VM_ASM_OP_REM 39
#define CKB_VM_ASM_OP_REMU 40
#define CKB_VM_ASM_OP_REMUW 41
#define CKB_VM_ASM_OP_REMW 42
#define CKB_VM_ASM_OP_SB 43
#define CKB_VM_ASM_OP_SD 44
#define CKB_VM_ASM_OP_SH 45
#define CKB_VM_ASM_OP_SLL 46
#define CKB_VM_ASM_OP_SLLI 47
#define CKB_VM_ASM_OP_SLLIW 48
#define CKB_VM_ASM_OP_SLLW 49
#define CKB_VM_ASM_OP_SLT 50
#define CKB_VM_ASM_OP_SLTI 51
#define CKB_VM_ASM_OP_SLTIU 52
#define CKB_VM_ASM_OP_SLTU 53
#define CKB_VM_ASM_OP_SRA 54
#define CKB_VM_ASM_OP_SRAI 55
#define CKB_VM_ASM_OP_SRAIW 56
#define CKB_VM_ASM_OP_SRAW 57
#define CKB_VM_ASM_OP_SRL 58
#define CKB_VM_ASM_OP_SRLI 59
#define CKB_VM_ASM_OP_SRLIW 60
#define CKB_VM_ASM_OP_SRLW 61
#define CKB_VM_ASM_OP_SUB 62
#define CKB_VM_ASM_OP_SUBW 63
#define CKB_VM_ASM_OP_SW 64
#define CKB_VM_ASM_OP_XOR 65
#define CKB_VM_ASM_OP_XORI 66
#define CKB_VM_ASM_OP_RVC_ADD 67
#define CKB_VM_ASM_OP_RVC_ADDI 68
#define CKB_VM_ASM_OP_RVC_ADDI16SP 69
#define CKB_VM_ASM_OP_RVC_ADDI4SPN 70
#define CKB_VM_ASM_OP_RVC_ADDIW 71
#define CKB_VM_ASM_OP_RVC_ADDW 72
#define CKB_VM_ASM_OP_RVC_AND 73
#define CKB_VM_ASM_OP_RVC_ANDI 74
#define CKB_VM_ASM_OP_RVC_BEQZ 75
#define CKB_VM_ASM_OP_RVC_BNEZ 76
#define CKB_VM_ASM_OP_RVC_EBREAK 77
#define CKB_VM_ASM_OP_RVC_J 78
#define CKB_VM_ASM_OP_RVC_JAL 79
#define CKB_VM_ASM_OP_RVC_JALR 80
#define CKB_VM_ASM_OP_RVC_JR 81
#define CKB_VM_ASM_OP_RVC_LD 82
#define CKB_VM_ASM_OP_RVC_LDSP 83
#define CKB_VM_ASM_OP_RVC_LI 84
#define CKB_VM_ASM_OP_RVC_LUI 85
#define CKB_VM_ASM_OP_RVC_LW 86
#define CKB_VM_ASM_OP_RVC_LWSP 87
#define CKB_VM_ASM_OP_RVC_MV 88
#define CKB_VM_ASM_OP_RVC_NOP 89
#define CKB_VM_ASM_OP_RVC_OR 90
#define CKB_VM_ASM_OP_RVC_SD 91
#define CKB_VM_ASM_OP_RVC_SDSP 92
#define CKB_VM_ASM_OP_RVC_SLLI 93
#define CKB_VM_ASM_OP_RVC_SLLI64 94
#define CKB_VM_ASM_OP_RVC_SRAI 95
#define CKB_VM_ASM_OP_RVC_SRAI64 96
#define CKB_VM_ASM_OP_RVC_SRLI 97
#define CKB_VM_ASM_OP_RVC_SRLI64 98
#define CKB_VM_ASM_OP_RVC_SUB 99
#define CKB_VM_ASM_OP_RVC_SUBW 100
#define CKB_VM_ASM_OP_RVC_SW 101
#define CKB_VM_ASM_OP_RVC_SWSP 102
#define CKB_VM_ASM_OP_RVC_XOR 103
#define CKB_VM_ASM_OP_CUSTOM_LOAD_IMM 104
#define CKB_VM_ASM_OP_CUSTOM_FUSION_COPY_FROM_SP 105
#define CKB_VM_ASM_OP_CUSTOM_TRACE_END 106

#ifdef CKB_VM_ASM_GENERATE_LABEL_TABLES
#ifdef __APPLE__
.global _ckb_vm_asm_labels
_ckb_vm_asm_labels:
#else
.global ckb_vm_asm_labels
ckb_vm_asm_labels:
#endif
.CKB_VM_ASM_LABEL_TABLE:
	.long	.CKB_VM_ASM_LABEL_OP_UNLOADED - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ADD - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ADDI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ADDIW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ADDW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AND - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ANDI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AUIPC - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BEQ - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BGE - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BGEU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BLT - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BLTU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BNE - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_DIV - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_DIVU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_DIVUW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_DIVW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_EBREAK - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ECALL - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_FENCE - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_FENCEI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_JAL - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_JALR - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LB - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LBU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LD - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LH - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LHU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LUI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LWU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_MUL - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_MULH - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_MULHSU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_MULHU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_MULW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_OR - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ORI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_REM - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_REMU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_REMUW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_REMW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SB - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SD - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SH - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SLL - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SLLI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SLLIW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SLLW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SLT - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SLTI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SLTIU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SLTU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SRA - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SRAI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SRAIW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SRAW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SRL - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SRLI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SRLIW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SRLW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SUB - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SUBW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_XOR - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_XORI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_ADD - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_ADDI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_ADDI16SP - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_ADDI4SPN - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_ADDIW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_ADDW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_AND - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_ANDI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_BEQZ - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_BNEZ - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_EBREAK - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_J - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_JAL - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_JALR - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_JR - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_LD - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_LDSP - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_LI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_LUI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_LW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_LWSP - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_MV - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_NOP - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_OR - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_SD - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_SDSP - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_SLLI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_SLLI64 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_SRAI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_SRAI64 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_SRLI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_SRLI64 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_SUB - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_SUBW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_SW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_SWSP - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RVC_XOR - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_CUSTOM_LOAD_IMM - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_CUSTOM_FUSION_COPY_FROM_SP - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_CUSTOM_TRACE_END - .CKB_VM_ASM_LABEL_TABLE
#endif /* CKB_VM_ASM_GENERATE_LABEL_TABLES */
