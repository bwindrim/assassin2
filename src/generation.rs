use crate::representation::*;

fn encode_indexed_indirect(operand: &IndexedIndirect) -> Vec<u8> {

    fn reg_field(reg: &IndexRegister) -> u8 {
        match reg {
            IndexRegister::X => 0 << 5,
            IndexRegister::Y => 1 << 5,
            IndexRegister::U => 2 << 5,
            IndexRegister::S => 3 << 5,
        }
    }

    match operand {
        IndexedIndirect::Const { offset, reg } => {
            match offset {
                0      => vec![0x80 | reg_field(reg)], // no offset
                1..=7  => vec![0x00 | reg_field(reg) | *offset as u8], // 5-bit offset
                8..=15 => vec![0x88 | reg_field(reg), *offset as u8], // 8-bit offset
                _      => vec![0x89 | reg_field(reg), (*offset >> 8) as u8, *offset as u8], // 16-bit offset
            }
        },
        IndexedIndirect::ConstInd { offset, reg } => {
            match offset {
                0      => vec![0x94 | reg_field(reg)], // no offset
                8..=15 => vec![0x98 | reg_field(reg), *offset as u8], // 8-bit offset
                _      => vec![0x99 | reg_field(reg), (*offset >> 8) as u8, *offset as u8], // 16-bit offset
            }
        },
        IndexedIndirect::Acc { offset, reg } => {
            match offset {
                AccOffsetRegister::A => vec![0x86 | reg_field(reg)], // A offset
                AccOffsetRegister::B => vec![0x85 | reg_field(reg)], // B offset
                AccOffsetRegister::D => vec![0x8B | reg_field(reg)], // D offset
            }
        },
        IndexedIndirect::AccInd { offset, reg } => {
            match offset {
                AccOffsetRegister::A => vec![0x96 | reg_field(reg)], // A offset indirect
                AccOffsetRegister::B => vec![0x95 | reg_field(reg)], // B offset indirect
                AccOffsetRegister::D => vec![0x9B | reg_field(reg)], // D offset indirect
            }
        },
        IndexedIndirect::Inc1 { reg }     => vec![reg_field(reg) | 0x80], // Increment by 1
        IndexedIndirect::Inc2 { reg }     => vec![reg_field(reg) | 0x81], // Increment by 2
        IndexedIndirect::Inc2Ind { reg } => vec![reg_field(reg) | 0x91], // Increment by 2, indirect
        IndexedIndirect::Dec1 { reg }     => vec![reg_field(reg) | 0x82], // Decrement by 1
        IndexedIndirect::Dec2 { reg }     => vec![reg_field(reg) | 0x83], // Decrement by 2
        IndexedIndirect::Dec2Ind { reg } => vec![reg_field(reg) | 0x93], // Decrement by 2, indirect
        IndexedIndirect::Pcr { target } => 
            if *target as i16 >= -128 && *target as i16 <= 127 {
                vec![0x8C, *target as u8] // PC-relative with 8-bit offset
            } else {
                vec![0x8D, (*target >> 8) as u8, *target as u8] // PC-relative with 16-bit offset
            },
        IndexedIndirect::PcrInd { target } => 
            if *target as i16 >= -128 && *target as i16 <= 127 {
                vec![0x9C, *target as u8] // PC-relative with 8-bit offset
            } else {
                vec![0x9D, (*target >> 8) as u8, *target as u8] // PC-relative with 16-bit offset
            },
        IndexedIndirect::Pc { offset } => 
            if *offset as i16 >= -128 && *offset as i16 <= 127 {
                vec![0x8C, *offset as u8] // PC-relative with 8-bit offset
            } else {
                vec![0x8D, (*offset >> 8) as u8, *offset as u8] // PC-relative with 16-bit offset
            },
        IndexedIndirect::PcInd { offset } => 
            if *offset as i16 >= -128 && *offset as i16 <= 127 {
                vec![0x9C, *offset as u8] // PC-relative with 8-bit offset
            } else {
                vec![0x9D, (*offset >> 8) as u8, *offset as u8] // PC-relative with 16-bit offset
            },
        IndexedIndirect::ExtInd(addr) => vec![0x9F, (*addr >> 8) as u8, *addr as u8], // Extended indirect
    }
}

fn encode_type0(opcode: u16) -> Vec<u8> {
    if opcode > 0xFF {
        vec![(opcode >> 8) as u8, opcode as u8]
    } else {
        vec![opcode as u8]
    }
}

