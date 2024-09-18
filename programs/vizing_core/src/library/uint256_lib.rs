use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub struct Uint256 {
    pub high: u128,
    pub low: u128,
}

impl Uint256 {
    pub fn new(high: u128, low: u128) -> Self {
        Self { high, low }
    }

    pub fn add(self, other: Self) -> Self {
        let (low, carry) = self.low.overflowing_add(other.low);
        let high = self.high + other.high + if carry { 1 } else { 0 };
        Self { high, low }
    }

    pub fn check_add(self, other: Self) -> Option<Self> {
        let (low, carry) = self.low.overflowing_add(other.low);
        let high = self
            .high
            .checked_add(other.high)?
            .checked_add(if carry { 1 } else { 0 })?;
        Some(Self { high, low })
    }

    pub fn check_sub(self, other: Self) -> Option<Self> {
        let (low, borrow) = self.low.overflowing_sub(other.low);
        let high = self
            .high
            .checked_sub(other.high)?
            .checked_sub(if borrow { 1 } else { 0 })?;
        Some(Self { high, low })
    }

    pub fn check_mul(self, other: Self) -> Option<Self> {
        let (low, carry) = self.low.overflowing_mul(other.low);

        let high_mul = self.high.checked_mul(other.high)?;
        let high_add1 = self.high.checked_mul(other.low)?;
        let high_add2 = self.low.checked_mul(other.high)?;

        let high = high_mul
            .checked_add(high_add1)?
            .checked_add(high_add2)?
            .checked_add(if carry { 1 } else { 0 })?;

        Some(Self { high, low })
    }

    pub fn check_div(self, other: Self) -> Option<Self> {
        if other.is_zero() {
            return None; 
        }
        let low = self.low.checked_div(other.low)?;
        let high = if self.high == 0 {
            0 
        } else {
            let total_self = (self.high.checked_shl(128))? + self.low; 
            let total_other = (other.high.checked_shl(128))? + other.low; 
            total_self.checked_div(total_other)? 
        };

        Some(Self { high, low })
    }

    pub fn is_zero(self) -> bool {
        if self.high == 0 && self.low == 0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn cmp(self, other: Self) -> u8 {
        if self.high < other.high {
            return 0;
        } else if self.high > other.high {
            return 2;
        } else {
            if self.low < other.low {
                return 0;
            } else if self.low > other.low {
                return 2;
            } else {
                return 1;
            }
        }
    }
}
