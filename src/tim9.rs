// autogenerated

use volatile;
use bit_field::BitField;

# [ doc = "General purpose timers" ]
# [ repr ( C ) ]
pub struct Tim9 {
    # [ doc = "0x00 - control register 1" ]
    pub cr1: volatile::ReadWrite<Cr1>,
    _reserved0: [u8; 4usize],
    # [ doc = "0x08 - slave mode control register" ]
    pub smcr: volatile::ReadWrite<Smcr>,
    # [ doc = "0x0c - DMA/Interrupt enable register" ]
    pub dier: volatile::ReadWrite<Dier>,
    # [ doc = "0x10 - status register" ]
    pub sr: volatile::ReadWrite<Sr>,
    # [ doc = "0x14 - event generation register" ]
    pub egr: volatile::WriteOnly<Egr>,
    # [ doc = "0x18 - capture/compare mode register 1 (output mode)" ]
    pub ccmr1_output: volatile::ReadWrite<Ccmr1Output>,
    _reserved1: [u8; 4usize],
    # [ doc = "0x20 - capture/compare enable register" ]
    pub ccer: volatile::ReadWrite<Ccer>,
    # [ doc = "0x24 - counter" ]
    pub cnt: volatile::ReadWrite<Cnt>,
    # [ doc = "0x28 - prescaler" ]
    pub psc: volatile::ReadWrite<Psc>,
    # [ doc = "0x2c - auto-reload register" ]
    pub arr: volatile::ReadWrite<Arr>,
    _reserved2: [u8; 4usize],
    # [ doc = "0x34 - capture/compare register 1" ]
    pub ccr1: volatile::ReadWrite<Ccr1>,
    # [ doc = "0x38 - capture/compare register 2" ]
    pub ccr2: volatile::ReadWrite<Ccr2>,
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr1 {
    bits: u32,
}

impl Cr1 {
    # [ doc = "Bits 8:9 - Clock division" ]
    pub fn ckd(&self) -> u8 {
        self.bits.get_range(8u8..10u8) as u8
    }
    # [ doc = "Bit 7 - Auto-reload preload enable" ]
    pub fn arpe(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bit 3 - One-pulse mode" ]
    pub fn opm(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Update request source" ]
    pub fn urs(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Update disable" ]
    pub fn udis(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Counter enable" ]
    pub fn cen(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Cr1 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Cr1 { bits: 0u32 }
    }
}

impl Cr1 {
    # [ doc = "Bits 8:9 - Clock division" ]
    pub fn set_ckd(&mut self, value: u8) {
        self.bits.set_range(8u8..10u8, value as u32);
    }
    # [ doc = "Bit 7 - Auto-reload preload enable" ]
    pub fn set_arpe(&mut self, value: bool) {
        self.bits.set_bit(7u8, value);
    }
    # [ doc = "Bit 3 - One-pulse mode" ]
    pub fn set_opm(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Update request source" ]
    pub fn set_urs(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Update disable" ]
    pub fn set_udis(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Counter enable" ]
    pub fn set_cen(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Smcr {
    bits: u32,
}

impl Smcr {
    # [ doc = "Bit 16 - Slave mode selection" ]
    pub fn sms_3(&self) -> bool {
        self.bits.get_bit(16u8)
    }
    # [ doc = "Bit 7 - Master/Slave mode" ]
    pub fn msm(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bits 4:6 - Trigger selection" ]
    pub fn ts(&self) -> u8 {
        self.bits.get_range(4u8..7u8) as u8
    }
    # [ doc = "Bits 0:2 - Slave mode selection" ]
    pub fn sms(&self) -> u8 {
        self.bits.get_range(0u8..3u8) as u8
    }
}

impl Default for Smcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Smcr { bits: 0u32 }
    }
}

impl Smcr {
    # [ doc = "Bit 16 - Slave mode selection" ]
    pub fn set_sms_3(&mut self, value: bool) {
        self.bits.set_bit(16u8, value);
    }
    # [ doc = "Bit 7 - Master/Slave mode" ]
    pub fn set_msm(&mut self, value: bool) {
        self.bits.set_bit(7u8, value);
    }
    # [ doc = "Bits 4:6 - Trigger selection" ]
    pub fn set_ts(&mut self, value: u8) {
        self.bits.set_range(4u8..7u8, value as u32);
    }
    # [ doc = "Bits 0:2 - Slave mode selection" ]
    pub fn set_sms(&mut self, value: u8) {
        self.bits.set_range(0u8..3u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dier {
    bits: u32,
}

impl Dier {
    # [ doc = "Bit 6 - Trigger interrupt enable" ]
    pub fn tie(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 2 - Capture/Compare 2 interrupt enable" ]
    pub fn cc2ie(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Capture/Compare 1 interrupt enable" ]
    pub fn cc1ie(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Update interrupt enable" ]
    pub fn uie(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Dier {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Dier { bits: 0u32 }
    }
}

impl Dier {
    # [ doc = "Bit 6 - Trigger interrupt enable" ]
    pub fn set_tie(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 2 - Capture/Compare 2 interrupt enable" ]
    pub fn set_cc2ie(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Capture/Compare 1 interrupt enable" ]
    pub fn set_cc1ie(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Update interrupt enable" ]
    pub fn set_uie(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sr {
    bits: u32,
}

impl Sr {
    # [ doc = "Bit 10 - Capture/compare 2 overcapture flag" ]
    pub fn cc2of(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bit 9 - Capture/Compare 1 overcapture flag" ]
    pub fn cc1of(&self) -> bool {
        self.bits.get_bit(9u8)
    }
    # [ doc = "Bit 6 - Trigger interrupt flag" ]
    pub fn tif(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 2 - Capture/Compare 2 interrupt flag" ]
    pub fn cc2if(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Capture/compare 1 interrupt flag" ]
    pub fn cc1if(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Update interrupt flag" ]
    pub fn uif(&self) -> bool {
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
    # [ doc = "Bit 10 - Capture/compare 2 overcapture flag" ]
    pub fn set_cc2of(&mut self, value: bool) {
        self.bits.set_bit(10u8, value);
    }
    # [ doc = "Bit 9 - Capture/Compare 1 overcapture flag" ]
    pub fn set_cc1of(&mut self, value: bool) {
        self.bits.set_bit(9u8, value);
    }
    # [ doc = "Bit 6 - Trigger interrupt flag" ]
    pub fn set_tif(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 2 - Capture/Compare 2 interrupt flag" ]
    pub fn set_cc2if(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Capture/compare 1 interrupt flag" ]
    pub fn set_cc1if(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Update interrupt flag" ]
    pub fn set_uif(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Egr {
    bits: u32,
}

impl Egr {
    # [ doc = "Bit 6 - Trigger generation" ]
    pub fn tg(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 2 - Capture/compare 2 generation" ]
    pub fn cc2g(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Capture/compare 1 generation" ]
    pub fn cc1g(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Update generation" ]
    pub fn ug(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Egr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Egr { bits: 0u32 }
    }
}

impl Egr {
    # [ doc = "Bit 6 - Trigger generation" ]
    pub fn set_tg(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 2 - Capture/compare 2 generation" ]
    pub fn set_cc2g(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Capture/compare 1 generation" ]
    pub fn set_cc1g(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Update generation" ]
    pub fn set_ug(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccmr1Output {
    bits: u32,
}

impl Ccmr1Output {
    # [ doc = "Bits 12:14 - Output Compare 2 mode" ]
    pub fn oc2m(&self) -> u8 {
        self.bits.get_range(12u8..15u8) as u8
    }
    # [ doc = "Bit 11 - Output Compare 2 preload enable" ]
    pub fn oc2pe(&self) -> bool {
        self.bits.get_bit(11u8)
    }
    # [ doc = "Bit 10 - Output Compare 2 fast enable" ]
    pub fn oc2fe(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bits 8:9 - Capture/Compare 2 selection" ]
    pub fn cc2s(&self) -> u8 {
        self.bits.get_range(8u8..10u8) as u8
    }
    # [ doc = "Bits 4:6 - Output Compare 1 mode" ]
    pub fn oc1m(&self) -> u8 {
        self.bits.get_range(4u8..7u8) as u8
    }
    # [ doc = "Bit 3 - Output Compare 1 preload enable" ]
    pub fn oc1pe(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Output Compare 1 fast enable" ]
    pub fn oc1fe(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
    pub fn cc1s(&self) -> u8 {
        self.bits.get_range(0u8..2u8) as u8
    }
}

impl Default for Ccmr1Output {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Ccmr1Output { bits: 0u32 }
    }
}

impl Ccmr1Output {
    # [ doc = "Bits 12:14 - Output Compare 2 mode" ]
    pub fn set_oc2m(&mut self, value: u8) {
        self.bits.set_range(12u8..15u8, value as u32);
    }
    # [ doc = "Bit 11 - Output Compare 2 preload enable" ]
    pub fn set_oc2pe(&mut self, value: bool) {
        self.bits.set_bit(11u8, value);
    }
    # [ doc = "Bit 10 - Output Compare 2 fast enable" ]
    pub fn set_oc2fe(&mut self, value: bool) {
        self.bits.set_bit(10u8, value);
    }
    # [ doc = "Bits 8:9 - Capture/Compare 2 selection" ]
    pub fn set_cc2s(&mut self, value: u8) {
        self.bits.set_range(8u8..10u8, value as u32);
    }
    # [ doc = "Bits 4:6 - Output Compare 1 mode" ]
    pub fn set_oc1m(&mut self, value: u8) {
        self.bits.set_range(4u8..7u8, value as u32);
    }
    # [ doc = "Bit 3 - Output Compare 1 preload enable" ]
    pub fn set_oc1pe(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Output Compare 1 fast enable" ]
    pub fn set_oc1fe(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
    pub fn set_cc1s(&mut self, value: u8) {
        self.bits.set_range(0u8..2u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccmr1Input {
    bits: u32,
}

impl Ccmr1Input {
    # [ doc = "Bits 12:14 - Input capture 2 filter" ]
    pub fn ic2f(&self) -> u8 {
        self.bits.get_range(12u8..15u8) as u8
    }
    # [ doc = "Bits 10:11 - Input capture 2 prescaler" ]
    pub fn ic2pcs(&self) -> u8 {
        self.bits.get_range(10u8..12u8) as u8
    }
    # [ doc = "Bits 8:9 - Capture/Compare 2 selection" ]
    pub fn cc2s(&self) -> u8 {
        self.bits.get_range(8u8..10u8) as u8
    }
    # [ doc = "Bits 4:6 - Input capture 1 filter" ]
    pub fn ic1f(&self) -> u8 {
        self.bits.get_range(4u8..7u8) as u8
    }
    # [ doc = "Bits 2:3 - Input capture 1 prescaler" ]
    pub fn icpcs(&self) -> u8 {
        self.bits.get_range(2u8..4u8) as u8
    }
    # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
    pub fn cc1s(&self) -> u8 {
        self.bits.get_range(0u8..2u8) as u8
    }
}

impl Default for Ccmr1Input {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Ccmr1Input { bits: 0u32 }
    }
}

impl Ccmr1Input {
    # [ doc = "Bits 12:14 - Input capture 2 filter" ]
    pub fn set_ic2f(&mut self, value: u8) {
        self.bits.set_range(12u8..15u8, value as u32);
    }
    # [ doc = "Bits 10:11 - Input capture 2 prescaler" ]
    pub fn set_ic2pcs(&mut self, value: u8) {
        self.bits.set_range(10u8..12u8, value as u32);
    }
    # [ doc = "Bits 8:9 - Capture/Compare 2 selection" ]
    pub fn set_cc2s(&mut self, value: u8) {
        self.bits.set_range(8u8..10u8, value as u32);
    }
    # [ doc = "Bits 4:6 - Input capture 1 filter" ]
    pub fn set_ic1f(&mut self, value: u8) {
        self.bits.set_range(4u8..7u8, value as u32);
    }
    # [ doc = "Bits 2:3 - Input capture 1 prescaler" ]
    pub fn set_icpcs(&mut self, value: u8) {
        self.bits.set_range(2u8..4u8, value as u32);
    }
    # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
    pub fn set_cc1s(&mut self, value: u8) {
        self.bits.set_range(0u8..2u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccer {
    bits: u32,
}

impl Ccer {
    # [ doc = "Bit 7 - Capture/Compare 2 output Polarity" ]
    pub fn cc2np(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bit 5 - Capture/Compare 2 output Polarity" ]
    pub fn cc2p(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 4 - Capture/Compare 2 output enable" ]
    pub fn cc2e(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - Capture/Compare 1 output Polarity" ]
    pub fn cc1np(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 1 - Capture/Compare 1 output Polarity" ]
    pub fn cc1p(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Capture/Compare 1 output enable" ]
    pub fn cc1e(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Ccer {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Ccer { bits: 0u32 }
    }
}

impl Ccer {
    # [ doc = "Bit 7 - Capture/Compare 2 output Polarity" ]
    pub fn set_cc2np(&mut self, value: bool) {
        self.bits.set_bit(7u8, value);
    }
    # [ doc = "Bit 5 - Capture/Compare 2 output Polarity" ]
    pub fn set_cc2p(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 4 - Capture/Compare 2 output enable" ]
    pub fn set_cc2e(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 3 - Capture/Compare 1 output Polarity" ]
    pub fn set_cc1np(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 1 - Capture/Compare 1 output Polarity" ]
    pub fn set_cc1p(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Capture/Compare 1 output enable" ]
    pub fn set_cc1e(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cnt {
    bits: u32,
}

impl Cnt {
    # [ doc = "Bits 0:15 - counter value" ]
    pub fn cnt(&self) -> u16 {
        self.bits.get_range(0u8..16u8) as u16
    }
}

impl Default for Cnt {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Cnt { bits: 0u32 }
    }
}

impl Cnt {
    # [ doc = "Bits 0:15 - counter value" ]
    pub fn set_cnt(&mut self, value: u16) {
        self.bits.set_range(0u8..16u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Psc {
    bits: u32,
}

impl Psc {
    # [ doc = "Bits 0:15 - Prescaler value" ]
    pub fn psc(&self) -> u16 {
        self.bits.get_range(0u8..16u8) as u16
    }
}

impl Default for Psc {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Psc { bits: 0u32 }
    }
}

impl Psc {
    # [ doc = "Bits 0:15 - Prescaler value" ]
    pub fn set_psc(&mut self, value: u16) {
        self.bits.set_range(0u8..16u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Arr {
    bits: u32,
}

impl Arr {
    # [ doc = "Bits 0:15 - Auto-reload value" ]
    pub fn arr(&self) -> u16 {
        self.bits.get_range(0u8..16u8) as u16
    }
}

impl Default for Arr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Arr { bits: 0u32 }
    }
}

impl Arr {
    # [ doc = "Bits 0:15 - Auto-reload value" ]
    pub fn set_arr(&mut self, value: u16) {
        self.bits.set_range(0u8..16u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr1 {
    bits: u32,
}

impl Ccr1 {
    # [ doc = "Bits 0:15 - Capture/Compare 1 value" ]
    pub fn ccr1(&self) -> u16 {
        self.bits.get_range(0u8..16u8) as u16
    }
}

impl Default for Ccr1 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Ccr1 { bits: 0u32 }
    }
}

impl Ccr1 {
    # [ doc = "Bits 0:15 - Capture/Compare 1 value" ]
    pub fn set_ccr1(&mut self, value: u16) {
        self.bits.set_range(0u8..16u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr2 {
    bits: u32,
}

impl Ccr2 {
    # [ doc = "Bits 0:15 - Capture/Compare 2 value" ]
    pub fn ccr2(&self) -> u16 {
        self.bits.get_range(0u8..16u8) as u16
    }
}

impl Default for Ccr2 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Ccr2 { bits: 0u32 }
    }
}

impl Ccr2 {
    # [ doc = "Bits 0:15 - Capture/Compare 2 value" ]
    pub fn set_ccr2(&mut self, value: u16) {
        self.bits.set_range(0u8..16u8, value as u32);
    }
}