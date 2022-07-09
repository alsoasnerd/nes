use crate::components::cpu::AddressingMode;
use crate::components::cpu::CPU;
use crate::components::assembly::Assembler;

pub fn trace(cpu: &CPU) -> String {
    let assembler = Assembler::new();

    let code = cpu.memory_read(cpu.register_pc);
    let opcode = assembler.opcodes.get(&code).unwrap();

    let old_pc = cpu.register_pc;
    let mut hex_dump = Vec::new();

    hex_dump.push(code);

    let (memory_address, memory_value) = match opcode.mode {
        AddressingMode::Immediate | AddressingMode::NoneAddressing => (0, 0),

        _ => {
            let address = cpu.get_absolute_address(&opcode.mode, old_pc + 1);
            (address, cpu.memory_read(address))
        }
    };

    let info = match opcode.len {
        1 => match opcode.code {
            0x0a | 0x4a | 0x2a | 0x6a => format!("A "),
            _ => String::from(""),
        },

        2 => {
            let address = cpu.memory_read(old_pc + 1);
            hex_dump.push(address);

            match opcode.mode {
                AddressingMode::Immediate => format!("#${:02x}", address),
                AddressingMode::ZeroPage => format!("${:02x} = {:02x}", memory_address, memory_value),

                AddressingMode::ZeroPageX => format!(
                    "${:02x},X @ {:02x} = {:02x}",
                    address, memory_address, memory_value
                ),

                AddressingMode::ZeroPageY => format!(
                    "${:02x},Y @ {:02x} = {:02x}",
                    address, memory_address, memory_value
                ),

                AddressingMode::IndirectX => format!(
                    "(${:02x},X) @ {:02x} = {:04x} = {:02x}",
                    address,
                    (address.wrapping_add(cpu.register_x)),
                    memory_address,
                    memory_value
                ),

                AddressingMode::IndirectY => format!(
                    "(${:02x}),Y = {:04x} @ {:04x} = {:02x}",
                    address,
                    (memory_address.wrapping_sub(cpu.register_y as u16)),
                    memory_address,
                    memory_value
                ),

                AddressingMode::NoneAddressing => {
                    let address = (old_pc as usize + 2).wrapping_add(address as usize);
                    format!("${:04x}", address)
                }

                _ => panic!(
                    "Unexpected Addressing Mode {:?} has opcode length 2. Code {:02x}",
                    opcode.mode, opcode.len
                )
            }
        },

        3 => {
            let low = cpu.memory_read(old_pc + 1);
            let high = cpu.memory_read(old_pc + 2);

            hex_dump.push(low);
            hex_dump.push(high);

            let address = cpu.memory_read_u16(old_pc + 1);

            match opcode.mode {
                AddressingMode::NoneAddressing => {
                    if opcode.code == 0x6C {
                        // JMP Indirect
                        let jmp_address = if address & 0x00FF == 0x00F {
                            let low = cpu.memory_read(address);
                            let high = cpu.memory_read(address & 0xFF00);

                            (high as u16) << 8 | (low as u16)
                        } else {
                            cpu.memory_read_u16(address)
                        };

                        format!("(${:04x}) = {:04x}", address, jmp_address)
                    } else {
                        format!("${:04x}", address)
                    }
                }

                AddressingMode::Absolute => format!("${:04x} = {:02x}", memory_address, memory_value),

                AddressingMode::AbsoluteX => format!(
                    "${:04x},X @ {:04x} = {:02x}",
                    address, memory_address, memory_value
                ),

                AddressingMode::AbsoluteY => format!(
                    "${:04x},Y @ {:04x} = {:02x}",
                    address, memory_address, memory_value
                ),

                _ => panic!(
                    "Unexpected Addressing Mode {:?} has opcode length 3. Code {:02x}",
                    opcode.mode, opcode.len
                )
            }
        },

        _ => String::from("")
    };

    let hex_string = hex_dump
        .iter()
        .map(|element| format!("{:02x}", element))
        .collect::<Vec<String>>()
        .join(" ");
    let assembly_string = format!("{:04x}  {:8} {: >4} {}", old_pc, hex_string, opcode.mnemonic, info)
        .trim()
        .to_string();

    format!(
        "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}",
        assembly_string, cpu.register_a, cpu.register_x, cpu.register_y, cpu.register_p, cpu.register_sp,
    )
    .to_ascii_uppercase()
}