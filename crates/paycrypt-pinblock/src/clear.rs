//! Clear PIN block codecs for ISO 9564-1:2017 §9.3 formats 0-3.

use crate::types::{ClearPinBlock, Pan, Pin, PinError, Rng};

/// A codec for the 8-byte clear PIN block formats (ISO 9564 formats 0-3).
pub trait ClearPinBlockCodec {
    /// Encodes `pin` (and `pan` for PAN-bound formats) into a clear PIN block.
    fn format(
        &self,
        pin: &Pin,
        pan: Option<&Pan>,
        rng: &mut dyn Rng,
    ) -> Result<ClearPinBlock, PinError>;

    /// Recovers the PIN from a clear PIN block.
    fn parse(&self, block: &ClearPinBlock, pan: Option<&Pan>) -> Result<Pin, PinError>;
}

fn pack_nibbles(nibbles: &[u8; 16]) -> [u8; 8] {
    let mut out = [0u8; 8];
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = (nibbles[2 * i] << 4) | (nibbles[2 * i + 1] & 0x0F);
    }
    out
}

fn build_pin_field(
    control: u8,
    pin: &Pin,
    mut pad: impl FnMut() -> u8,
) -> Result<[u8; 16], PinError> {
    let digits = pin.digits();
    // Pin::new enforces this too; the nibble layout depends on it.
    if digits.len() < 4 || digits.len() > 12 {
        return Err(PinError::BadPinLength);
    }
    let mut nibbles = [0u8; 16];
    nibbles[0] = control & 0x0F;
    nibbles[1] = digits.len() as u8;
    for (i, d) in digits.iter().enumerate() {
        nibbles[2 + i] = d - b'0';
    }
    for n in nibbles.iter_mut().skip(2 + digits.len()) {
        *n = pad() & 0x0F;
    }
    Ok(nibbles)
}

/// ISO-0/3 account field: `0000` plus the rightmost 12 PAN digits before the check digit.
fn account_field(pan: &Pan) -> [u8; 8] {
    let pan_str = pan.as_str().as_bytes();
    // Pan::new guarantees non-empty.
    let body = &pan_str[..pan_str.len() - 1];
    let mut nibbles = [0u8; 16];
    let take = body.len().min(12);
    let start_nibble = 16 - take;
    for (i, d) in body[body.len() - take..].iter().enumerate() {
        nibbles[start_nibble + i] = d - b'0';
    }
    pack_nibbles(&nibbles)
}

fn xor8(a: &[u8; 8], b: &[u8; 8]) -> [u8; 8] {
    let mut out = [0u8; 8];
    for i in 0..8 {
        out[i] = a[i] ^ b[i];
    }
    out
}

fn parse_pin_field(expected_control: u8, field: &[u8; 8]) -> Result<Pin, PinError> {
    let control = field[0] >> 4;
    if control != expected_control {
        return Err(PinError::BadBlock);
    }
    let len = (field[0] & 0x0F) as usize;
    if !(4..=12).contains(&len) {
        return Err(PinError::BadBlock);
    }
    let mut digits = alloc::vec::Vec::with_capacity(len);
    for i in 0..len {
        let nibble_idx = 2 + i;
        let byte = field[nibble_idx / 2];
        let nibble = if nibble_idx % 2 == 0 {
            byte >> 4
        } else {
            byte & 0x0F
        };
        if nibble > 9 {
            return Err(PinError::BadBlock);
        }
        digits.push(b'0' + nibble);
    }
    let s = core::str::from_utf8(&digits).map_err(|_| PinError::BadBlock)?;
    Pin::new(s)
}

/// ISO 9564 format 0 (ANSI X9.8): PIN field XOR PAN account field.
pub struct Iso0;
/// ISO 9564 format 1: PIN field with random fill, no PAN.
pub struct Iso1;
/// ISO 9564 format 2: offline / IC card, `F`-padded, no PAN.
pub struct Iso2;
/// ISO 9564 format 3: like format 0 but control nibble 3 and random `A-F` pad.
pub struct Iso3;

fn format_xor(
    control: u8,
    pin: &Pin,
    pan: Option<&Pan>,
    pad: impl FnMut() -> u8,
) -> Result<ClearPinBlock, PinError> {
    let pan = pan.ok_or(PinError::PanRequired)?;
    let pin_field = pack_nibbles(&build_pin_field(control, pin, pad)?);
    let acct = account_field(pan);
    Ok(ClearPinBlock::from_bytes(xor8(&pin_field, &acct)))
}

fn parse_xor(control: u8, block: &ClearPinBlock, pan: Option<&Pan>) -> Result<Pin, PinError> {
    let pan = pan.ok_or(PinError::PanRequired)?;
    let field = xor8(block.as_bytes(), &account_field(pan));
    parse_pin_field(control, &field)
}

impl ClearPinBlockCodec for Iso0 {
    fn format(
        &self,
        pin: &Pin,
        pan: Option<&Pan>,
        _rng: &mut dyn Rng,
    ) -> Result<ClearPinBlock, PinError> {
        format_xor(0, pin, pan, || 0x0F)
    }

    fn parse(&self, block: &ClearPinBlock, pan: Option<&Pan>) -> Result<Pin, PinError> {
        parse_xor(0, block, pan)
    }
}

impl ClearPinBlockCodec for Iso3 {
    fn format(
        &self,
        pin: &Pin,
        pan: Option<&Pan>,
        rng: &mut dyn Rng,
    ) -> Result<ClearPinBlock, PinError> {
        format_xor(3, pin, pan, || {
            let mut b = [0u8];
            rng.fill(&mut b);
            0x0A + (b[0] % 6)
        })
    }

    fn parse(&self, block: &ClearPinBlock, pan: Option<&Pan>) -> Result<Pin, PinError> {
        parse_xor(3, block, pan)
    }
}

impl ClearPinBlockCodec for Iso2 {
    fn format(
        &self,
        pin: &Pin,
        _pan: Option<&Pan>,
        _rng: &mut dyn Rng,
    ) -> Result<ClearPinBlock, PinError> {
        let field = pack_nibbles(&build_pin_field(2, pin, || 0x0F)?);
        Ok(ClearPinBlock::from_bytes(field))
    }

    fn parse(&self, block: &ClearPinBlock, _pan: Option<&Pan>) -> Result<Pin, PinError> {
        parse_pin_field(2, block.as_bytes())
    }
}

impl ClearPinBlockCodec for Iso1 {
    fn format(
        &self,
        pin: &Pin,
        _pan: Option<&Pan>,
        rng: &mut dyn Rng,
    ) -> Result<ClearPinBlock, PinError> {
        let field = pack_nibbles(&build_pin_field(1, pin, || {
            let mut b = [0u8];
            rng.fill(&mut b);
            b[0]
        })?);
        Ok(ClearPinBlock::from_bytes(field))
    }

    fn parse(&self, block: &ClearPinBlock, _pan: Option<&Pan>) -> Result<Pin, PinError> {
        parse_pin_field(1, block.as_bytes())
    }
}
