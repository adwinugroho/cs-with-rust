fn main() {
    println!("Example of RISC (Reduced Instruction Set Computer)");

    let mut cpu = CPU {
        registers: [0; 32],
        pc: 0,
        _memory: vec![0; 1024], // belum kepake, entar kalo ga mager implement assembly yang laen
    };

    let instr: u32 = 0x002081B3;

    cpu.execute(instr);
}

struct CPU {
    registers: [i32; 32],
    pc: u32,
    _memory: Vec<u8>,
}

const OP_ADD: u32 = 0x33;
const OP_LOAD: u32 = 0x03;
const OP_STORE: u32 = 0x23;
const OP_JUMP: u32 = 0x6F;

impl CPU {
    fn execute(&mut self, instr: u32) {
        let opcode = instr & 0x7F;

        match opcode {
            OP_ADD => {
                let rd = (instr >> 7) & 0x1F;
                let rs1 = (instr >> 15) & 0x1F;
                let rs2 = (instr >> 20) & 0x1F;

                self.registers[rd as usize] =
                    self.registers[rs1 as usize] + self.registers[rs2 as usize];

                println!("ADD R{} = R{} + R{}", rd, rs1, rs2);
            }

            OP_LOAD => {
                println!("Ini LOAD.. TODO");
            }

            OP_STORE => {
                println!("Ini STORE.. TODO");
            }

            OP_JUMP => {
                let offset = (instr >> 12) & 0xFFFFF;
                self.pc = offset;
                println!("JUMP PC = {}", offset);
            }

            _ => {
                println!("Unknown opcode: {:#x}", opcode);
            }
        }
    }
}
