// autogenerated

use volatile;
use bit_field::BitField;

# [ doc = "Window watchdog" ]
# [ repr ( C ) ]
pub struct Wwdg {
    # [ doc = "0x00 - Control register" ]
    pub cr: volatile::ReadWrite<Cr>,
    # [ doc = "0x04 - Configuration register" ]
    pub cfr: volatile::ReadWrite<Cfr>,
    # [ doc = "0x08 - Status register" ]
    pub sr: volatile::ReadWrite<Sr>,
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr {
    bits: u32,
}

impl Cr {
    # [ doc = "Bit 7 - Activation bit" ]
    pub fn wdga(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bits 0:6 - 7-bit counter (MSB to LSB)" ]
    pub fn t(&self) -> u8 {
        self.bits.get_range(0u8..7u8) as u8
    }
}

impl Default for Cr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Cr { bits: 127u32 }
    }
}

impl Cr {
    # [ doc = "Bit 7 - Activation bit" ]
    pub fn set_wdga(&mut self, value: bool) {
        self.bits.set_bit(7u8, value);
    }
    # [ doc = "Bits 0:6 - 7-bit counter (MSB to LSB)" ]
    pub fn set_t(&mut self, value: u8) {
        self.bits.set_range(0u8..7u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cfr {
    bits: u32,
}

impl Cfr {
    # [ doc = "Bit 9 - Early wakeup interrupt" ]
    pub fn ewi(&self) -> bool {
        self.bits.get_bit(9u8)
    }
    # [ doc = "Bit 8 - Timer base" ]
    pub fn wdgtb1(&self) -> bool {
        self.bits.get_bit(8u8)
    }
    # [ doc = "Bit 7 - Timer base" ]
    pub fn wdgtb0(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bits 0:6 - 7-bit window value" ]
    pub fn w(&self) -> u8 {
        self.bits.get_range(0u8..7u8) as u8
    }
}

impl Default for Cfr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Cfr { bits: 127u32 }
    }
}

impl Cfr {
    # [ doc = "Bit 9 - Early wakeup interrupt" ]
    pub fn set_ewi(&mut self, value: bool) {
        self.bits.set_bit(9u8, value);
    }
    # [ doc = "Bit 8 - Timer base" ]
    pub fn set_wdgtb1(&mut self, value: bool) {
        self.bits.set_bit(8u8, value);
    }
    # [ doc = "Bit 7 - Timer base" ]
    pub fn set_wdgtb0(&mut self, value: bool) {
        self.bits.set_bit(7u8, value);
    }
    # [ doc = "Bits 0:6 - 7-bit window value" ]
    pub fn set_w(&mut self, value: u8) {
        self.bits.set_range(0u8..7u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sr {
    bits: u32,
}

impl Sr {
    # [ doc = "Bit 0 - Early wakeup interrupt flag" ]
    pub fn ewif(&self) -> bool {
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
    # [ doc = "Bit 0 - Early wakeup interrupt flag" ]
    pub fn set_ewif(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}