fn encode_type1<T: IntoBytes + Copy>(opcode: u16, operand: &Type1<T>) -> Vec<u8> {

    fn encode_type1_opcode<T: IntoBytes + Copy>(opcode: u16, operand: &Type1<T>) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        if opcode > 0xFF {
            bytes.push((opcode >> 8) as u8);
        }
        bytes.extend(match operand {
            Type1::IMM(_) => vec![(opcode as u8) | 0x00],
            Type1::DIR(_) => vec![(opcode as u8) | 0x10],
            Type1::EXT(_) => vec![(opcode as u8) | 0x30],
            Type1::IND(_) => vec![(opcode as u8) | 0x20],
        });
        bytes
    }

    fn encode_type1_operand<T: IntoBytes + Copy>(operand: &Type1<T>) -> Vec<u8> {
        match operand {
            Type1::IMM(value) => gen_bytes::<T>(*value),
            Type1::DIR(addr) => vec![*addr],
            Type1::EXT(addr) => vec![(*addr >> 8) as u8, *addr as u8],
            Type1::IND(indirect) => encode_indexed_indirect(indirect),
        }
    }

    let mut bytes = encode_type1_opcode(opcode, operand);
    bytes.extend(encode_type1_operand(operand));
    bytes
}

fn encode_type2(opcode: u16, operand: &Type2) -> Vec<u8> {

    fn encode_type2_opcode(opcode: u16, operand: &Type2) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        if opcode > 0xFF {
            bytes.push((opcode >> 8) as u8);
        }
        bytes.extend(match operand {
            Type2::DIR(_) => vec![(opcode as u8) | 0x00],
            Type2::EXT(_) => vec![(opcode as u8) | 0x70],
            Type2::IND(_) => vec![(opcode as u8) | 0x60],
        });
        bytes
    }

    fn encode_type2_operand(operand: &Type2) -> Vec<u8> {
        match operand {
            Type2::DIR(addr) => vec![*addr],
            Type2::EXT(addr) => vec![(*addr >> 8) as u8, *addr as u8],
            Type2::IND(indirect) => encode_indexed_indirect(indirect),
        }
    }

    let mut bytes = encode_type2_opcode(opcode, operand);
    bytes.extend(encode_type2_operand(operand));
    bytes
}

fn encode_typecc(opcode: u8, operand: &Typecc) -> Vec<u8> {
    vec![opcode, operand.mask]
}

fn encode_typext(opcode: u8, operand: &Typext) -> Vec<u8> {

    fn register_code_8(reg: &TfrExgRegister8) -> u8 {
        match reg {
            TfrExgRegister8::A => 8,
            TfrExgRegister8::B => 9,
            TfrExgRegister8::CC => 10,
            TfrExgRegister8::DP => 11,
        }
    }

    fn register_code_16(reg: &TfrExgRegister16) -> u8 {
        match reg {
            TfrExgRegister16::D => 0,
            TfrExgRegister16::X => 1,
            TfrExgRegister16::Y => 2,
            TfrExgRegister16::U => 3,
            TfrExgRegister16::S => 4,
            TfrExgRegister16::PC => 5,
        }
    }

    vec![opcode,
        match operand {
                Typext::BYTE(r1, r2) => (register_code_8(r1) << 4)  | register_code_8(r2),
                Typext::WORD(r1, r2) => (register_code_16(r1) << 4) | register_code_16(r2)  
            }
    ]
}

fn encode_typepspl(opcode: u8, operand: &Typepspl) -> Vec<u8> {
    let mut mask: u8 = 0;
    for reg in &operand.registers {
        mask |= match reg {
            PushPullRegister::CC => 0x01,
            PushPullRegister::A  => 0x02,
            PushPullRegister::B  => 0x04,
            PushPullRegister::DP => 0x08,
            PushPullRegister::X  => 0x10,
            PushPullRegister::Y  => 0x20,
            PushPullRegister::US => 0x40,
            PushPullRegister::PC => 0x80,
        };
    }
    vec![opcode, mask]
}

