pub(crate) const MODE1_ADDR: u8 = 0x00_u8;
pub(crate) const LED_BASE_ADDR: u8 = 0x06_u8;
pub(crate) const PRE_SCALE_ADDR: u8 = 0xFE_u8;

pub(crate) const LED_BASE_OFFSET_MULTIPLIER: u8 = 0x04_u8;

pub(crate) const LED_ON_L_BASE_OFFSET: u8 = 0x00_u8;

pub(crate) const MODE1_RESTART_BIT: u8 = 1_u8 << 7_u8;
pub(crate) const MODE1_SLEEP_BIT: u8 = 1_u8 << 4_u8;
pub(crate) const MODE1_ALLCALL_BIT: u8 = 0_u8 << 4_u8;

/// Computes the base address of the LED register for the given channel.
///
/// # Arguments
///
/// * `channel` - The channel number (0-15).
///
/// # Returns
///
/// The base address of the LED register for the given channel.
#[inline]
pub(crate) fn led_base_addr(channel: u8) -> u8 {
    assert!(channel < 16_u8);

    LED_BASE_ADDR + channel * LED_BASE_OFFSET_MULTIPLIER
}

/// Computes the address of the LED ON_L register for the given channel.
///
/// # Arguments
///
/// * `channel` - The channel number (0-15).
///
/// # Returns
///
/// The address of the LED ON_L register for the given channel.
#[inline]
pub(crate) fn led_on_l_addr(channel: u8) -> u8 {
    led_base_addr(channel) + LED_ON_L_BASE_OFFSET
}
