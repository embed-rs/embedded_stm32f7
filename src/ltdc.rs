// autogenerated

use volatile;
use bit_field::BitField;

# [ doc = "LCD-TFT Controller" ]
# [ repr ( C ) ]
pub struct Ltdc {
    _reserved0: [u8; 8usize],
    # [ doc = "0x08 - Synchronization Size Configuration Register" ]
    pub sscr: volatile::ReadWrite<Sscr>,
    # [ doc = "0x0c - Back Porch Configuration Register" ]
    pub bpcr: volatile::ReadWrite<Bpcr>,
    # [ doc = "0x10 - Active Width Configuration Register" ]
    pub awcr: volatile::ReadWrite<Awcr>,
    # [ doc = "0x14 - Total Width Configuration Register" ]
    pub twcr: volatile::ReadWrite<Twcr>,
    # [ doc = "0x18 - Global Control Register" ]
    pub gcr: volatile::ReadWrite<Gcr>,
    _reserved1: [u8; 8usize],
    # [ doc = "0x24 - Shadow Reload Configuration Register" ]
    pub srcr: volatile::ReadWrite<Srcr>,
    _reserved2: [u8; 4usize],
    # [ doc = "0x2c - Background Color Configuration Register" ]
    pub bccr: volatile::ReadWrite<Bccr>,
    _reserved3: [u8; 4usize],
    # [ doc = "0x34 - Interrupt Enable Register" ]
    pub ier: volatile::ReadWrite<Ier>,
    # [ doc = "0x38 - Interrupt Status Register" ]
    pub isr: volatile::ReadOnly<Isr>,
    # [ doc = "0x3c - Interrupt Clear Register" ]
    pub icr: volatile::WriteOnly<Icr>,
    # [ doc = "0x40 - Line Interrupt Position Configuration Register" ]
    pub lipcr: volatile::ReadWrite<Lipcr>,
    # [ doc = "0x44 - Current Position Status Register" ]
    pub cpsr: volatile::ReadOnly<Cpsr>,
    # [ doc = "0x48 - Current Display Status Register" ]
    pub cdsr: volatile::ReadOnly<Cdsr>,
    _reserved4: [u8; 56usize],
    # [ doc = "0x84 - Layerx Control Register" ]
    pub l1cr: volatile::ReadWrite<L1cr>,
    # [ doc = "0x88 - Layerx Window Horizontal Position Configuration Register" ]
    pub l1whpcr: volatile::ReadWrite<L1whpcr>,
    # [ doc = "0x8c - Layerx Window Vertical Position Configuration Register" ]
    pub l1wvpcr: volatile::ReadWrite<L1wvpcr>,
    # [ doc = "0x90 - Layerx Color Keying Configuration Register" ]
    pub l1ckcr: volatile::ReadWrite<L1ckcr>,
    # [ doc = "0x94 - Layerx Pixel Format Configuration Register" ]
    pub l1pfcr: volatile::ReadWrite<L1pfcr>,
    # [ doc = "0x98 - Layerx Constant Alpha Configuration Register" ]
    pub l1cacr: volatile::ReadWrite<L1cacr>,
    # [ doc = "0x9c - Layerx Default Color Configuration Register" ]
    pub l1dccr: volatile::ReadWrite<L1dccr>,
    # [ doc = "0xa0 - Layerx Blending Factors Configuration Register" ]
    pub l1bfcr: volatile::ReadWrite<L1bfcr>,
    _reserved5: [u8; 8usize],
    # [ doc = "0xac - Layerx Color Frame Buffer Address Register" ]
    pub l1cfbar: volatile::ReadWrite<L1cfbar>,
    # [ doc = "0xb0 - Layerx Color Frame Buffer Length Register" ]
    pub l1cfblr: volatile::ReadWrite<L1cfblr>,
    # [ doc = "0xb4 - Layerx ColorFrame Buffer Line Number Register" ]
    pub l1cfblnr: volatile::ReadWrite<L1cfblnr>,
    _reserved6: [u8; 12usize],
    # [ doc = "0xc4 - Layerx CLUT Write Register" ]
    pub l1clutwr: volatile::WriteOnly<L1clutwr>,
    _reserved7: [u8; 60usize],
    # [ doc = "0x104 - Layerx Control Register" ]
    pub l2cr: volatile::ReadWrite<L2cr>,
    # [ doc = "0x108 - Layerx Window Horizontal Position Configuration Register" ]
    pub l2whpcr: volatile::ReadWrite<L2whpcr>,
    # [ doc = "0x10c - Layerx Window Vertical Position Configuration Register" ]
    pub l2wvpcr: volatile::ReadWrite<L2wvpcr>,
    # [ doc = "0x110 - Layerx Color Keying Configuration Register" ]
    pub l2ckcr: volatile::ReadWrite<L2ckcr>,
    # [ doc = "0x114 - Layerx Pixel Format Configuration Register" ]
    pub l2pfcr: volatile::ReadWrite<L2pfcr>,
    # [ doc = "0x118 - Layerx Constant Alpha Configuration Register" ]
    pub l2cacr: volatile::ReadWrite<L2cacr>,
    # [ doc = "0x11c - Layerx Default Color Configuration Register" ]
    pub l2dccr: volatile::ReadWrite<L2dccr>,
    # [ doc = "0x120 - Layerx Blending Factors Configuration Register" ]
    pub l2bfcr: volatile::ReadWrite<L2bfcr>,
    _reserved8: [u8; 8usize],
    # [ doc = "0x12c - Layerx Color Frame Buffer Address Register" ]
    pub l2cfbar: volatile::ReadWrite<L2cfbar>,
    # [ doc = "0x130 - Layerx Color Frame Buffer Length Register" ]
    pub l2cfblr: volatile::ReadWrite<L2cfblr>,
    # [ doc = "0x134 - Layerx ColorFrame Buffer Line Number Register" ]
    pub l2cfblnr: volatile::ReadWrite<L2cfblnr>,
    _reserved9: [u8; 12usize],
    # [ doc = "0x144 - Layerx CLUT Write Register" ]
    pub l2clutwr: volatile::WriteOnly<L2clutwr>,
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sscr {
    bits: u32,
}

impl Sscr {
    # [ doc = "Bits 16:25 - Horizontal Synchronization Width (in units of pixel clock period)" ]
    pub fn hsw(&self) -> u16 {
        self.bits.get_range(16u8..26u8) as u16
    }
    # [ doc = "Bits 0:10 - Vertical Synchronization Height (in units of horizontal scan line)" ]
    pub fn vsh(&self) -> u16 {
        self.bits.get_range(0u8..11u8) as u16
    }
}

impl Default for Sscr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Sscr { bits: 0u32 }
    }
}

