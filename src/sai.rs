// autogenerated

use volatile;
use bit_field::BitField;

# [ doc = "Serial audio interface" ]
# [ repr ( C ) ]
pub struct Sai {
    # [ doc = "0x00 - Global configuration register" ]
    pub gcr: volatile::ReadWrite<Gcr>,
    # [ doc = "0x04 - AConfiguration register 1" ]
    pub acr1: volatile::ReadWrite<Acr1>,
    # [ doc = "0x08 - AConfiguration register 2" ]
    pub acr2: volatile::ReadWrite<Acr2>,
    # [ doc = "0x0c - AFRCR" ]
    pub afrcr: volatile::ReadWrite<Afrcr>,
    # [ doc = "0x10 - ASlot register" ]
    pub aslotr: volatile::ReadWrite<Aslotr>,
    # [ doc = "0x14 - AInterrupt mask register2" ]
    pub aim: volatile::ReadWrite<Aim>,
    # [ doc = "0x18 - AStatus register" ]
    pub asr: volatile::ReadWrite<Asr>,
    # [ doc = "0x1c - AClear flag register" ]
    pub aclrfr: volatile::ReadWrite<Aclrfr>,
    # [ doc = "0x20 - AData register" ]
    pub adr: volatile::ReadWrite<Adr>,
    # [ doc = "0x24 - BConfiguration register 1" ]
    pub bcr1: volatile::ReadWrite<Bcr1>,
    # [ doc = "0x28 - BConfiguration register 2" ]
    pub bcr2: volatile::ReadWrite<Bcr2>,
    # [ doc = "0x2c - BFRCR" ]
    pub bfrcr: volatile::ReadWrite<Bfrcr>,
    # [ doc = "0x30 - BSlot register" ]
    pub bslotr: volatile::ReadWrite<Bslotr>,
    # [ doc = "0x34 - BInterrupt mask register2" ]
    pub bim: volatile::ReadWrite<Bim>,
    # [ doc = "0x38 - BStatus register" ]
    pub bsr: volatile::ReadOnly<Bsr>,
    # [ doc = "0x3c - BClear flag register" ]
    pub bclrfr: volatile::WriteOnly<Bclrfr>,
    # [ doc = "0x40 - BData register" ]
    pub bdr: volatile::ReadWrite<Bdr>,
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Gcr {
    bits: u32,
}

impl Gcr {
    # [ doc = "Bits 0:1 - Synchronization inputs" ]
    pub fn syncin(&self) -> u8 {
        self.bits.get_range(0u8..2u8) as u8
    }
    # [ doc = "Bits 4:5 - Synchronization outputs" ]
    pub fn syncout(&self) -> u8 {
        self.bits.get_range(4u8..6u8) as u8
    }
}

impl Default for Gcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Gcr { bits: 0u32 }
    }
}

