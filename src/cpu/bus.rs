use byteorder::{LittleEndian, ByteOrder};

pub struct Bus {
    bios: Box<[u8]>
}

impl Bus {
    pub fn new(bios: Box<[u8]>) -> Bus {
        Bus {
            bios: bios
        }
    }

    pub fn load32(&self, address: u32) -> u32 {
        if address % 4 != 0 {
            panic!("unaligned load32 from adress {:#x}", address);
        }

        match address {
            address if 0xbfc00000 <= address && address < 0xbfc80000 => LittleEndian::read_u32(&self.bios[(address - 0xbfc00000) as usize..]),
			address if 0x1f801000 <= address && address < 0x1f801024 => { println!("load32 from unimplemented mem control {:#x}", address); 0 },
			address if 0x1f801060 <= address && address < 0x1f801064 => { println!("load32 from unimplemented ram size {:#x}", address); 0 },
			address if 0xfffe0130 <= address && address < 0xfffe0134 => { println!("load32 from unimplemented cache control {:#x}", address); 0 }
			_ => panic!("load32 from unimplemented range {:#08x}", address)
		}
    }

    pub fn store32(&self, address: u32, data: u32) {
        if address % 4 != 0 {
            panic!("unaligned store32 to adress {:#x}", address);
        }

        match address {
			address if 0xbfc00000 <= address && address < 0xbfc80000 => panic!("store32 to BIOS range {:#08x}", address),
			address if 0x1f801000 <= address && address < 0x1f801024 => println!("store32 to unimplemented mem control {:#x}", address),
			address if 0x1f801060 <= address && address < 0x1f801064 => println!("store32 to unimplemented ram size {:#x}", address),
			address if 0xfffe0130 <= address && address < 0xfffe0134 => println!("store32 to unimplemented cache control {:#x}", address),
			_ => panic!("store32 to unimplemented range {:#08x}", address)
		}
    }
}