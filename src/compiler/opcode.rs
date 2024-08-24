#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    SETI,   // rd, 32bit imm ;set reg Value as 32bit imm
    SETF,   // rd, 32bit imm ;set reg Value as 32bit float imm
    SETS,   // rd, string    ;set String to str_table, return ptr to rd, will save space for str
    SETNIL, // rd            ;set reg Value as NIL
    STORE, // rd, r1, 32bit imm ; store r1 to table based on imm, return rd idx, will not save space for str
    LOAD,  // rd, r1, 32bit imm ; load r1 idx from table based on imm, return rd
    // imm=
    // 0: INT_TABLE
    // 1: FLOAT_TABLE
    // 2: STR_TABLE
    ADD,     // rd, r1, r2
    SUB,     // rd, r1, r2
    MUL,     // rd, r1, r2
    DIV,     // rd, r1, r2
    CVT_I_D, // frd, r1
    CVT_D_I, // rd, fr1
    NEGATE,  // rd, r1
    HLT,

    JMP,    // 32bit imm | @label
    JMP_IF, // r1, 32bit imm | @label

    EQ,  // rd, r1, r2
    NEQ, // rd, r1, r2
    GT,  // rd, r1, r2
    GTE, // rd, r1, r2
    LT,  // rd, r1, r2
    LTE, // rd, r1, r2

    BITAND,  // rd, r1, r2
    BITOR,   // rd, r1, r2
    BITXOR,  // rd, r1, r2
    BITNOT,  // rd, r1
    BITSHL,  // rd, r1, imm
    BITSHRL, // rd, r1, imm
    BITSHRA, // rd, r1, imm

    VMCALL, // r1, r2, imm
    // imm = 0 : print r1 value, if r2 == int && r2 == 1, print "\n"
    // imm = 1 : get int32 to r1
    // imm = 2 : get float to r1
    // imm = 3 : get string from input, save ptr to r1

    PUSH, // r1
    POP,  // rd

    GET_LEN, // rd, r1

    SET_ARG, // r1, imm
    GET_ARG, // rd, imm
    CALL,    // rd | @label
    RETURN,  // rd

    NEW_LIST, // rd
    SET_LIST, // rd, r1, r2
    GET_LIST, // rd, r1, r2

    NEW_TABLE, // rd
    SET_TABLE, // rd, r1, r2
    GET_TABLE, // rd, r1, r2

    NEW_ARRAY, // rd
    SET_ARRAY, // rd, r1, r2
    GET_ARRAY, // rd, r1, r2

    IGL,
    NOP,
}

pub fn opcode_to_bytes(op: Opcode) -> [u8; 4] {
  let value = op as u32;
  
  if cfg!(target_endian = "big") {
      [
          ((value >> 24) & 0xFF) as u8,
          ((value >> 16) & 0xFF) as u8,
          ((value >> 8) & 0xFF) as u8,
          (value & 0xFF) as u8,
      ]
  } else {
      [
          ((value >> 24) & 0xFF) as u8,
          ((value >> 16) & 0xFF) as u8,
          ((value >> 8) & 0xFF) as u8,
          (value & 0xFF) as u8,
      ]
  }
}