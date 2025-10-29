enum AddressingMode {
    A,    // Use accumulator.
    Abs,  // Use data at address.
    AbsX, // Use data at (address + X).
    AbsY, // Use data at (address + Y).
    Imm,  // Use byte as data.
    Impl, // No operand.
    Ind,  // Use data at address as new address.
    XInd, // Use data at (address + x) as new address.
    IndY, // Use (data + y) at address as new address.
    Rel,  // Use data at (program counter + byte).
    Zpg,  // Operand is low byte of zero-page address
    ZpgX, // Operand is low byte of zero-page address incremented by X
    ZpgY, // Operand is low byte of zero-page address incremented by Y
}

enum Instruction {
    ADC, //
    AND, //
    ASL, //
    BCC, //
    BCS, //
    BEQ, //
    BIT, //
    BMI, //
    BNE, //
    BPL, //
    BRK, //
    BVC, //
    BVS, //
    CLC, //
    CLD, //
    CLI, //
    CLV, //
    CMP, //
    CPX, //
    CPY, //
    DEC, //
    DEX, //
    DEY, //
    EOR, //
    INC, //
    INX, //
    INY, //
    JMP, //
    JSR, //
    LDA, //
    LDX, //
    LDY, //
    LSR, //
    NOP, //
    ORA, //
    PHA, //
    PHP, //
    PLA, //
    PLP, //
    ROL, //
    ROR, //
    RTI, //
    RTS, //
    SBC, //
    SEC, //
    SED, //
    SEI, //
    STA, //
    STX, //
    STY, //
    TAX, //
    TAY, //
    TSX, //
    TXA, //
    TXS, //
    TYA, //

    // Illegal opcodes below
    ALR, //
    ANC, //
    AN2, //
    ANE, //
    ARR, //
    DCP, //
    ISC, //
    LAS, //
    LAX, //
    LXA, //
    RLA, //
    RRA, //
    SAX, //
    SBX, //
    SHA, //
    SHX, //
    SHY, //
    SLO, //
    SRE, //
    TAS, //
    USB, //
    JAM, //
}

pub fn get_instruction(opcode: u8) -> (Instruction, AddressingMode) {
    match opcode {
        0x00 => (Instruction::BRK, AddressingMode::Impl),
        0x10 => (Instruction::BPL, AddressingMode::Rel),
        0x20 => (Instruction::JSR, AddressingMode::Abs),
        0x30 => (Instruction::BMI, AddressingMode::Rel),
        0x40 => (Instruction::RTI, AddressingMode::Impl),
        0x50 => (Instruction::BVC, AddressingMode::Rel),
        0x60 => (Instruction::RTS, AddressingMode::Impl),
        0x70 => (Instruction::BVS, AddressingMode::Rel),
        0x80 => (Instruction::NOP, AddressingMode::Imm),
        0x90 => (Instruction::BCC, AddressingMode::Rel),
        0xA0 => (Instruction::LDY, AddressingMode::Imm),
        0xB0 => (Instruction::BCS, AddressingMode::Rel),
        0xC0 => (Instruction::CPY, AddressingMode::Imm),
        0xD0 => (Instruction::BNE, AddressingMode::Rel),
        0xE0 => (Instruction::CPX, AddressingMode::Imm),
        0xF0 => (Instruction::BEQ, AddressingMode::Rel),

        0x01 => (Instruction::ORA, AddressingMode::XInd),
        0x11 => (Instruction::ORA, AddressingMode::IndY),
        0x21 => (Instruction::AND, AddressingMode::XInd),
        0x31 => (Instruction::AND, AddressingMode::IndY),
        0x41 => (Instruction::EOR, AddressingMode::XInd),
        0x51 => (Instruction::EOR, AddressingMode::IndY),
        0x61 => (Instruction::ADC, AddressingMode::XInd),
        0x71 => (Instruction::ADC, AddressingMode::IndY),
        0x81 => (Instruction::STA, AddressingMode::XInd),
        0x91 => (Instruction::STA, AddressingMode::IndY),
        0xA1 => (Instruction::LDA, AddressingMode::XInd),
        0xB1 => (Instruction::LDA, AddressingMode::IndY),
        0xC1 => (Instruction::CMP, AddressingMode::XInd),
        0xD1 => (Instruction::CMP, AddressingMode::IndY),
        0xE1 => (Instruction::SBC, AddressingMode::XInd),
        0xF1 => (Instruction::SBC, AddressingMode::IndY),

        0x02 => (Instruction::JAM, AddressingMode::Impl),
        0x12 => (Instruction::JAM, AddressingMode::Impl),
        0x22 => (Instruction::JAM, AddressingMode::Impl),
        0x32 => (Instruction::JAM, AddressingMode::Impl),
        0x42 => (Instruction::JAM, AddressingMode::Impl),
        0x52 => (Instruction::JAM, AddressingMode::Impl),
        0x62 => (Instruction::JAM, AddressingMode::Impl),
        0x72 => (Instruction::JAM, AddressingMode::Impl),
        0x82 => (Instruction::NOP, AddressingMode::Imm),
        0x92 => (Instruction::JAM, AddressingMode::Impl),
        0xA2 => (Instruction::LDX, AddressingMode::Imm),
        0xB2 => (Instruction::JAM, AddressingMode::Impl),
        0xC2 => (Instruction::NOP, AddressingMode::Imm),
        0xD2 => (Instruction::JAM, AddressingMode::Impl),
        0xE2 => (Instruction::NOP, AddressingMode::Imm),
        0xF2 => (Instruction::JAM, AddressingMode::Impl),

        0x03 => (Instruction::SLO, AddressingMode::XInd),
        0x13 => (Instruction::SLO, AddressingMode::IndY),
        0x23 => (Instruction::RLA, AddressingMode::XInd),
        0x33 => (Instruction::RLA, AddressingMode::IndY),
        0x43 => (Instruction::SRE, AddressingMode::XInd),
        0x53 => (Instruction::SRE, AddressingMode::IndY),
        0x63 => (Instruction::RRA, AddressingMode::XInd),
        0x73 => (Instruction::RRA, AddressingMode::IndY),
        0x83 => (Instruction::SAX, AddressingMode::XInd),
        0x93 => (Instruction::SHA, AddressingMode::IndY),
        0xA3 => (Instruction::LAX, AddressingMode::XInd),
        0xB3 => (Instruction::LAX, AddressingMode::IndY),
        0xC3 => (Instruction::DCP, AddressingMode::XInd),
        0xD3 => (Instruction::DCP, AddressingMode::IndY),
        0xE3 => (Instruction::ISC, AddressingMode::XInd),
        0xF3 => (Instruction::ISC, AddressingMode::IndY),
        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),

        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),

        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),

        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),

        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),

        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),

        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),

        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),

        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),

        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),

        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),

        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),
    }
}
