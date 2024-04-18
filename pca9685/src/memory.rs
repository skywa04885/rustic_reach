pub(crate) const MODE1_ADDR: u8 = 0x00_u8;
pub(crate) const MODE2_ADDR: u8 = 0x01_u8;
pub(crate) const SUBADDR1_ADDR: u8 = 0x02_u8;
pub(crate) const SUBADDR2_ADDR: u8 = 0x03_u8;
pub(crate) const SUBADDR3_ADDR: u8 = 0x04_u8;
pub(crate) const ALLCALLADR_ADDR: u8 = 0x05_u8;

pub(crate) const LED_BASE_ADDR: u8 = 0x06_u8;
pub(crate) const LED_BASE_OFFSET_MULTIPLIER: u8 = 0x04_u8;

pub(crate) const LED_ON_L_BASE_OFFSET: u8 = 0x00_u8;
pub(crate) const LED_ON_H_BASE_OFFSET: u8 = 0x01_u8;
pub(crate) const LED_OFF_L_BASE_OFFSET: u8 = 0x02_u8;
pub(crate) const LED_OFF_H_BASE_OFFSET: u8 = 0x03_u8;

pub(crate) const ALL_LED_ON_L_ADDR: u8 = 0xFA_u8;
pub(crate) const ALL_LED_ON_H_ADDR: u8 = 0xFB_u8;
pub(crate) const ALL_LED_OFF_L_ADDR: u8 = 0xFC_u8;
pub(crate) const ALL_LED_OFF_H_ADDR: u8 = 0xFD_u8;
pub(crate) const PRE_SCALE_ADDR: u8 = 0xFE_u8;
pub(crate) const TEST_MODE: u8 = 0xFF_u8;