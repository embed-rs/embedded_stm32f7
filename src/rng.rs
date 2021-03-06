// autogenerated

use volatile;
use bit_field::BitField;

# [ doc = "Random number generator" ]
# [ repr ( C ) ]
pub struct Rng {
    # [ doc = "0x00 - control register" ]
    pub cr: volatile::ReadWrite<Cr>,
    # [ doc = "0x04 - status register" ]
    pub sr: volatile::ReadWrite<Sr>,
    # [ doc = "0x08 - data register" ]
    pub dr: volatile::ReadOnly<Dr>,
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Cr {
    bits: u32,
}

impl Cr {
    # [ doc = "Bit 3 - Interrupt enable" ]
    pub fn ie(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Random number generator enable" ]
    pub fn rngen(&self) -> bool {
        self.bits.get_bit(2u8)
    }
}

impl Default for Cr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Cr { bits: 0u32 }
    }
}

impl Cr {
    # [ doc = "Bit 3 - Interrupt enable" ]
    pub fn set_ie(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Random number generator enable" ]
    pub fn set_rngen(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Sr {
    bits: u32,
}

impl Sr {
    # [ doc = "Bit 6 - Seed error interrupt status" ]
    pub fn seis(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 5 - Clock error interrupt status" ]
    pub fn ceis(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 2 - Seed error current status" ]
    pub fn secs(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Clock error current status" ]
    pub fn cecs(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Data ready" ]
    pub fn drdy(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Sr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Sr { bits: 0u32 }
    }
}

impl Sr {
    # [ doc = "Bit 6 - Seed error interrupt status" ]
    pub fn set_seis(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 5 - Clock error interrupt status" ]
    pub fn set_ceis(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Dr {
    bits: u32,
}

impl Dr {
    # [ doc = "Bits 0:31 - Random data" ]
    pub fn rndata(&self) -> u32 {
        self.bits.get_range(0u8..32u8) as u32
    }
}

impl Default for Dr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Dr { bits: 0u32 }
    }
}

impl Dr {
    # [ doc = "Bits 0:31 - Random data" ]
    pub fn set_rndata(&mut self, value: u32) {
        self.bits.set_range(0u8..32u8, value as u32);
    }
}
