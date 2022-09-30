use crate::components::assembly;
use crate::components::cpu::AddressingMode;
use crate::components::cpu::CPU;
use std::collections::HashMap;

pub fn trace(cpu: &mut CPU) -> String {
    let ref opscodes: HashMap<u8, &'static assembly::OpCode> = *assembly::OPCODES_MAP;

    let code = cpu.memory_read(cpu.register_pc);
    let ops = opscodes.get(&code).unwrap();

    let begin = cpu.register_pc;
    let mut hex_dump = vec![];
    hex_dump.push(code);

    let (memory_address, stored_value) = match ops.mode {
        AddressingMode::Immediate | AddressingMode::NoneAddressing => (0, 0),
        _ => {
            let (address, _) = cpu.get_absolute_address(&ops.mode, begin + 1);
            (address, cpu.memory_read(address))
        }
    };

    let tmp = match ops.len {
        1 => match ops.code {
            0x0a | 0x4a | 0x2a | 0x6a => format!("A "),
            _ => String::from(""),
        },
        2 => {
            let address: u8 = cpu.memory_read(begin + 1);
            // let value = cpu.memory_read(address));
            hex_dump.push(address);

            match ops.mode {
                AddressingMode::Immediate => format!("#${:02x}", address),
                AddressingMode::ZeroPage => {
                    format!("${:02x} = {:02x}", memory_address, stored_value)
                }
                AddressingMode::ZeroPageX => format!(
                    "${:02x},X @ {:02x} = {:02x}",
                    address, memory_address, stored_value
                ),
                AddressingMode::ZeroPageY => format!(
                    "${:02x},Y @ {:02x} = {:02x}",
                    address, memory_address, stored_value
                ),
                AddressingMode::IndirectX => format!(
                    "(${:02x},X) @ {:02x} = {:04x} = {:02x}",
                    address,
                    (address.wrapping_add(cpu.register_x)),
                    memory_address,
                    stored_value
                ),
                AddressingMode::IndirectY => format!(
                    "(${:02x}),Y = {:04x} @ {:04x} = {:02x}",
                    address,
                    (memory_address.wrapping_sub(cpu.register_y as u16)),
                    memory_address,
                    stored_value
                ),
                AddressingMode::NoneAddressing => {
                    // assuming local jumps: BNE, BVS, etc....
                    let address: usize =
                        (begin as usize + 2).wrapping_add((address as i8) as usize);
                    format!("${:04x}", address)
                }

                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 2. code {:02x}",
                    ops.mode, ops.code
                ),
            }
        }
        3 => {
            let address_lo = cpu.memory_read(begin + 1);
            let address_hi = cpu.memory_read(begin + 2);
            hex_dump.push(address_lo);
            hex_dump.push(address_hi);

            let address = cpu.memory_read_u16(begin + 1);

            match ops.mode {
                AddressingMode::NoneAddressing => {
                    if ops.code == 0x6c {
                        //jmp indirect
                        let jmp_address = if address & 0x00FF == 0x00FF {
                            let lo = cpu.memory_read(address);
                            let hi = cpu.memory_read(address & 0xFF00);
                            (hi as u16) << 8 | (lo as u16)
                        } else {
                            cpu.memory_read_u16(address)
                        };

                        // let jmp_address = cpu.memory_read_u16(address);
                        format!("(${:04x}) = {:04x}", address, jmp_address)
                    } else {
                        format!("${:04x}", address)
                    }
                }
                AddressingMode::Absolute => {
                    format!("${:04x} = {:02x}", memory_address, stored_value)
                }
                AddressingMode::AbsoluteX => format!(
                    "${:04x},X @ {:04x} = {:02x}",
                    address, memory_address, stored_value
                ),
                AddressingMode::AbsoluteY => format!(
                    "${:04x},Y @ {:04x} = {:02x}",
                    address, memory_address, stored_value
                ),
                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 3. code {:02x}",
                    ops.mode, ops.code
                ),
            }
        }
        _ => String::from(""),
    };

    let hex_str = hex_dump
        .iter()
        .map(|z| format!("{:02x}", z))
        .collect::<Vec<String>>()
        .join(" ");
    let asm_str = format!("{:04x}  {:8} {: >4} {}", begin, hex_str, ops.mnemonic, tmp)
        .trim()
        .to_string();

    format!(
        "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}",
        asm_str, cpu.register_a, cpu.register_x, cpu.register_y, cpu.register_p, cpu.register_sp,
    )
    .to_ascii_uppercase()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::components::bus::BUS;
    use crate::components::cartridge::test::test_rom;
    use crate::components::joypads::Joypad;
    use crate::components::ppu::PPU;

    #[test]
    fn test_format_trace() {
        let mut bus = BUS::new(test_rom(), |_ppu: &PPU, _joypad: &mut Joypad| {});
        bus.memory_write(100, 0xa2);
        bus.memory_write(101, 0x01);
        bus.memory_write(102, 0xca);
        bus.memory_write(103, 0x88);
        bus.memory_write(104, 0x00);

        let mut cpu = CPU::new(bus);
        cpu.register_pc = 0x64;
        cpu.register_a = 1;
        cpu.register_x = 2;
        cpu.register_y = 3;
        let mut result: Vec<String> = vec![];
        cpu.run_with_callback(|cpu| {
            result.push(trace(cpu));
        });
        assert_eq!(
            "0064  A2 01     LDX #$01                        A:01 X:02 Y:03 P:24 SP:FD",
            result[0]
        );
        assert_eq!(
            "0066  CA        DEX                             A:01 X:01 Y:03 P:24 SP:FD",
            result[1]
        );
        assert_eq!(
            "0067  88        DEY                             A:01 X:00 Y:03 P:26 SP:FD",
            result[2]
        );
    }

    #[test]
    fn test_format_memory_access() {
        let mut bus = BUS::new(test_rom(), |_ppu: &PPU, _joypad: &mut Joypad| {});
        // ORA ($33), Y
        bus.memory_write(100, 0x11);
        bus.memory_write(101, 0x33);

        //data
        bus.memory_write(0x33, 00);
        bus.memory_write(0x34, 04);

        //target cell
        bus.memory_write(0x400, 0xAA);

        let mut cpu = CPU::new(bus);
        cpu.register_pc = 0x64;
        cpu.register_y = 0;
        let mut result: Vec<String> = vec![];
        cpu.run_with_callback(|cpu| {
            result.push(trace(cpu));
        });
        assert_eq!(
            "0064  11 33     ORA ($33),Y = 0400 @ 0400 = AA  A:00 X:00 Y:00 P:24 SP:FD",
            result[0]
        );
    }
}
