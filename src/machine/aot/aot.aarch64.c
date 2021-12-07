#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "dasm_proto.h"
#include "dasm_arm64.h"

#include "cdefinitions_generated.h"

||#if (defined(_WIN32) != WIN)
#error "Wrong DynASM flags used: pass -D WIN to dynasm.lua to generate windows specific file"
#endif

|.arch arm64
|.section code
|.globals lbl_
|.actionlist bf_actions

#include "aot.h"

#define INVALID_AARCH64_REGISTER 29
#define VALID_AARCH64_REGISTER(r) (((r) < INVALID_AARCH64_REGISTER) && ((r) != 18))

#define AOT_TAG_X64_REGISTER AOT_TAG_NATIVE_REGISTER
#define aarch64_register_t native_register_t

aarch64_register_t riscv_reg_to_aarch64_reg(riscv_register_t r)
{
  switch (r) {
    case REGISTER_RA:
      return 8;
    case REGISTER_SP:
      return 9;
    case REGISTER_A0:
      return 10;
    case REGISTER_A1:
      return 11;
    case REGISTER_S0:
      return 12;
    case REGISTER_TEMP1:
      return 13;
    case REGISTER_TEMP2:
      return 14;
    case REGISTER_TEMP3:
      return 15;
    case REGISTER_TEMP4:
      return 16;
    case REGISTER_TEMP5:
      return 17;
    default:
      return INVALID_X64_REGISTER;
  }
}

|.type machine, AsmMachine, x0

|.macro load_imm64, aarch64_reg, imm64
| ldr x..aarch64_reg, =imm64
|.endmacro

|.macro load_imm, aarch64_reg, imm
||if ((context->version == 0) && (imm >> 32) > 0 && ((imm & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
|   load_imm64 aarch64_reg, imm
|   sxtw x..aarch64_reg, w..aarch64_reg
||} else {
|   load_imm64 aarch64_reg, imm
||}
|.endmacro

AotContext* aot_new(uint32_t npc, uint32_t version)
{
  dasm_State** Dst;
  AotContext* context = malloc(sizeof(AotContext));
  context->npc = npc;
  context->version = version;
  dasm_init(&context->d, DASM_MAXSECTION);
  dasm_setupglobal(&context->d, context->labels, lbl__MAX);
  dasm_setup(&context->d, bf_actions);
  dasm_growpc(&context->d, context->npc);
  Dst = &context->d;

  |.macro prepcall
    | stp x0, x1, [sp, -96]!
    | stp x8, x9, [sp, 16]
    | stp x10, x11, [sp, 32]
    | stp x12, x13, [sp, 48]
    | stp x14, x15, [sp, 64]
    | stp x16, x17, [sp, 80]
  |.endmacro
  |.macro postcall
    | ldp x16, x17, [sp, 80]
    | ldp x14, x15, [sp, 64]
    | ldp x12, x13, [sp, 48]
    | ldp x10, x11, [sp, 32]
    | ldp x8, x9, [sp, 16]
    | ldp x0, x1, [sp], 96
  |.endmacro

  |.code
  | ldr x8, machine->registers[REGISTER_RA]
  | ldr x9, machine->registers[REGISTER_SP]
  | ldr x10, machine->registers[REGISTER_A0]
  | ldr x11, machine->registers[REGISTER_A1]
  | ldr x12, machine->registers[REGISTER_S0]
  | br x1
  return context;
}

void aot_finalize(AotContext* context)
{
  dasm_free(&context->d);
  free(context);
}

int aot_link(AotContext* context, size_t *szp)
{
  dasm_State** Dst = &context->d;

  |->exit:
  | str x8, machine->registers[REGISTER_RA]
  | str x9, machine->registers[REGISTER_SP]
  | str x10, machine->registers[REGISTER_A0]
  | str x11, machine->registers[REGISTER_A1]
  | str x12, machine->registers[REGISTER_S0]
  | ret
  return dasm_link(&context->d, szp);
}

int aot_encode(AotContext* context, void *buffer)
{
  return dasm_encode(&context->d, buffer);
}

int aot_getpclabel(AotContext* context, uint32_t label, uint32_t* offset)
{
  int ret;
  if (label >= context->npc) {
    return ERROR_NOT_ENOUGH_LABELS;
  }
  ret = dasm_getpclabel(&context->d, label);
  if (ret < 0) { return ret; }
  *offset = (uint32_t) ret;
  return DASM_S_OK;
}

int aot_label(AotContext* context, uint32_t label)
{
  dasm_State** Dst = &context->d;
  if (label >= context->npc) {
    return ERROR_NOT_ENOUGH_LABELS;
  }
  |=>label:
  return DASM_S_OK;
}

int aot_add(AotContext* context, riscv_register_t target, AotValue a, AotValue b)
{
  
}
