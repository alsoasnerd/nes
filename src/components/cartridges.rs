const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
const PRG_ROM_PAGE_SIZE: usize = 16384;
const CHR_ROM_PAGE_SIZE: usize = 8192;

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen
}

pub struct ROM {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub screen_mirroring: Mirroring
}

impl ROM {
    pub fn new(binary_data: &Vec<u8>) -> Result<Self, String> {
        if &binary_data[0..4] != NES_TAG {
            return Err("File is not in iNES file format".to_string());
        }

        let mapper = (binary_data[7] & 0b1111_0000) | (binary_data[6] >> 4);

        let ines_version = (binary_data[7] >> 2) & 0b11;

        if ines_version != 0 {
            return Err("NES2.0 format is not supported".to_string());
        }

        let four_screen = binary_data[6] & 0b1000 != 0;
        let vertical_mirroring = binary_data[6] & 0b1 != 0;
        let screen_mirroring = match (four_screen, vertical_mirroring) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };

        let prg_rom_size = binary_data[4] as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = binary_data[5] as usize * CHR_ROM_PAGE_SIZE;

        let skip_trainer = binary_data[6] & 0b100 != 0;

        let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        Ok(Self {
            prg_rom: binary_data[prg_rom_start..chr_rom_start].to_vec(),
            chr_rom: binary_data[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec(),
            mapper,
            screen_mirroring
        })
    }
}

pub mod test {

    use super::*;

    struct TestRom {
        header: Vec<u8>,
        trainer: Option<Vec<u8>>,
        pgp_rom: Vec<u8>,
        chr_rom: Vec<u8>,
    }

    fn create_rom(rom: TestRom) -> Vec<u8> {
        let mut result = Vec::with_capacity(
            rom.header.len()
                + rom.trainer.as_ref().map_or(0, |t| t.len())
                + rom.pgp_rom.len()
                + rom.chr_rom.len(),
        );

        result.extend(&rom.header);
        if let Some(t) = rom.trainer {
            result.extend(t);
        }

        result.extend(&rom.pgp_rom);
        result.extend(&rom.chr_rom);

        result
    }

    pub fn test_rom() -> ROM {
        let test_rom = create_rom( TestRom {
            header: vec![
                0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x31, 00, 00, 00, 00, 00, 00, 00, 00, 00,
            ],
            trainer: None,
            pgp_rom: vec![1; 2 * PRG_ROM_PAGE_SIZE],
            chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
        });

        ROM::new(&test_rom).unwrap()
    }
}