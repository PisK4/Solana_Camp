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
        let high: u128;
        let (mut low, low_over_mul) = self.low.overflowing_mul(other.low);
        let (mut low_over_high, mut low_over_low): (u128, u128) = (0, 0);
        if low_over_mul {
            let low1_low = (self.low & 0xFFFFFFFFFFFFFFFF) as u64;
            let low1_high = (self.low >> 64) as u64;
            let low2_low = (other.low & 0xFFFFFFFFFFFFFFFF) as u64;
            let low2_high = (other.low >> 64) as u64;

            let low_low = low1_low as u128 * low2_low as u128;
            let low_high = low1_high as u128 * low2_low as u128;
            let high_low = low1_low as u128 * low2_high as u128;
            let high_high = low1_high as u128 * low2_high as u128;

            low_over_low = low_low
                + ((low_high & 0xFFFFFFFFFFFFFFFF) << 64)
                + ((high_low & 0xFFFFFFFFFFFFFFFF) << 64);
            let get_low_over_high = (low_high >> 64) + (high_low >> 64) + high_high;

            if low_over_low < low_low {
                low_over_high = get_low_over_high + 1;
            } else {
                low_over_high = get_low_over_high;
            }
        }

        if self.high == 1 && self.low == 0 && other.high == 1 && other.low == 0 {
            high = u128::MAX;
            low = u128::MAX;
        } else if self.high >= 1 && other.high == 0 {
            if low_over_mul {
                let (a, a_state) = other.low.overflowing_mul(self.high);
                if a_state {
                    msg!("Overflow!");
                    return None;
                } else {
                    let (b, b_state) = a.overflowing_add(low_over_high.clone());
                    if b_state {
                        msg!("Overflow!");
                        return None;
                    } else {
                        high = b;
                    }
                }
            } else {
                let (a, a_state) = other.low.overflowing_mul(self.high);
                if a_state {
                    msg!("Overflow!");
                    return None;
                } else {
                    high = a;
                }
            }
        } else if self.high == 0 && other.high >= 1 {
            if low_over_mul {
                let (a, a_state) = self.low.overflowing_mul(other.high);
                if a_state {
                    msg!("Overflow!");
                    return None;
                } else {
                    let (b, b_state) = a.overflowing_add(low_over_high.clone());
                    if b_state {
                        msg!("Overflow!");
                        return None;
                    } else {
                        high = b;
                    }
                }
            } else {
                let (a, a_state) = self.low.overflowing_mul(other.high);
                if a_state {
                    msg!("Overflow!");
                    return None;
                } else {
                    high = a;
                }
            }
        } else if self.high == 0 && other.high == 0 {
            if low_over_mul {
                high = low_over_high.clone();
            } else {
                high = 0;
            }
        } else {
            msg!("Overflow!");
            return None;
        }

        Some(Self { high, low })
    }

    pub fn check_div(self, other: Self) -> Option<Self> {
        if other.is_zero() {
            msg!("Division by zero error!");
            return None;
        }

        let (high, low): (u128, u128);

        if self.high == 0 && other.high == 0 {
            high = 0;
            low = self.low.checked_div(other.low)?;
        } else if self.high == 0 && other.high != 0 {
            high = 0;
            low = 0;
        } else if self.high != 0 && other.high == 0 {
            if other.low == 0 {
                high = 0;
                low = 0;
            } else {
                let a = u128::MAX.checked_div(other.low)?;
                let new_number1 = Uint256::new(0, a);
                let new_number2 = Uint256::new(0, self.high);
                let final_high_div_num = new_number1.check_mul(new_number2)?;
                let final_low_div;
                if self.low != 0 {
                    final_low_div = self.low.checked_div(other.low)?;
                } else {
                    final_low_div = 0;
                }
                let final_low_div_num = Uint256::new(0, final_low_div);
                let final_uint256 = final_high_div_num.check_add(final_low_div_num)?;
                high = final_uint256.high;
                low = final_uint256.low;
            }
        } else {
            let (double_other_high, over_mul) = other.high.overflowing_mul(2);
            if self.high > other.high {
                if over_mul {
                    high = 0;
                    low = 1;
                } else {
                    if self.high > double_other_high {
                        high = 0;
                        low = self.high.checked_div(other.high)?;
                    } else {
                        if self.high == double_other_high && other.low == 0 {
                            high = 0;
                            low = 2;
                        } else {
                            high = 0;
                            low = 1;
                        }
                    }
                }
            } else if self.high == other.high && self.low >= other.low {
                high = 0;
                low = 1;
            } else {
                high = 0;
                low = 0;
            }
        }
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