impl Gcr {
    # [ doc = "Bits 0:1 - Synchronization inputs" ]
    pub fn set_syncin(&mut self, value: u8) {
        self.bits.set_range(0u8..2u8, value as u32);
    }
    # [ doc = "Bits 4:5 - Synchronization outputs" ]
    pub fn set_syncout(&mut self, value: u8) {
        self.bits.set_range(4u8..6u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Acr1 {
    bits: u32,
}

impl Acr1 {
    # [ doc = "Bits 20:23 - Master clock divider" ]
    pub fn mcjdiv(&self) -> u8 {
        self.bits.get_range(20u8..24u8) as u8
    }
    # [ doc = "Bit 19 - No divider" ]
    pub fn nodiv(&self) -> bool {
        self.bits.get_bit(19u8)
    }
    # [ doc = "Bit 17 - DMA enable" ]
    pub fn dmaen(&self) -> bool {
        self.bits.get_bit(17u8)
    }
    # [ doc = "Bit 16 - Audio block A enable" ]
    pub fn saiaen(&self) -> bool {
        self.bits.get_bit(16u8)
    }
    # [ doc = "Bit 13 - Output drive" ]
    pub fn out_dri(&self) -> bool {
        self.bits.get_bit(13u8)
    }
    # [ doc = "Bit 12 - Mono mode" ]
    pub fn mono(&self) -> bool {
        self.bits.get_bit(12u8)
    }
    # [ doc = "Bits 10:11 - Synchronization enable" ]
    pub fn syncen(&self) -> u8 {
        self.bits.get_range(10u8..12u8) as u8
    }
    # [ doc = "Bit 9 - Clock strobing edge" ]
    pub fn ckstr(&self) -> bool {
        self.bits.get_bit(9u8)
    }
    # [ doc = "Bit 8 - Least significant bit first" ]
    pub fn lsbfirst(&self) -> bool {
        self.bits.get_bit(8u8)
    }
    # [ doc = "Bits 5:7 - Data size" ]
    pub fn ds(&self) -> u8 {
        self.bits.get_range(5u8..8u8) as u8
    }
    # [ doc = "Bits 2:3 - Protocol configuration" ]
    pub fn prtcfg(&self) -> u8 {
        self.bits.get_range(2u8..4u8) as u8
    }
    # [ doc = "Bits 0:1 - Audio block mode" ]
    pub fn mode(&self) -> u8 {
        self.bits.get_range(0u8..2u8) as u8
    }
}

impl Default for Acr1 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Acr1 { bits: 64u32 }
    }
}

impl Acr1 {
    # [ doc = "Bits 20:23 - Master clock divider" ]
    pub fn set_mcjdiv(&mut self, value: u8) {
        self.bits.set_range(20u8..24u8, value as u32);
    }
    # [ doc = "Bit 19 - No divider" ]
    pub fn set_nodiv(&mut self, value: bool) {
        self.bits.set_bit(19u8, value);
    }
    # [ doc = "Bit 17 - DMA enable" ]
    pub fn set_dmaen(&mut self, value: bool) {
        self.bits.set_bit(17u8, value);
    }
    # [ doc = "Bit 16 - Audio block A enable" ]
    pub fn set_saiaen(&mut self, value: bool) {
        self.bits.set_bit(16u8, value);
    }
    # [ doc = "Bit 13 - Output drive" ]
    pub fn set_out_dri(&mut self, value: bool) {
        self.bits.set_bit(13u8, value);
    }
    # [ doc = "Bit 12 - Mono mode" ]
    pub fn set_mono(&mut self, value: bool) {
        self.bits.set_bit(12u8, value);
    }
    # [ doc = "Bits 10:11 - Synchronization enable" ]
    pub fn set_syncen(&mut self, value: u8) {
        self.bits.set_range(10u8..12u8, value as u32);
    }
    # [ doc = "Bit 9 - Clock strobing edge" ]
    pub fn set_ckstr(&mut self, value: bool) {
        self.bits.set_bit(9u8, value);
    }
    # [ doc = "Bit 8 - Least significant bit first" ]
    pub fn set_lsbfirst(&mut self, value: bool) {
        self.bits.set_bit(8u8, value);
    }
    # [ doc = "Bits 5:7 - Data size" ]
    pub fn set_ds(&mut self, value: u8) {
        self.bits.set_range(5u8..8u8, value as u32);
    }
    # [ doc = "Bits 2:3 - Protocol configuration" ]
    pub fn set_prtcfg(&mut self, value: u8) {
        self.bits.set_range(2u8..4u8, value as u32);
    }
    # [ doc = "Bits 0:1 - Audio block mode" ]
    pub fn set_mode(&mut self, value: u8) {
        self.bits.set_range(0u8..2u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Acr2 {
    bits: u32,
}

impl Acr2 {
    # [ doc = "Bits 14:15 - Companding mode" ]
    pub fn comp(&self) -> u8 {
        self.bits.get_range(14u8..16u8) as u8
    }
    # [ doc = "Bit 13 - Complement bit" ]
    pub fn cpl(&self) -> bool {
        self.bits.get_bit(13u8)
    }
    # [ doc = "Bits 7:12 - Mute counter" ]
    pub fn mutecn(&self) -> u8 {
        self.bits.get_range(7u8..13u8) as u8
    }
    # [ doc = "Bit 6 - Mute value" ]
    pub fn muteval(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 5 - Mute" ]
    pub fn mute(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 4 - Tristate management on data line" ]
    pub fn tris(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - FIFO flush" ]
    pub fn fflus(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bits 0:2 - FIFO threshold" ]
    pub fn fth(&self) -> u8 {
        self.bits.get_range(0u8..3u8) as u8
    }
}

impl Default for Acr2 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Acr2 { bits: 0u32 }
    }
}

impl Acr2 {
    # [ doc = "Bits 14:15 - Companding mode" ]
    pub fn set_comp(&mut self, value: u8) {
        self.bits.set_range(14u8..16u8, value as u32);
    }
    # [ doc = "Bit 13 - Complement bit" ]
    pub fn set_cpl(&mut self, value: bool) {
        self.bits.set_bit(13u8, value);
    }
    # [ doc = "Bits 7:12 - Mute counter" ]
    pub fn set_mutecn(&mut self, value: u8) {
        self.bits.set_range(7u8..13u8, value as u32);
    }
    # [ doc = "Bit 6 - Mute value" ]
    pub fn set_muteval(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 5 - Mute" ]
    pub fn set_mute(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 4 - Tristate management on data line" ]
    pub fn set_tris(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 3 - FIFO flush" ]
    pub fn set_fflus(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bits 0:2 - FIFO threshold" ]
    pub fn set_fth(&mut self, value: u8) {
        self.bits.set_range(0u8..3u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Afrcr {
    bits: u32,
}

impl Afrcr {
    # [ doc = "Bit 18 - Frame synchronization offset" ]
    pub fn fsoff(&self) -> bool {
        self.bits.get_bit(18u8)
    }
    # [ doc = "Bit 17 - Frame synchronization polarity" ]
    pub fn fspol(&self) -> bool {
        self.bits.get_bit(17u8)
    }
    # [ doc = "Bit 16 - Frame synchronization definition" ]
    pub fn fsdef(&self) -> bool {
        self.bits.get_bit(16u8)
    }
    # [ doc = "Bits 8:14 - Frame synchronization active level length" ]
    pub fn fsall(&self) -> u8 {
        self.bits.get_range(8u8..15u8) as u8
    }
    # [ doc = "Bits 0:7 - Frame length" ]
    pub fn frl(&self) -> u8 {
        self.bits.get_range(0u8..8u8) as u8
    }
}

impl Default for Afrcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Afrcr { bits: 7u32 }
    }
}

impl Afrcr {
    # [ doc = "Bit 18 - Frame synchronization offset" ]
    pub fn set_fsoff(&mut self, value: bool) {
        self.bits.set_bit(18u8, value);
    }
    # [ doc = "Bit 17 - Frame synchronization polarity" ]
    pub fn set_fspol(&mut self, value: bool) {
        self.bits.set_bit(17u8, value);
    }
    # [ doc = "Bit 16 - Frame synchronization definition" ]
    pub fn set_fsdef(&mut self, value: bool) {
        self.bits.set_bit(16u8, value);
    }
    # [ doc = "Bits 8:14 - Frame synchronization active level length" ]
    pub fn set_fsall(&mut self, value: u8) {
        self.bits.set_range(8u8..15u8, value as u32);
    }
    # [ doc = "Bits 0:7 - Frame length" ]
    pub fn set_frl(&mut self, value: u8) {
        self.bits.set_range(0u8..8u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Aslotr {
    bits: u32,
}

impl Aslotr {
    # [ doc = "Bits 16:31 - Slot enable" ]
    pub fn sloten(&self) -> u16 {
        self.bits.get_range(16u8..32u8) as u16
    }
    # [ doc = "Bits 8:11 - Number of slots in an audio frame" ]
    pub fn nbslot(&self) -> u8 {
        self.bits.get_range(8u8..12u8) as u8
    }
    # [ doc = "Bits 6:7 - Slot size" ]
    pub fn slotsz(&self) -> u8 {
        self.bits.get_range(6u8..8u8) as u8
    }
    # [ doc = "Bits 0:4 - First bit offset" ]
    pub fn fboff(&self) -> u8 {
        self.bits.get_range(0u8..5u8) as u8
    }
}

impl Default for Aslotr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Aslotr { bits: 0u32 }
    }
}

impl Aslotr {
    # [ doc = "Bits 16:31 - Slot enable" ]
    pub fn set_sloten(&mut self, value: u16) {
        self.bits.set_range(16u8..32u8, value as u32);
    }
    # [ doc = "Bits 8:11 - Number of slots in an audio frame" ]
    pub fn set_nbslot(&mut self, value: u8) {
        self.bits.set_range(8u8..12u8, value as u32);
    }
    # [ doc = "Bits 6:7 - Slot size" ]
    pub fn set_slotsz(&mut self, value: u8) {
        self.bits.set_range(6u8..8u8, value as u32);
    }
    # [ doc = "Bits 0:4 - First bit offset" ]
    pub fn set_fboff(&mut self, value: u8) {
        self.bits.set_range(0u8..5u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Aim {
    bits: u32,
}

impl Aim {
    # [ doc = "Bit 6 - Late frame synchronization detection interrupt enable" ]
    pub fn lfsdet(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable" ]
    pub fn afsdetie(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 4 - Codec not ready interrupt enable" ]
    pub fn cnrdyie(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - FIFO request interrupt enable" ]
    pub fn freqie(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Wrong clock configuration interrupt enable" ]
    pub fn wckcfg(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Mute detection interrupt enable" ]
    pub fn mutedet(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Overrun/underrun interrupt enable" ]
    pub fn ovrudrie(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Aim {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Aim { bits: 0u32 }
    }
}

impl Aim {
    # [ doc = "Bit 6 - Late frame synchronization detection interrupt enable" ]
    pub fn set_lfsdet(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable" ]
    pub fn set_afsdetie(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 4 - Codec not ready interrupt enable" ]
    pub fn set_cnrdyie(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 3 - FIFO request interrupt enable" ]
    pub fn set_freqie(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Wrong clock configuration interrupt enable" ]
    pub fn set_wckcfg(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Mute detection interrupt enable" ]
    pub fn set_mutedet(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Overrun/underrun interrupt enable" ]
    pub fn set_ovrudrie(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Asr {
    bits: u32,
}

impl Asr {
    # [ doc = "Bits 16:18 - FIFO level threshold" ]
    pub fn flvl(&self) -> u8 {
        self.bits.get_range(16u8..19u8) as u8
    }
    # [ doc = "Bit 6 - Late frame synchronization detection" ]
    pub fn lfsdet(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 5 - Anticipated frame synchronization detection" ]
    pub fn afsdet(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 4 - Codec not ready" ]
    pub fn cnrdy(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - FIFO request" ]
    pub fn freq(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Wrong clock configuration flag. This bit is read only." ]
    pub fn wckcfg(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Mute detection" ]
    pub fn mutedet(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Overrun / underrun" ]
    pub fn ovrudr(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Asr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Asr { bits: 0u32 }
    }
}

impl Asr {
    # [ doc = "Bits 16:18 - FIFO level threshold" ]
    pub fn set_flvl(&mut self, value: u8) {
        self.bits.set_range(16u8..19u8, value as u32);
    }
    # [ doc = "Bit 6 - Late frame synchronization detection" ]
    pub fn set_lfsdet(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 5 - Anticipated frame synchronization detection" ]
    pub fn set_afsdet(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 4 - Codec not ready" ]
    pub fn set_cnrdy(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 3 - FIFO request" ]
    pub fn set_freq(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Wrong clock configuration flag. This bit is read only." ]
    pub fn set_wckcfg(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Mute detection" ]
    pub fn set_mutedet(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Overrun / underrun" ]
    pub fn set_ovrudr(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Aclrfr {
    bits: u32,
}

impl Aclrfr {
    # [ doc = "Bit 6 - Clear late frame synchronization detection flag" ]
    pub fn lfsdet(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 5 - Clear anticipated frame synchronization detection flag." ]
    pub fn cafsdet(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 4 - Clear codec not ready flag" ]
    pub fn cnrdy(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 2 - Clear wrong clock configuration flag" ]
    pub fn wckcfg(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Mute detection flag" ]
    pub fn mutedet(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Clear overrun / underrun" ]
    pub fn ovrudr(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Aclrfr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Aclrfr { bits: 0u32 }
    }
}

impl Aclrfr {
    # [ doc = "Bit 6 - Clear late frame synchronization detection flag" ]
    pub fn set_lfsdet(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 5 - Clear anticipated frame synchronization detection flag." ]
    pub fn set_cafsdet(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 4 - Clear codec not ready flag" ]
    pub fn set_cnrdy(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 2 - Clear wrong clock configuration flag" ]
    pub fn set_wckcfg(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Mute detection flag" ]
    pub fn set_mutedet(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Clear overrun / underrun" ]
    pub fn set_ovrudr(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Adr {
    bits: u32,
}

impl Adr {
    # [ doc = "Bits 0:31 - Data" ]
    pub fn data(&self) -> u32 {
        self.bits.get_range(0u8..32u8) as u32
    }
}

impl Default for Adr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Adr { bits: 0u32 }
    }
}

impl Adr {
    # [ doc = "Bits 0:31 - Data" ]
    pub fn set_data(&mut self, value: u32) {
        self.bits.set_range(0u8..32u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bcr1 {
    bits: u32,
}

impl Bcr1 {
    # [ doc = "Bits 20:23 - Master clock divider" ]
    pub fn mcjdiv(&self) -> u8 {
        self.bits.get_range(20u8..24u8) as u8
    }
    # [ doc = "Bit 19 - No divider" ]
    pub fn nodiv(&self) -> bool {
        self.bits.get_bit(19u8)
    }
    # [ doc = "Bit 17 - DMA enable" ]
    pub fn dmaen(&self) -> bool {
        self.bits.get_bit(17u8)
    }
    # [ doc = "Bit 16 - Audio block B enable" ]
    pub fn saiben(&self) -> bool {
        self.bits.get_bit(16u8)
    }
    # [ doc = "Bit 13 - Output drive" ]
    pub fn out_dri(&self) -> bool {
        self.bits.get_bit(13u8)
    }
    # [ doc = "Bit 12 - Mono mode" ]
    pub fn mono(&self) -> bool {
        self.bits.get_bit(12u8)
    }
    # [ doc = "Bits 10:11 - Synchronization enable" ]
    pub fn syncen(&self) -> u8 {
        self.bits.get_range(10u8..12u8) as u8
    }
    # [ doc = "Bit 9 - Clock strobing edge" ]
    pub fn ckstr(&self) -> bool {
        self.bits.get_bit(9u8)
    }
    # [ doc = "Bit 8 - Least significant bit first" ]
    pub fn lsbfirst(&self) -> bool {
        self.bits.get_bit(8u8)
    }
    # [ doc = "Bits 5:7 - Data size" ]
    pub fn ds(&self) -> u8 {
        self.bits.get_range(5u8..8u8) as u8
    }
    # [ doc = "Bits 2:3 - Protocol configuration" ]
    pub fn prtcfg(&self) -> u8 {
        self.bits.get_range(2u8..4u8) as u8
    }
    # [ doc = "Bits 0:1 - Audio block mode" ]
    pub fn mode(&self) -> u8 {
        self.bits.get_range(0u8..2u8) as u8
    }
}

impl Default for Bcr1 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Bcr1 { bits: 64u32 }
    }
}

impl Bcr1 {
    # [ doc = "Bits 20:23 - Master clock divider" ]
    pub fn set_mcjdiv(&mut self, value: u8) {
        self.bits.set_range(20u8..24u8, value as u32);
    }
    # [ doc = "Bit 19 - No divider" ]
    pub fn set_nodiv(&mut self, value: bool) {
        self.bits.set_bit(19u8, value);
    }
    # [ doc = "Bit 17 - DMA enable" ]
    pub fn set_dmaen(&mut self, value: bool) {
        self.bits.set_bit(17u8, value);
    }
    # [ doc = "Bit 16 - Audio block B enable" ]
    pub fn set_saiben(&mut self, value: bool) {
        self.bits.set_bit(16u8, value);
    }
    # [ doc = "Bit 13 - Output drive" ]
    pub fn set_out_dri(&mut self, value: bool) {
        self.bits.set_bit(13u8, value);
    }
    # [ doc = "Bit 12 - Mono mode" ]
    pub fn set_mono(&mut self, value: bool) {
        self.bits.set_bit(12u8, value);
    }
    # [ doc = "Bits 10:11 - Synchronization enable" ]
    pub fn set_syncen(&mut self, value: u8) {
        self.bits.set_range(10u8..12u8, value as u32);
    }
    # [ doc = "Bit 9 - Clock strobing edge" ]
    pub fn set_ckstr(&mut self, value: bool) {
        self.bits.set_bit(9u8, value);
    }
    # [ doc = "Bit 8 - Least significant bit first" ]
    pub fn set_lsbfirst(&mut self, value: bool) {
        self.bits.set_bit(8u8, value);
    }
    # [ doc = "Bits 5:7 - Data size" ]
    pub fn set_ds(&mut self, value: u8) {
        self.bits.set_range(5u8..8u8, value as u32);
    }
    # [ doc = "Bits 2:3 - Protocol configuration" ]
    pub fn set_prtcfg(&mut self, value: u8) {
        self.bits.set_range(2u8..4u8, value as u32);
    }
    # [ doc = "Bits 0:1 - Audio block mode" ]
    pub fn set_mode(&mut self, value: u8) {
        self.bits.set_range(0u8..2u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bcr2 {
    bits: u32,
}

impl Bcr2 {
    # [ doc = "Bits 14:15 - Companding mode" ]
    pub fn comp(&self) -> u8 {
        self.bits.get_range(14u8..16u8) as u8
    }
    # [ doc = "Bit 13 - Complement bit" ]
    pub fn cpl(&self) -> bool {
        self.bits.get_bit(13u8)
    }
    # [ doc = "Bits 7:12 - Mute counter" ]
    pub fn mutecn(&self) -> u8 {
        self.bits.get_range(7u8..13u8) as u8
    }
    # [ doc = "Bit 6 - Mute value" ]
    pub fn muteval(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 5 - Mute" ]
    pub fn mute(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 4 - Tristate management on data line" ]
    pub fn tris(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - FIFO flush" ]
    pub fn fflus(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bits 0:2 - FIFO threshold" ]
    pub fn fth(&self) -> u8 {
        self.bits.get_range(0u8..3u8) as u8
    }
}

impl Default for Bcr2 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Bcr2 { bits: 0u32 }
    }
}

impl Bcr2 {
    # [ doc = "Bits 14:15 - Companding mode" ]
    pub fn set_comp(&mut self, value: u8) {
        self.bits.set_range(14u8..16u8, value as u32);
    }
    # [ doc = "Bit 13 - Complement bit" ]
    pub fn set_cpl(&mut self, value: bool) {
        self.bits.set_bit(13u8, value);
    }
    # [ doc = "Bits 7:12 - Mute counter" ]
    pub fn set_mutecn(&mut self, value: u8) {
        self.bits.set_range(7u8..13u8, value as u32);
    }
    # [ doc = "Bit 6 - Mute value" ]
    pub fn set_muteval(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 5 - Mute" ]
    pub fn set_mute(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 4 - Tristate management on data line" ]
    pub fn set_tris(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 3 - FIFO flush" ]
    pub fn set_fflus(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bits 0:2 - FIFO threshold" ]
    pub fn set_fth(&mut self, value: u8) {
        self.bits.set_range(0u8..3u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bfrcr {
    bits: u32,
}

impl Bfrcr {
    # [ doc = "Bit 18 - Frame synchronization offset" ]
    pub fn fsoff(&self) -> bool {
        self.bits.get_bit(18u8)
    }
    # [ doc = "Bit 17 - Frame synchronization polarity" ]
    pub fn fspol(&self) -> bool {
        self.bits.get_bit(17u8)
    }
    # [ doc = "Bit 16 - Frame synchronization definition" ]
    pub fn fsdef(&self) -> bool {
        self.bits.get_bit(16u8)
    }
    # [ doc = "Bits 8:14 - Frame synchronization active level length" ]
    pub fn fsall(&self) -> u8 {
        self.bits.get_range(8u8..15u8) as u8
    }
    # [ doc = "Bits 0:7 - Frame length" ]
    pub fn frl(&self) -> u8 {
        self.bits.get_range(0u8..8u8) as u8
    }
}

impl Default for Bfrcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Bfrcr { bits: 7u32 }
    }
}

impl Bfrcr {
    # [ doc = "Bit 18 - Frame synchronization offset" ]
    pub fn set_fsoff(&mut self, value: bool) {
        self.bits.set_bit(18u8, value);
    }
    # [ doc = "Bit 17 - Frame synchronization polarity" ]
    pub fn set_fspol(&mut self, value: bool) {
        self.bits.set_bit(17u8, value);
    }
    # [ doc = "Bit 16 - Frame synchronization definition" ]
    pub fn set_fsdef(&mut self, value: bool) {
        self.bits.set_bit(16u8, value);
    }
    # [ doc = "Bits 8:14 - Frame synchronization active level length" ]
    pub fn set_fsall(&mut self, value: u8) {
        self.bits.set_range(8u8..15u8, value as u32);
    }
    # [ doc = "Bits 0:7 - Frame length" ]
    pub fn set_frl(&mut self, value: u8) {
        self.bits.set_range(0u8..8u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bslotr {
    bits: u32,
}

impl Bslotr {
    # [ doc = "Bits 16:31 - Slot enable" ]
    pub fn sloten(&self) -> u16 {
        self.bits.get_range(16u8..32u8) as u16
    }
    # [ doc = "Bits 8:11 - Number of slots in an audio frame" ]
    pub fn nbslot(&self) -> u8 {
        self.bits.get_range(8u8..12u8) as u8
    }
    # [ doc = "Bits 6:7 - Slot size" ]
    pub fn slotsz(&self) -> u8 {
        self.bits.get_range(6u8..8u8) as u8
    }
    # [ doc = "Bits 0:4 - First bit offset" ]
    pub fn fboff(&self) -> u8 {
        self.bits.get_range(0u8..5u8) as u8
    }
}

impl Default for Bslotr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Bslotr { bits: 0u32 }
    }
}

impl Bslotr {
    # [ doc = "Bits 16:31 - Slot enable" ]
    pub fn set_sloten(&mut self, value: u16) {
        self.bits.set_range(16u8..32u8, value as u32);
    }
    # [ doc = "Bits 8:11 - Number of slots in an audio frame" ]
    pub fn set_nbslot(&mut self, value: u8) {
        self.bits.set_range(8u8..12u8, value as u32);
    }
    # [ doc = "Bits 6:7 - Slot size" ]
    pub fn set_slotsz(&mut self, value: u8) {
        self.bits.set_range(6u8..8u8, value as u32);
    }
    # [ doc = "Bits 0:4 - First bit offset" ]
    pub fn set_fboff(&mut self, value: u8) {
        self.bits.set_range(0u8..5u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bim {
    bits: u32,
}

impl Bim {
    # [ doc = "Bit 6 - Late frame synchronization detection interrupt enable" ]
    pub fn lfsdetie(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable" ]
    pub fn afsdetie(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 4 - Codec not ready interrupt enable" ]
    pub fn cnrdyie(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - FIFO request interrupt enable" ]
    pub fn freqie(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Wrong clock configuration interrupt enable" ]
    pub fn wckcfg(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Mute detection interrupt enable" ]
    pub fn mutedet(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Overrun/underrun interrupt enable" ]
    pub fn ovrudrie(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Bim {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Bim { bits: 0u32 }
    }
}

impl Bim {
    # [ doc = "Bit 6 - Late frame synchronization detection interrupt enable" ]
    pub fn set_lfsdetie(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable" ]
    pub fn set_afsdetie(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 4 - Codec not ready interrupt enable" ]
    pub fn set_cnrdyie(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 3 - FIFO request interrupt enable" ]
    pub fn set_freqie(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Wrong clock configuration interrupt enable" ]
    pub fn set_wckcfg(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Mute detection interrupt enable" ]
    pub fn set_mutedet(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Overrun/underrun interrupt enable" ]
    pub fn set_ovrudrie(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bsr {
    bits: u32,
}

impl Bsr {
    # [ doc = "Bits 16:18 - FIFO level threshold" ]
    pub fn flvl(&self) -> u8 {
        self.bits.get_range(16u8..19u8) as u8
    }
    # [ doc = "Bit 6 - Late frame synchronization detection" ]
    pub fn lfsdet(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 5 - Anticipated frame synchronization detection" ]
    pub fn afsdet(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 4 - Codec not ready" ]
    pub fn cnrdy(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - FIFO request" ]
    pub fn freq(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Wrong clock configuration flag" ]
    pub fn wckcfg(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Mute detection" ]
    pub fn mutedet(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Overrun / underrun" ]
    pub fn ovrudr(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Bsr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Bsr { bits: 0u32 }
    }
}

impl Bsr {
    # [ doc = "Bits 16:18 - FIFO level threshold" ]
    pub fn set_flvl(&mut self, value: u8) {
        self.bits.set_range(16u8..19u8, value as u32);
    }
    # [ doc = "Bit 6 - Late frame synchronization detection" ]
    pub fn set_lfsdet(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 5 - Anticipated frame synchronization detection" ]
    pub fn set_afsdet(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 4 - Codec not ready" ]
    pub fn set_cnrdy(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 3 - FIFO request" ]
    pub fn set_freq(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Wrong clock configuration flag" ]
    pub fn set_wckcfg(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Mute detection" ]
    pub fn set_mutedet(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Overrun / underrun" ]
    pub fn set_ovrudr(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bclrfr {
    bits: u32,
}

impl Bclrfr {
    # [ doc = "Bit 6 - Clear late frame synchronization detection flag" ]
    pub fn lfsdet(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 5 - Clear anticipated frame synchronization detection flag" ]
    pub fn cafsdet(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 4 - Clear codec not ready flag" ]
    pub fn cnrdy(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 2 - Clear wrong clock configuration flag" ]
    pub fn wckcfg(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Mute detection flag" ]
    pub fn mutedet(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Clear overrun / underrun" ]
    pub fn ovrudr(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Bclrfr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Bclrfr { bits: 0u32 }
    }
}

impl Bclrfr {
    # [ doc = "Bit 6 - Clear late frame synchronization detection flag" ]
    pub fn set_lfsdet(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 5 - Clear anticipated frame synchronization detection flag" ]
    pub fn set_cafsdet(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 4 - Clear codec not ready flag" ]
    pub fn set_cnrdy(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 2 - Clear wrong clock configuration flag" ]
    pub fn set_wckcfg(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Mute detection flag" ]
    pub fn set_mutedet(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Clear overrun / underrun" ]
    pub fn set_ovrudr(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bdr {
    bits: u32,
}

impl Bdr {
    # [ doc = "Bits 0:31 - Data" ]
    pub fn data(&self) -> u32 {
        self.bits.get_range(0u8..32u8) as u32
    }
}

impl Default for Bdr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Bdr { bits: 0u32 }
    }
}

impl Bdr {
    # [ doc = "Bits 0:31 - Data" ]
    pub fn set_data(&mut self, value: u32) {
        self.bits.set_range(0u8..32u8, value as u32);
    }
}