impl Sscr {
    # [ doc = "Bits 16:25 - Horizontal Synchronization Width (in units of pixel clock period)" ]
    pub fn set_hsw(&mut self, value: u16) {
        self.bits.set_range(16u8..26u8, value as u32);
    }
    # [ doc = "Bits 0:10 - Vertical Synchronization Height (in units of horizontal scan line)" ]
    pub fn set_vsh(&mut self, value: u16) {
        self.bits.set_range(0u8..11u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bpcr {
    bits: u32,
}

impl Bpcr {
    # [ doc = "Bits 16:25 - Accumulated Horizontal back porch (in units of pixel clock period)" ]
    pub fn ahbp(&self) -> u16 {
        self.bits.get_range(16u8..26u8) as u16
    }
    # [ doc = "Bits 0:10 - Accumulated Vertical back porch (in units of horizontal scan line)" ]
    pub fn avbp(&self) -> u16 {
        self.bits.get_range(0u8..11u8) as u16
    }
}

impl Default for Bpcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Bpcr { bits: 0u32 }
    }
}

impl Bpcr {
    # [ doc = "Bits 16:25 - Accumulated Horizontal back porch (in units of pixel clock period)" ]
    pub fn set_ahbp(&mut self, value: u16) {
        self.bits.set_range(16u8..26u8, value as u32);
    }
    # [ doc = "Bits 0:10 - Accumulated Vertical back porch (in units of horizontal scan line)" ]
    pub fn set_avbp(&mut self, value: u16) {
        self.bits.set_range(0u8..11u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awcr {
    bits: u32,
}

impl Awcr {
    # [ doc = "Bits 16:25 - AAW" ]
    pub fn aaw(&self) -> u16 {
        self.bits.get_range(16u8..26u8) as u16
    }
    # [ doc = "Bits 0:10 - Accumulated Active Height (in units of horizontal scan line)" ]
    pub fn aah(&self) -> u16 {
        self.bits.get_range(0u8..11u8) as u16
    }
}

impl Default for Awcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Awcr { bits: 0u32 }
    }
}

impl Awcr {
    # [ doc = "Bits 16:25 - AAW" ]
    pub fn set_aaw(&mut self, value: u16) {
        self.bits.set_range(16u8..26u8, value as u32);
    }
    # [ doc = "Bits 0:10 - Accumulated Active Height (in units of horizontal scan line)" ]
    pub fn set_aah(&mut self, value: u16) {
        self.bits.set_range(0u8..11u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Twcr {
    bits: u32,
}

impl Twcr {
    # [ doc = "Bits 16:25 - Total Width (in units of pixel clock period)" ]
    pub fn totalw(&self) -> u16 {
        self.bits.get_range(16u8..26u8) as u16
    }
    # [ doc = "Bits 0:10 - Total Height (in units of horizontal scan line)" ]
    pub fn totalh(&self) -> u16 {
        self.bits.get_range(0u8..11u8) as u16
    }
}

impl Default for Twcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Twcr { bits: 0u32 }
    }
}

impl Twcr {
    # [ doc = "Bits 16:25 - Total Width (in units of pixel clock period)" ]
    pub fn set_totalw(&mut self, value: u16) {
        self.bits.set_range(16u8..26u8, value as u32);
    }
    # [ doc = "Bits 0:10 - Total Height (in units of horizontal scan line)" ]
    pub fn set_totalh(&mut self, value: u16) {
        self.bits.set_range(0u8..11u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Gcr {
    bits: u32,
}

impl Gcr {
    # [ doc = "Bit 31 - Horizontal Synchronization Polarity" ]
    pub fn hspol(&self) -> bool {
        self.bits.get_bit(31u8)
    }
    # [ doc = "Bit 30 - Vertical Synchronization Polarity" ]
    pub fn vspol(&self) -> bool {
        self.bits.get_bit(30u8)
    }
    # [ doc = "Bit 29 - Data Enable Polarity" ]
    pub fn depol(&self) -> bool {
        self.bits.get_bit(29u8)
    }
    # [ doc = "Bit 28 - Pixel Clock Polarity" ]
    pub fn pcpol(&self) -> bool {
        self.bits.get_bit(28u8)
    }
    # [ doc = "Bit 16 - Dither Enable" ]
    pub fn den(&self) -> bool {
        self.bits.get_bit(16u8)
    }
    # [ doc = "Bits 12:14 - Dither Red Width" ]
    pub fn drw(&self) -> u8 {
        self.bits.get_range(12u8..15u8) as u8
    }
    # [ doc = "Bits 8:10 - Dither Green Width" ]
    pub fn dgw(&self) -> u8 {
        self.bits.get_range(8u8..11u8) as u8
    }
    # [ doc = "Bits 4:6 - Dither Blue Width" ]
    pub fn dbw(&self) -> u8 {
        self.bits.get_range(4u8..7u8) as u8
    }
    # [ doc = "Bit 0 - LCD-TFT controller enable bit" ]
    pub fn ltdcen(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Gcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Gcr { bits: 8736u32 }
    }
}

impl Gcr {
    # [ doc = "Bit 31 - Horizontal Synchronization Polarity" ]
    pub fn set_hspol(&mut self, value: bool) {
        self.bits.set_bit(31u8, value);
    }
    # [ doc = "Bit 30 - Vertical Synchronization Polarity" ]
    pub fn set_vspol(&mut self, value: bool) {
        self.bits.set_bit(30u8, value);
    }
    # [ doc = "Bit 29 - Data Enable Polarity" ]
    pub fn set_depol(&mut self, value: bool) {
        self.bits.set_bit(29u8, value);
    }
    # [ doc = "Bit 28 - Pixel Clock Polarity" ]
    pub fn set_pcpol(&mut self, value: bool) {
        self.bits.set_bit(28u8, value);
    }
    # [ doc = "Bit 16 - Dither Enable" ]
    pub fn set_den(&mut self, value: bool) {
        self.bits.set_bit(16u8, value);
    }
    # [ doc = "Bit 0 - LCD-TFT controller enable bit" ]
    pub fn set_ltdcen(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Srcr {
    bits: u32,
}

impl Srcr {
    # [ doc = "Bit 1 - Vertical Blanking Reload" ]
    pub fn vbr(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Immediate Reload" ]
    pub fn imr(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Srcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Srcr { bits: 0u32 }
    }
}

impl Srcr {
    # [ doc = "Bit 1 - Vertical Blanking Reload" ]
    pub fn set_vbr(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Immediate Reload" ]
    pub fn set_imr(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bccr {
    bits: u32,
}

impl Bccr {
    # [ doc = "Bits 0:23 - Background Color Red value" ]
    pub fn bc(&self) -> u32 {
        self.bits.get_range(0u8..24u8) as u32
    }
}

impl Default for Bccr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Bccr { bits: 0u32 }
    }
}

impl Bccr {
    # [ doc = "Bits 0:23 - Background Color Red value" ]
    pub fn set_bc(&mut self, value: u32) {
        self.bits.set_range(0u8..24u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ier {
    bits: u32,
}

impl Ier {
    # [ doc = "Bit 3 - Register Reload interrupt enable" ]
    pub fn rrie(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Transfer Error Interrupt Enable" ]
    pub fn terrie(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - FIFO Underrun Interrupt Enable" ]
    pub fn fuie(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Line Interrupt Enable" ]
    pub fn lie(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Ier {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Ier { bits: 0u32 }
    }
}

impl Ier {
    # [ doc = "Bit 3 - Register Reload interrupt enable" ]
    pub fn set_rrie(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Transfer Error Interrupt Enable" ]
    pub fn set_terrie(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - FIFO Underrun Interrupt Enable" ]
    pub fn set_fuie(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Line Interrupt Enable" ]
    pub fn set_lie(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Isr {
    bits: u32,
}

impl Isr {
    # [ doc = "Bit 3 - Register Reload Interrupt Flag" ]
    pub fn rrif(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Transfer Error interrupt flag" ]
    pub fn terrif(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - FIFO Underrun Interrupt flag" ]
    pub fn fuif(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Line Interrupt flag" ]
    pub fn lif(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Isr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Isr { bits: 0u32 }
    }
}

impl Isr {
    # [ doc = "Bit 3 - Register Reload Interrupt Flag" ]
    pub fn set_rrif(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Transfer Error interrupt flag" ]
    pub fn set_terrif(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - FIFO Underrun Interrupt flag" ]
    pub fn set_fuif(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Line Interrupt flag" ]
    pub fn set_lif(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Icr {
    bits: u32,
}

impl Icr {
    # [ doc = "Bit 3 - Clears Register Reload Interrupt Flag" ]
    pub fn crrif(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Clears the Transfer Error Interrupt Flag" ]
    pub fn cterrif(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Clears the FIFO Underrun Interrupt flag" ]
    pub fn cfuif(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Clears the Line Interrupt Flag" ]
    pub fn clif(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Icr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Icr { bits: 0u32 }
    }
}

impl Icr {
    # [ doc = "Bit 3 - Clears Register Reload Interrupt Flag" ]
    pub fn set_crrif(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Clears the Transfer Error Interrupt Flag" ]
    pub fn set_cterrif(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Clears the FIFO Underrun Interrupt flag" ]
    pub fn set_cfuif(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Clears the Line Interrupt Flag" ]
    pub fn set_clif(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Lipcr {
    bits: u32,
}

impl Lipcr {
    # [ doc = "Bits 0:10 - Line Interrupt Position" ]
    pub fn lipos(&self) -> u16 {
        self.bits.get_range(0u8..11u8) as u16
    }
}

impl Default for Lipcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Lipcr { bits: 0u32 }
    }
}

impl Lipcr {
    # [ doc = "Bits 0:10 - Line Interrupt Position" ]
    pub fn set_lipos(&mut self, value: u16) {
        self.bits.set_range(0u8..11u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cpsr {
    bits: u32,
}

impl Cpsr {
    # [ doc = "Bits 16:31 - Current X Position" ]
    pub fn cxpos(&self) -> u16 {
        self.bits.get_range(16u8..32u8) as u16
    }
    # [ doc = "Bits 0:15 - Current Y Position" ]
    pub fn cypos(&self) -> u16 {
        self.bits.get_range(0u8..16u8) as u16
    }
}

impl Default for Cpsr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Cpsr { bits: 0u32 }
    }
}

impl Cpsr {
    # [ doc = "Bits 16:31 - Current X Position" ]
    pub fn set_cxpos(&mut self, value: u16) {
        self.bits.set_range(16u8..32u8, value as u32);
    }
    # [ doc = "Bits 0:15 - Current Y Position" ]
    pub fn set_cypos(&mut self, value: u16) {
        self.bits.set_range(0u8..16u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cdsr {
    bits: u32,
}

impl Cdsr {
    # [ doc = "Bit 3 - Horizontal Synchronization display Status" ]
    pub fn hsyncs(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Vertical Synchronization display Status" ]
    pub fn vsyncs(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Horizontal Data Enable display Status" ]
    pub fn hdes(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Vertical Data Enable display Status" ]
    pub fn vdes(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Cdsr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Cdsr { bits: 15u32 }
    }
}

impl Cdsr {
    # [ doc = "Bit 3 - Horizontal Synchronization display Status" ]
    pub fn set_hsyncs(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Vertical Synchronization display Status" ]
    pub fn set_vsyncs(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Horizontal Data Enable display Status" ]
    pub fn set_hdes(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Vertical Data Enable display Status" ]
    pub fn set_vdes(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L1cr {
    bits: u32,
}

impl L1cr {
    # [ doc = "Bit 4 - Color Look-Up Table Enable" ]
    pub fn cluten(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 1 - Color Keying Enable" ]
    pub fn colken(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Layer Enable" ]
    pub fn len(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for L1cr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L1cr { bits: 0u32 }
    }
}

impl L1cr {
    # [ doc = "Bit 4 - Color Look-Up Table Enable" ]
    pub fn set_cluten(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 1 - Color Keying Enable" ]
    pub fn set_colken(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Layer Enable" ]
    pub fn set_len(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L1whpcr {
    bits: u32,
}

impl L1whpcr {
    # [ doc = "Bits 16:27 - Window Horizontal Stop Position" ]
    pub fn whsppos(&self) -> u16 {
        self.bits.get_range(16u8..28u8) as u16
    }
    # [ doc = "Bits 0:11 - Window Horizontal Start Position" ]
    pub fn whstpos(&self) -> u16 {
        self.bits.get_range(0u8..12u8) as u16
    }
}

impl Default for L1whpcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L1whpcr { bits: 0u32 }
    }
}

impl L1whpcr {
    # [ doc = "Bits 16:27 - Window Horizontal Stop Position" ]
    pub fn set_whsppos(&mut self, value: u16) {
        self.bits.set_range(16u8..28u8, value as u32);
    }
    # [ doc = "Bits 0:11 - Window Horizontal Start Position" ]
    pub fn set_whstpos(&mut self, value: u16) {
        self.bits.set_range(0u8..12u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L1wvpcr {
    bits: u32,
}

impl L1wvpcr {
    # [ doc = "Bits 16:26 - Window Vertical Stop Position" ]
    pub fn wvsppos(&self) -> u16 {
        self.bits.get_range(16u8..27u8) as u16
    }
    # [ doc = "Bits 0:10 - Window Vertical Start Position" ]
    pub fn wvstpos(&self) -> u16 {
        self.bits.get_range(0u8..11u8) as u16
    }
}

impl Default for L1wvpcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L1wvpcr { bits: 0u32 }
    }
}

impl L1wvpcr {
    # [ doc = "Bits 16:26 - Window Vertical Stop Position" ]
    pub fn set_wvsppos(&mut self, value: u16) {
        self.bits.set_range(16u8..27u8, value as u32);
    }
    # [ doc = "Bits 0:10 - Window Vertical Start Position" ]
    pub fn set_wvstpos(&mut self, value: u16) {
        self.bits.set_range(0u8..11u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L1ckcr {
    bits: u32,
}

impl L1ckcr {
    # [ doc = "Bits 16:23 - Color Key Red value" ]
    pub fn ckred(&self) -> u8 {
        self.bits.get_range(16u8..24u8) as u8
    }
    # [ doc = "Bits 8:15 - Color Key Green value" ]
    pub fn ckgreen(&self) -> u8 {
        self.bits.get_range(8u8..16u8) as u8
    }
    # [ doc = "Bits 0:7 - Color Key Blue value" ]
    pub fn ckblue(&self) -> u8 {
        self.bits.get_range(0u8..8u8) as u8
    }
}

impl Default for L1ckcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L1ckcr { bits: 0u32 }
    }
}

impl L1ckcr {
    # [ doc = "Bits 16:23 - Color Key Red value" ]
    pub fn set_ckred(&mut self, value: u8) {
        self.bits.set_range(16u8..24u8, value as u32);
    }
    # [ doc = "Bits 8:15 - Color Key Green value" ]
    pub fn set_ckgreen(&mut self, value: u8) {
        self.bits.set_range(8u8..16u8, value as u32);
    }
    # [ doc = "Bits 0:7 - Color Key Blue value" ]
    pub fn set_ckblue(&mut self, value: u8) {
        self.bits.set_range(0u8..8u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L1pfcr {
    bits: u32,
}

impl L1pfcr {
    # [ doc = "Bits 0:2 - Pixel Format" ]
    pub fn pf(&self) -> u8 {
        self.bits.get_range(0u8..3u8) as u8
    }
}

impl Default for L1pfcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L1pfcr { bits: 0u32 }
    }
}

impl L1pfcr {
    # [ doc = "Bits 0:2 - Pixel Format" ]
    pub fn set_pf(&mut self, value: u8) {
        self.bits.set_range(0u8..3u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L1cacr {
    bits: u32,
}

impl L1cacr {
    # [ doc = "Bits 0:7 - Constant Alpha" ]
    pub fn consta(&self) -> u8 {
        self.bits.get_range(0u8..8u8) as u8
    }
}

impl Default for L1cacr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L1cacr { bits: 0u32 }
    }
}

impl L1cacr {
    # [ doc = "Bits 0:7 - Constant Alpha" ]
    pub fn set_consta(&mut self, value: u8) {
        self.bits.set_range(0u8..8u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L1dccr {
    bits: u32,
}

impl L1dccr {
    # [ doc = "Bits 24:31 - Default Color Alpha" ]
    pub fn dcalpha(&self) -> u8 {
        self.bits.get_range(24u8..32u8) as u8
    }
    # [ doc = "Bits 16:23 - Default Color Red" ]
    pub fn dcred(&self) -> u8 {
        self.bits.get_range(16u8..24u8) as u8
    }
    # [ doc = "Bits 8:15 - Default Color Green" ]
    pub fn dcgreen(&self) -> u8 {
        self.bits.get_range(8u8..16u8) as u8
    }
    # [ doc = "Bits 0:7 - Default Color Blue" ]
    pub fn dcblue(&self) -> u8 {
        self.bits.get_range(0u8..8u8) as u8
    }
}

impl Default for L1dccr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L1dccr { bits: 0u32 }
    }
}

impl L1dccr {
    # [ doc = "Bits 24:31 - Default Color Alpha" ]
    pub fn set_dcalpha(&mut self, value: u8) {
        self.bits.set_range(24u8..32u8, value as u32);
    }
    # [ doc = "Bits 16:23 - Default Color Red" ]
    pub fn set_dcred(&mut self, value: u8) {
        self.bits.set_range(16u8..24u8, value as u32);
    }
    # [ doc = "Bits 8:15 - Default Color Green" ]
    pub fn set_dcgreen(&mut self, value: u8) {
        self.bits.set_range(8u8..16u8, value as u32);
    }
    # [ doc = "Bits 0:7 - Default Color Blue" ]
    pub fn set_dcblue(&mut self, value: u8) {
        self.bits.set_range(0u8..8u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L1bfcr {
    bits: u32,
}

impl L1bfcr {
    # [ doc = "Bits 8:10 - Blending Factor 1" ]
    pub fn bf1(&self) -> u8 {
        self.bits.get_range(8u8..11u8) as u8
    }
    # [ doc = "Bits 0:2 - Blending Factor 2" ]
    pub fn bf2(&self) -> u8 {
        self.bits.get_range(0u8..3u8) as u8
    }
}

impl Default for L1bfcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L1bfcr { bits: 1543u32 }
    }
}

impl L1bfcr {
    # [ doc = "Bits 8:10 - Blending Factor 1" ]
    pub fn set_bf1(&mut self, value: u8) {
        self.bits.set_range(8u8..11u8, value as u32);
    }
    # [ doc = "Bits 0:2 - Blending Factor 2" ]
    pub fn set_bf2(&mut self, value: u8) {
        self.bits.set_range(0u8..3u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L1cfbar {
    bits: u32,
}

impl L1cfbar {
    # [ doc = "Bits 0:31 - Color Frame Buffer Start Address" ]
    pub fn cfbadd(&self) -> u32 {
        self.bits.get_range(0u8..32u8) as u32
    }
}

impl Default for L1cfbar {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L1cfbar { bits: 0u32 }
    }
}

impl L1cfbar {
    # [ doc = "Bits 0:31 - Color Frame Buffer Start Address" ]
    pub fn set_cfbadd(&mut self, value: u32) {
        self.bits.set_range(0u8..32u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L1cfblr {
    bits: u32,
}

impl L1cfblr {
    # [ doc = "Bits 16:28 - Color Frame Buffer Pitch in bytes" ]
    pub fn cfbp(&self) -> u16 {
        self.bits.get_range(16u8..29u8) as u16
    }
    # [ doc = "Bits 0:12 - Color Frame Buffer Line Length" ]
    pub fn cfbll(&self) -> u16 {
        self.bits.get_range(0u8..13u8) as u16
    }
}

impl Default for L1cfblr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L1cfblr { bits: 0u32 }
    }
}

impl L1cfblr {
    # [ doc = "Bits 16:28 - Color Frame Buffer Pitch in bytes" ]
    pub fn set_cfbp(&mut self, value: u16) {
        self.bits.set_range(16u8..29u8, value as u32);
    }
    # [ doc = "Bits 0:12 - Color Frame Buffer Line Length" ]
    pub fn set_cfbll(&mut self, value: u16) {
        self.bits.set_range(0u8..13u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L1cfblnr {
    bits: u32,
}

impl L1cfblnr {
    # [ doc = "Bits 0:10 - Frame Buffer Line Number" ]
    pub fn cfblnbr(&self) -> u16 {
        self.bits.get_range(0u8..11u8) as u16
    }
}

impl Default for L1cfblnr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L1cfblnr { bits: 0u32 }
    }
}

impl L1cfblnr {
    # [ doc = "Bits 0:10 - Frame Buffer Line Number" ]
    pub fn set_cfblnbr(&mut self, value: u16) {
        self.bits.set_range(0u8..11u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L1clutwr {
    bits: u32,
}

impl L1clutwr {
    # [ doc = "Bits 24:31 - CLUT Address" ]
    pub fn clutadd(&self) -> u8 {
        self.bits.get_range(24u8..32u8) as u8
    }
    # [ doc = "Bits 16:23 - Red value" ]
    pub fn red(&self) -> u8 {
        self.bits.get_range(16u8..24u8) as u8
    }
    # [ doc = "Bits 8:15 - Green value" ]
    pub fn green(&self) -> u8 {
        self.bits.get_range(8u8..16u8) as u8
    }
    # [ doc = "Bits 0:7 - Blue value" ]
    pub fn blue(&self) -> u8 {
        self.bits.get_range(0u8..8u8) as u8
    }
}

impl Default for L1clutwr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L1clutwr { bits: 0u32 }
    }
}

impl L1clutwr {
    # [ doc = "Bits 24:31 - CLUT Address" ]
    pub fn set_clutadd(&mut self, value: u8) {
        self.bits.set_range(24u8..32u8, value as u32);
    }
    # [ doc = "Bits 16:23 - Red value" ]
    pub fn set_red(&mut self, value: u8) {
        self.bits.set_range(16u8..24u8, value as u32);
    }
    # [ doc = "Bits 8:15 - Green value" ]
    pub fn set_green(&mut self, value: u8) {
        self.bits.set_range(8u8..16u8, value as u32);
    }
    # [ doc = "Bits 0:7 - Blue value" ]
    pub fn set_blue(&mut self, value: u8) {
        self.bits.set_range(0u8..8u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L2cr {
    bits: u32,
}

impl L2cr {
    # [ doc = "Bit 4 - Color Look-Up Table Enable" ]
    pub fn cluten(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 1 - Color Keying Enable" ]
    pub fn colken(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Layer Enable" ]
    pub fn len(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for L2cr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L2cr { bits: 0u32 }
    }
}

impl L2cr {
    # [ doc = "Bit 4 - Color Look-Up Table Enable" ]
    pub fn set_cluten(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 1 - Color Keying Enable" ]
    pub fn set_colken(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Layer Enable" ]
    pub fn set_len(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L2whpcr {
    bits: u32,
}

impl L2whpcr {
    # [ doc = "Bits 16:27 - Window Horizontal Stop Position" ]
    pub fn whsppos(&self) -> u16 {
        self.bits.get_range(16u8..28u8) as u16
    }
    # [ doc = "Bits 0:11 - Window Horizontal Start Position" ]
    pub fn whstpos(&self) -> u16 {
        self.bits.get_range(0u8..12u8) as u16
    }
}

impl Default for L2whpcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L2whpcr { bits: 0u32 }
    }
}

impl L2whpcr {
    # [ doc = "Bits 16:27 - Window Horizontal Stop Position" ]
    pub fn set_whsppos(&mut self, value: u16) {
        self.bits.set_range(16u8..28u8, value as u32);
    }
    # [ doc = "Bits 0:11 - Window Horizontal Start Position" ]
    pub fn set_whstpos(&mut self, value: u16) {
        self.bits.set_range(0u8..12u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L2wvpcr {
    bits: u32,
}

impl L2wvpcr {
    # [ doc = "Bits 16:26 - Window Vertical Stop Position" ]
    pub fn wvsppos(&self) -> u16 {
        self.bits.get_range(16u8..27u8) as u16
    }
    # [ doc = "Bits 0:10 - Window Vertical Start Position" ]
    pub fn wvstpos(&self) -> u16 {
        self.bits.get_range(0u8..11u8) as u16
    }
}

impl Default for L2wvpcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L2wvpcr { bits: 0u32 }
    }
}

impl L2wvpcr {
    # [ doc = "Bits 16:26 - Window Vertical Stop Position" ]
    pub fn set_wvsppos(&mut self, value: u16) {
        self.bits.set_range(16u8..27u8, value as u32);
    }
    # [ doc = "Bits 0:10 - Window Vertical Start Position" ]
    pub fn set_wvstpos(&mut self, value: u16) {
        self.bits.set_range(0u8..11u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L2ckcr {
    bits: u32,
}

impl L2ckcr {
    # [ doc = "Bits 15:23 - Color Key Red value" ]
    pub fn ckred(&self) -> u16 {
        self.bits.get_range(15u8..24u8) as u16
    }
    # [ doc = "Bits 8:14 - Color Key Green value" ]
    pub fn ckgreen(&self) -> u8 {
        self.bits.get_range(8u8..15u8) as u8
    }
    # [ doc = "Bits 0:7 - Color Key Blue value" ]
    pub fn ckblue(&self) -> u8 {
        self.bits.get_range(0u8..8u8) as u8
    }
}

impl Default for L2ckcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L2ckcr { bits: 0u32 }
    }
}

impl L2ckcr {
    # [ doc = "Bits 15:23 - Color Key Red value" ]
    pub fn set_ckred(&mut self, value: u16) {
        self.bits.set_range(15u8..24u8, value as u32);
    }
    # [ doc = "Bits 8:14 - Color Key Green value" ]
    pub fn set_ckgreen(&mut self, value: u8) {
        self.bits.set_range(8u8..15u8, value as u32);
    }
    # [ doc = "Bits 0:7 - Color Key Blue value" ]
    pub fn set_ckblue(&mut self, value: u8) {
        self.bits.set_range(0u8..8u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L2pfcr {
    bits: u32,
}

impl L2pfcr {
    # [ doc = "Bits 0:2 - Pixel Format" ]
    pub fn pf(&self) -> u8 {
        self.bits.get_range(0u8..3u8) as u8
    }
}

impl Default for L2pfcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L2pfcr { bits: 0u32 }
    }
}

impl L2pfcr {
    # [ doc = "Bits 0:2 - Pixel Format" ]
    pub fn set_pf(&mut self, value: u8) {
        self.bits.set_range(0u8..3u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L2cacr {
    bits: u32,
}

impl L2cacr {
    # [ doc = "Bits 0:7 - Constant Alpha" ]
    pub fn consta(&self) -> u8 {
        self.bits.get_range(0u8..8u8) as u8
    }
}

impl Default for L2cacr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L2cacr { bits: 0u32 }
    }
}

impl L2cacr {
    # [ doc = "Bits 0:7 - Constant Alpha" ]
    pub fn set_consta(&mut self, value: u8) {
        self.bits.set_range(0u8..8u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L2dccr {
    bits: u32,
}

impl L2dccr {
    # [ doc = "Bits 24:31 - Default Color Alpha" ]
    pub fn dcalpha(&self) -> u8 {
        self.bits.get_range(24u8..32u8) as u8
    }
    # [ doc = "Bits 16:23 - Default Color Red" ]
    pub fn dcred(&self) -> u8 {
        self.bits.get_range(16u8..24u8) as u8
    }
    # [ doc = "Bits 8:15 - Default Color Green" ]
    pub fn dcgreen(&self) -> u8 {
        self.bits.get_range(8u8..16u8) as u8
    }
    # [ doc = "Bits 0:7 - Default Color Blue" ]
    pub fn dcblue(&self) -> u8 {
        self.bits.get_range(0u8..8u8) as u8
    }
}

impl Default for L2dccr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L2dccr { bits: 0u32 }
    }
}

impl L2dccr {
    # [ doc = "Bits 24:31 - Default Color Alpha" ]
    pub fn set_dcalpha(&mut self, value: u8) {
        self.bits.set_range(24u8..32u8, value as u32);
    }
    # [ doc = "Bits 16:23 - Default Color Red" ]
    pub fn set_dcred(&mut self, value: u8) {
        self.bits.set_range(16u8..24u8, value as u32);
    }
    # [ doc = "Bits 8:15 - Default Color Green" ]
    pub fn set_dcgreen(&mut self, value: u8) {
        self.bits.set_range(8u8..16u8, value as u32);
    }
    # [ doc = "Bits 0:7 - Default Color Blue" ]
    pub fn set_dcblue(&mut self, value: u8) {
        self.bits.set_range(0u8..8u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L2bfcr {
    bits: u32,
}

impl L2bfcr {
    # [ doc = "Bits 8:10 - Blending Factor 1" ]
    pub fn bf1(&self) -> u8 {
        self.bits.get_range(8u8..11u8) as u8
    }
    # [ doc = "Bits 0:2 - Blending Factor 2" ]
    pub fn bf2(&self) -> u8 {
        self.bits.get_range(0u8..3u8) as u8
    }
}

impl Default for L2bfcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L2bfcr { bits: 1543u32 }
    }
}

impl L2bfcr {
    # [ doc = "Bits 8:10 - Blending Factor 1" ]
    pub fn set_bf1(&mut self, value: u8) {
        self.bits.set_range(8u8..11u8, value as u32);
    }
    # [ doc = "Bits 0:2 - Blending Factor 2" ]
    pub fn set_bf2(&mut self, value: u8) {
        self.bits.set_range(0u8..3u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L2cfbar {
    bits: u32,
}

impl L2cfbar {
    # [ doc = "Bits 0:31 - Color Frame Buffer Start Address" ]
    pub fn cfbadd(&self) -> u32 {
        self.bits.get_range(0u8..32u8) as u32
    }
}

impl Default for L2cfbar {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L2cfbar { bits: 0u32 }
    }
}

impl L2cfbar {
    # [ doc = "Bits 0:31 - Color Frame Buffer Start Address" ]
    pub fn set_cfbadd(&mut self, value: u32) {
        self.bits.set_range(0u8..32u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L2cfblr {
    bits: u32,
}

impl L2cfblr {
    # [ doc = "Bits 16:28 - Color Frame Buffer Pitch in bytes" ]
    pub fn cfbp(&self) -> u16 {
        self.bits.get_range(16u8..29u8) as u16
    }
    # [ doc = "Bits 0:12 - Color Frame Buffer Line Length" ]
    pub fn cfbll(&self) -> u16 {
        self.bits.get_range(0u8..13u8) as u16
    }
}

impl Default for L2cfblr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L2cfblr { bits: 0u32 }
    }
}

impl L2cfblr {
    # [ doc = "Bits 16:28 - Color Frame Buffer Pitch in bytes" ]
    pub fn set_cfbp(&mut self, value: u16) {
        self.bits.set_range(16u8..29u8, value as u32);
    }
    # [ doc = "Bits 0:12 - Color Frame Buffer Line Length" ]
    pub fn set_cfbll(&mut self, value: u16) {
        self.bits.set_range(0u8..13u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L2cfblnr {
    bits: u32,
}

impl L2cfblnr {
    # [ doc = "Bits 0:10 - Frame Buffer Line Number" ]
    pub fn cfblnbr(&self) -> u16 {
        self.bits.get_range(0u8..11u8) as u16
    }
}

impl Default for L2cfblnr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L2cfblnr { bits: 0u32 }
    }
}

impl L2cfblnr {
    # [ doc = "Bits 0:10 - Frame Buffer Line Number" ]
    pub fn set_cfblnbr(&mut self, value: u16) {
        self.bits.set_range(0u8..11u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct L2clutwr {
    bits: u32,
}

impl L2clutwr {
    # [ doc = "Bits 24:31 - CLUT Address" ]
    pub fn clutadd(&self) -> u8 {
        self.bits.get_range(24u8..32u8) as u8
    }
    # [ doc = "Bits 16:23 - Red value" ]
    pub fn red(&self) -> u8 {
        self.bits.get_range(16u8..24u8) as u8
    }
    # [ doc = "Bits 8:15 - Green value" ]
    pub fn green(&self) -> u8 {
        self.bits.get_range(8u8..16u8) as u8
    }
    # [ doc = "Bits 0:7 - Blue value" ]
    pub fn blue(&self) -> u8 {
        self.bits.get_range(0u8..8u8) as u8
    }
}

impl Default for L2clutwr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        L2clutwr { bits: 0u32 }
    }
}

impl L2clutwr {
    # [ doc = "Bits 24:31 - CLUT Address" ]
    pub fn set_clutadd(&mut self, value: u8) {
        self.bits.set_range(24u8..32u8, value as u32);
    }
    # [ doc = "Bits 16:23 - Red value" ]
    pub fn set_red(&mut self, value: u8) {
        self.bits.set_range(16u8..24u8, value as u32);
    }
    # [ doc = "Bits 8:15 - Green value" ]
    pub fn set_green(&mut self, value: u8) {
        self.bits.set_range(8u8..16u8, value as u32);
    }
    # [ doc = "Bits 0:7 - Blue value" ]
    pub fn set_blue(&mut self, value: u8) {
        self.bits.set_range(0u8..8u8, value as u32);
    }
}