pub fn encode_instruction(instr: &Instruction) -> Vec<u8> {
    // This is a placeholder implementation. In a real assembler, this function would
    // need to handle all the different instruction formats and addressing modes.
    match instr {
        Instruction::ABX => encode_type0(0x3A), // Opcode for ABX
        Instruction::ADCA(operand)  => encode_type1(0x89, operand), // Base opcode for ADCA
        Instruction::ADCB(operand)  => encode_type1(0xC9, operand), // Base opcode for ADCB
        Instruction::ADDA(operand)  => encode_type1(0x8B, operand), // Base opcode for ADDA
        Instruction::ADDB(operand)  => encode_type1(0xCB, operand), // Base opcode for ADDB
        Instruction::ADDD(operand) => encode_type1(0xC3, operand), // Base opcode for ADDD
        Instruction::ANDA(operand)  => encode_type1(0x84, operand), // Base opcode for ANDA
        Instruction::ANDB(operand)  => encode_type1(0xC4, operand), // Base opcode for ANDB
        Instruction::ANDCC(operand) => encode_typecc(0x1C, operand), // Opcode for ANDCC
        Instruction::ASL(operand) => encode_type2(0x08, operand), // Base opcode for ASL
        Instruction::ASLA => encode_type0(0x48), // Opcode for ASLA (same as LSLA)
        Instruction::ASLB => encode_type0(0x58), // Opcode for ASLB (same as LSLB)
        Instruction::ASR(operand) => encode_type2(0x07, operand), // Base opcode for ASR
        Instruction::ASRA => encode_type0(0x47), // Opcode for ASRA
        Instruction::ASRB => encode_type0(0x57), // Opcode for ASRB
        Instruction::BITA(operand) => encode_type1(0x85, operand), // Base opcode for BITA
        Instruction::BITB(operand) => encode_type1(0xC5, operand), // Base opcode for BITB
        Instruction::CLC  => encode_type0(0x1CFE), // ANDCC #$FE (clear carry)
        Instruction::CLF  => encode_type0(0x1CBF), // ANDCC #$BF (clear fast interrupt disable)
        Instruction::CLI  => encode_type0(0x1CEF), // ANDCC #$EF (clear interrupt disable)
        Instruction::CLIF => encode_type0(0x1CAF), // ANDCC #$AF (clear interrupt disables)
        Instruction::CLR(operand)  => encode_type2(0x0F, operand), // Opcode for CLR
        Instruction::CLRA => encode_type0(0x4F), // Opcode for CLRA
        Instruction::CLRB => encode_type0(0x5F), // Opcode for CLRB
        Instruction::CLV  => encode_type0(0x1CFD), // ANDCC #$FD (clear overflow)
        Instruction::CMPA(operand)  => encode_type1(0x81,   operand), // Base opcode for CMPA
        Instruction::CMPB(operand)  => encode_type1(0xC1,   operand), // Base opcode for CMPB
        Instruction::CMPD(operand) => encode_type1(0x1083, operand), // Base opcode for CMPD
        Instruction::CMPS(operand) => encode_type1(0x118C, operand), // Base opcode for CMPS
        Instruction::CMPU(operand) => encode_type1(0x1183, operand), // Base opcode for CMPU
        Instruction::CMPX(operand) => encode_type1(0x8C,   operand), // Base opcode for CMPX
        Instruction::CMPY(operand) => encode_type1(0x108C, operand), // Base opcode for CMPY
        Instruction::COM(operand) => encode_type2(0x03, operand), // Base opcode for COM
        Instruction::COMA => encode_type0(0x43), // Opcode for COMA
        Instruction::COMB => encode_type0(0x53), // Opcode for COMB
        Instruction::CWAI(operand) => encode_typecc(0x3C, operand), // Opcode for CWAI
        Instruction::DAA  => encode_type0(0x19), // Opcode for DAA
        Instruction::DEC(operand) => encode_type2(0x0A, operand), // Base opcode for DEC
        Instruction::DECA => encode_type0(0x4A), // Opcode for DECA
        Instruction::DECB => encode_type0(0x5A), // Opcode for DECB
        Instruction::EORA(operand) => encode_type1(0x88, operand), // Base opcode for EORA
        Instruction::EORB(operand) => encode_type1(0xC8, operand), // Base opcode for EORB
        Instruction::EXG(operand) => encode_typext(0x1E, operand), // Opcode for EXG
        Instruction::INC(operand) => encode_type2(0x0C, operand), // Base opcode for INC
        Instruction::INCA => encode_type0(0x4C), // Opcode for INCA
        Instruction::INCB => encode_type0(0x5C), // Opcode for INCB
        Instruction::LDA(operand)  => encode_type1(0x86,   operand), // Base opcode for LDA
        Instruction::LDB(operand)  => encode_type1(0xC6,   operand), // Base opcode for LDB
        Instruction::LDD(operand) => encode_type1(0xCC,   operand), // Base opcode for LDD
        Instruction::LDS(operand) => encode_type1(0x10CE, operand), // Base opcode for LDS
        Instruction::LDU(operand) => encode_type1(0xCE,   operand), // Base opcode for LDU
        Instruction::LDX(operand) => encode_type1(0x8E,   operand), // Base opcode for LDX
        Instruction::LDY(operand) => encode_type1(0x108E, operand), // Base opcode for LDY
        Instruction::LSL(operand) => encode_type2(0x08, operand), // Base opcode for LSL (same as ASL)
        Instruction::LSLA => encode_type0(0x48), // Opcode for LSLA (same as ASLA)
        Instruction::LSLB => encode_type0(0x58), // Opcode for LSLB (same as ASLB)
        Instruction::LSR(operand) => encode_type2(0x04, operand), // Base opcode for LSR
        Instruction::LSRA => encode_type0(0x44), // Opcode for LSRA
        Instruction::LSRB => encode_type0(0x54), // Opcode for LSRB
        Instruction::MUL  => encode_type0(0x3D), // Opcode for MUL
        Instruction::NEG(operand) => encode_type2(0x00, operand), // Base opcode for NEG
        Instruction::NEGA => encode_type0(0x40), // Opcode for NEGA
        Instruction::NEGB => encode_type0(0x50), // Opcode for NEGB
        Instruction::NOP  => encode_type0(0x12), // Opcode for NOP
        Instruction::ORA(operand) => encode_type1(0x8A, operand), // Base opcode for ORA
        Instruction::ORB(operand) => encode_type1(0xCA, operand), // Base opcode for ORB
        Instruction::ORCC(operand) => encode_typecc(0x1A, operand), // Opcode for ORCC
        Instruction::PSHS(operand) => encode_typepspl(0x34, operand), // Opcode for PSHS
        Instruction::PSHU(operand) => encode_typepspl(0x36, operand), // Opcode for PSHU
        Instruction::PULS(operand) => encode_typepspl(0x35, operand), // Opcode for PULS
        Instruction::PULU(operand) => encode_typepspl(0x37, operand), // Opcode for PULU
        Instruction::ROL(operand) => encode_type2(0x09, operand), // Base opcode for ROL
        Instruction::ROLA => encode_type0(0x49), // Opcode for ROLA
        Instruction::ROLB => encode_type0(0x59), // Opcode for ROLB
        Instruction::ROR(operand) => encode_type2(0x06, operand), // Base opcode for ROR
        Instruction::RORA => encode_type0(0x46), // Opcode for RORA
        Instruction::RORB => encode_type0(0x56), // Opcode for RORB
        Instruction::RTI  => encode_type0(0x3B), // Opcode for RTI
        Instruction::RTS  => encode_type0(0x39), // Opcode for RTS
        Instruction::SBCA(operand) => encode_type1(0x82, operand), // Base opcode for SBCA
        Instruction::SBCB(operand) => encode_type1(0xC2, operand), // Base opcode for SBCB
        Instruction::SEC  => encode_type0(0x1A01), // ORCC #$01 (set carry)
        Instruction::SEF  => encode_type0(0x1A40), // ORCC #$40 (set fast interrupt disable)
        Instruction::SEI  => encode_type0(0x1A10), // ORCC #$10 (set interrupt disable)
        Instruction::SEIF => encode_type0(0x1A50), // ORCC #$50 (set both interrupt disables)
        Instruction::SEV  => encode_type0(0x1A02), // ORCC #$02 (set overflow)
        Instruction::STA(operand) => encode_type2(0x97,   operand), // Base opcode for STA
        Instruction::STB(operand) => encode_type2(0xD7,   operand), // Base opcode for STB
        Instruction::STD(operand) => encode_type2(0xDD,   operand), // Base opcode for STD
        Instruction::STS(operand) => encode_type2(0x10DF, operand), // Base opcode for STS
        Instruction::STU(operand) => encode_type2(0xDF,   operand), // Base opcode for STU
        Instruction::STX(operand) => encode_type2(0x9F,   operand), // Base opcode for STX
        Instruction::STY(operand) => encode_type2(0x109F, operand), // Base opcode for STY
        Instruction::SUBA(operand)  => encode_type1(0x80, operand), // Base opcode for SUBA
        Instruction::SUBB(operand)  => encode_type1(0xC0, operand), // Base opcode for SUBB
        Instruction::SUBD(operand) => encode_type1(0x83, operand), // Base opcode for SUBD
        Instruction::SEX  => encode_type0(0x1D), // Opcode for SEX
        Instruction::SWI  => encode_type0(0x3F), // Opcode for SWI
        Instruction::SWI2 => encode_type0(0x103F), // Opcode for SWI2 (two-byte opcode)
        Instruction::SWI3 => encode_type0(0x113F), // Opcode for SWI3 (two-byte opcode)
        Instruction::SYNC => encode_type0(0x13), // Opcode for SYNC
        Instruction::TFR(operand) => encode_typext(0x1F, operand), // Opcode for TFR
        Instruction::TST(operand) => encode_type2(0x0D, operand), // Base opcode for TST
        Instruction::TSTA => encode_type0(0x4D), // Opcode for TSTA
        Instruction::TSTB => encode_type0(0x5D), // Opcode for TSTB

        _ => unimplemented!("*** Instruction not implemented in this example ***"),
    }
}
