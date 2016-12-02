// autogenerated

use volatile;
use bit_field::BitField;

# [ doc = "Common ADC registers" ]
# [ repr ( C ) ]
pub struct CAdc {
    # [ doc = "0x00 - ADC Common status register" ]
    pub csr: volatile::ReadOnly<Csr>,
    # [ doc = "0x04 - ADC common control register" ]
    pub ccr: volatile::ReadWrite<Ccr>,
    # [ doc = "0x08 - ADC common regular data register for dual and triple modes" ]
    pub cdr: volatile::ReadOnly<Cdr>,
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Csr {
    bits: u32,
}

impl Csr {
    # [ doc = "Bit 21 - Overrun flag of ADC3" ]
    pub fn ovr3(&self) -> bool {
        self.bits.get_bit(21u8)
    }
    # [ doc = "Bit 20 - Regular channel Start flag of ADC 3" ]
    pub fn strt3(&self) -> bool {
        self.bits.get_bit(20u8)
    }
    # [ doc = "Bit 19 - Injected channel Start flag of ADC 3" ]
    pub fn jstrt3(&self) -> bool {
        self.bits.get_bit(19u8)
    }
    # [ doc = "Bit 18 - Injected channel end of conversion of ADC 3" ]
    pub fn jeoc3(&self) -> bool {
        self.bits.get_bit(18u8)
    }
    # [ doc = "Bit 17 - End of conversion of ADC 3" ]
    pub fn eoc3(&self) -> bool {
        self.bits.get_bit(17u8)
    }
    # [ doc = "Bit 16 - Analog watchdog flag of ADC 3" ]
    pub fn awd3(&self) -> bool {
        self.bits.get_bit(16u8)
    }
    # [ doc = "Bit 13 - Overrun flag of ADC 2" ]
    pub fn ovr2(&self) -> bool {
        self.bits.get_bit(13u8)
    }
    # [ doc = "Bit 12 - Regular channel Start flag of ADC 2" ]
    pub fn strt2(&self) -> bool {
        self.bits.get_bit(12u8)
    }
    # [ doc = "Bit 11 - Injected channel Start flag of ADC 2" ]
    pub fn jstrt2(&self) -> bool {
        self.bits.get_bit(11u8)
    }
    # [ doc = "Bit 10 - Injected channel end of conversion of ADC 2" ]
    pub fn jeoc2(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bit 9 - End of conversion of ADC 2" ]
    pub fn eoc2(&self) -> bool {
        self.bits.get_bit(9u8)
    }
    # [ doc = "Bit 8 - Analog watchdog flag of ADC 2" ]
    pub fn awd2(&self) -> bool {
        self.bits.get_bit(8u8)
    }
    # [ doc = "Bit 5 - Overrun flag of ADC 1" ]
    pub fn ovr1(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 4 - Regular channel Start flag of ADC 1" ]
    pub fn strt1(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - Injected channel Start flag of ADC 1" ]
    pub fn jstrt1(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Injected channel end of conversion of ADC 1" ]
    pub fn jeoc1(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - End of conversion of ADC 1" ]
    pub fn eoc1(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Analog watchdog flag of ADC 1" ]
    pub fn awd1(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Csr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Csr { bits: 0u32 }
    }
}

impl Csr {
    # [ doc = "Bit 21 - Overrun flag of ADC3" ]
    pub fn set_ovr3(&mut self, value: bool) {
        self.bits.set_bit(21u8, value);
    }
    # [ doc = "Bit 20 - Regular channel Start flag of ADC 3" ]
    pub fn set_strt3(&mut self, value: bool) {
        self.bits.set_bit(20u8, value);
    }
    # [ doc = "Bit 19 - Injected channel Start flag of ADC 3" ]
    pub fn set_jstrt3(&mut self, value: bool) {
        self.bits.set_bit(19u8, value);
    }
    # [ doc = "Bit 18 - Injected channel end of conversion of ADC 3" ]
    pub fn set_jeoc3(&mut self, value: bool) {
        self.bits.set_bit(18u8, value);
    }
    # [ doc = "Bit 17 - End of conversion of ADC 3" ]
    pub fn set_eoc3(&mut self, value: bool) {
        self.bits.set_bit(17u8, value);
    }
    # [ doc = "Bit 16 - Analog watchdog flag of ADC 3" ]
    pub fn set_awd3(&mut self, value: bool) {
        self.bits.set_bit(16u8, value);
    }
    # [ doc = "Bit 13 - Overrun flag of ADC 2" ]
    pub fn set_ovr2(&mut self, value: bool) {
        self.bits.set_bit(13u8, value);
    }
    # [ doc = "Bit 12 - Regular channel Start flag of ADC 2" ]
    pub fn set_strt2(&mut self, value: bool) {
        self.bits.set_bit(12u8, value);
    }
    # [ doc = "Bit 11 - Injected channel Start flag of ADC 2" ]
    pub fn set_jstrt2(&mut self, value: bool) {
        self.bits.set_bit(11u8, value);
    }
    # [ doc = "Bit 10 - Injected channel end of conversion of ADC 2" ]
    pub fn set_jeoc2(&mut self, value: bool) {
        self.bits.set_bit(10u8, value);
    }
    # [ doc = "Bit 9 - End of conversion of ADC 2" ]
    pub fn set_eoc2(&mut self, value: bool) {
        self.bits.set_bit(9u8, value);
    }
    # [ doc = "Bit 8 - Analog watchdog flag of ADC 2" ]
    pub fn set_awd2(&mut self, value: bool) {
        self.bits.set_bit(8u8, value);
    }
    # [ doc = "Bit 5 - Overrun flag of ADC 1" ]
    pub fn set_ovr1(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 4 - Regular channel Start flag of ADC 1" ]
    pub fn set_strt1(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 3 - Injected channel Start flag of ADC 1" ]
    pub fn set_jstrt1(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Injected channel end of conversion of ADC 1" ]
    pub fn set_jeoc1(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - End of conversion of ADC 1" ]
    pub fn set_eoc1(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Analog watchdog flag of ADC 1" ]
    pub fn set_awd1(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr {
    bits: u32,
}

impl Ccr {
    # [ doc = "Bit 23 - Temperature sensor and VREFINT enable" ]
    pub fn tsvrefe(&self) -> bool {
        self.bits.get_bit(23u8)
    }
    # [ doc = "Bit 22 - VBAT enable" ]
    pub fn vbate(&self) -> bool {
        self.bits.get_bit(22u8)
    }
    # [ doc = "Bits 16:17 - ADC prescaler" ]
    pub fn adcpre(&self) -> u8 {
        self.bits.get_range(16u8..18u8) as u8
    }
    # [ doc = "Bits 14:15 - Direct memory access mode for multi ADC mode" ]
    pub fn dma(&self) -> u8 {
        self.bits.get_range(14u8..16u8) as u8
    }
    # [ doc = "Bit 13 - DMA disable selection for multi-ADC mode" ]
    pub fn dds(&self) -> bool {
        self.bits.get_bit(13u8)
    }
    # [ doc = "Bits 8:11 - Delay between 2 sampling phases" ]
    pub fn delay(&self) -> u8 {
        self.bits.get_range(8u8..12u8) as u8
    }
    # [ doc = "Bits 0:4 - Multi ADC mode selection" ]
    pub fn mult(&self) -> u8 {
        self.bits.get_range(0u8..5u8) as u8
    }
}

impl Default for Ccr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Ccr { bits: 0u32 }
    }
}

impl Ccr {
    # [ doc = "Bit 23 - Temperature sensor and VREFINT enable" ]
    pub fn set_tsvrefe(&mut self, value: bool) {
        self.bits.set_bit(23u8, value);
    }
    # [ doc = "Bit 22 - VBAT enable" ]
    pub fn set_vbate(&mut self, value: bool) {
        self.bits.set_bit(22u8, value);
    }
    # [ doc = "Bits 16:17 - ADC prescaler" ]
    pub fn set_adcpre(&mut self, value: u8) {
        self.bits.set_range(16u8..18u8, value as u32);
    }
    # [ doc = "Bits 14:15 - Direct memory access mode for multi ADC mode" ]
    pub fn set_dma(&mut self, value: u8) {
        self.bits.set_range(14u8..16u8, value as u32);
    }
    # [ doc = "Bit 13 - DMA disable selection for multi-ADC mode" ]
    pub fn set_dds(&mut self, value: bool) {
        self.bits.set_bit(13u8, value);
    }
    # [ doc = "Bits 8:11 - Delay between 2 sampling phases" ]
    pub fn set_delay(&mut self, value: u8) {
        self.bits.set_range(8u8..12u8, value as u32);
    }
    # [ doc = "Bits 0:4 - Multi ADC mode selection" ]
    pub fn set_mult(&mut self, value: u8) {
        self.bits.set_range(0u8..5u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cdr {
    bits: u32,
}

impl Cdr {
    # [ doc = "Bits 16:31 - 2nd data item of a pair of regular conversions" ]
    pub fn data2(&self) -> u16 {
        self.bits.get_range(16u8..32u8) as u16
    }
    # [ doc = "Bits 0:15 - 1st data item of a pair of regular conversions" ]
    pub fn data1(&self) -> u16 {
        self.bits.get_range(0u8..16u8) as u16
    }
}

impl Default for Cdr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Cdr { bits: 0u32 }
    }
}

impl Cdr {
    # [ doc = "Bits 16:31 - 2nd data item of a pair of regular conversions" ]
    pub fn set_data2(&mut self, value: u16) {
        self.bits.set_range(16u8..32u8, value as u32);
    }
    # [ doc = "Bits 0:15 - 1st data item of a pair of regular conversions" ]
    pub fn set_data1(&mut self, value: u16) {
        self.bits.set_range(0u8..16u8, value as u32);
    }
}