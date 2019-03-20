struct Cpu {
    dar: [u32; 8], // 32-bit data registers and address registers
    pc :u32, // 32-bit program counter
    ccr: u8, // 8-bit condition code register

    fp: [f32; 8], // floating-point data registers
    fpcr: u16, // floating-point control register
    fpsr: u32, // floating-point status register
    fpiar: u32, // floating-point instruction address register
}