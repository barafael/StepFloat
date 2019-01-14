use std::fmt;
use std::ops::Neg;

#[derive(Copy, Clone, Debug)]
struct FloatBitPattern {
    value: u32,
}

impl std::fmt::LowerHex for FloatBitPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.value)
    }
}

impl From<f32> for FloatBitPattern {
    fn from(value: f32) -> Self {
        FloatBitPattern {
            value : unsafe {
                std::mem::transmute::<f32, u32>(value)
            }
        }
    }
}

impl From<FloatBitPattern> for f32 {
    fn from(pattern: FloatBitPattern) -> Self {
        unsafe {
            std::mem::transmute::<u32, f32>(pattern.value)
        }
    }
}

impl FloatBitPattern {
    fn new(value: u32) -> Self {
        FloatBitPattern {
            value
        }
    }
}

impl Neg for FloatBitPattern {
    type Output = FloatBitPattern;
    #[inline]
    fn neg(self) -> FloatBitPattern { 
        FloatBitPattern {
            value : if self.value & 0x8000_0000 == 0x8000_0000 {
                self.value & 0x7AAA_AAAA
            } else {
                self.value | 0x8000_0000
            },
        }
    }
}

trait Increment {
    fn incr(&mut self);
}

trait Decrement {
    fn decr(&mut self);
}

impl Increment for FloatBitPattern {
    fn incr(&mut self) {
        let mantissa = self.value & 0b0_00000000_11111111111111111111111;
        if mantissa == 0b0_00000000_11111111111111111111111 {
            let mut exponent = self.value & 0b0_11111111_00000000000000000000000;
            exponent >>= 23;
            exponent += 1;
            exponent <<= 23;
            self.value &= 0b1_00000000_11111111111111111111111;
            self.value |= exponent;
            // reset mantissa
            self.value &= 0b1_11111111_00000000000000000000000;
        } else {
            let mut mantissa = self.value & 0b0_00000000_11111111111111111111111;
            mantissa += 1;
            self.value &= 0b1_11111111_00000000000000000000000;
            self.value |= mantissa;
        }
    }
}

impl Decrement for FloatBitPattern {
    fn decr(&mut self) {
        let mantissa = self.value & 0b0_00000000_11111111111111111111111;
        if mantissa == 0b0_00000000_00000000000000000000000 {
            let mut exponent = self.value & 0b0_11111111_00000000000000000000000;
            exponent >>= 23;
            exponent -= 1;
            exponent <<= 23;
            self.value &= 0b1_00000000_11111111111111111111111;
            self.value |= exponent;
            // set mantissa
            self.value |= 0b0_00000000_11111111111111111111111;
        } else {
            let mut mantissa = self.value & 0b0_00000000_11111111111111111111111;
            mantissa -= 1;
            self.value &= 0b1_11111111_00000000000000000000000;
            self.value |= mantissa;
        }
    }
}

impl Increment for f32 {
    fn incr(&mut self) {
        let mut pattern = FloatBitPattern::from(*self);
        pattern.incr();
        *self = f32::from(pattern);
    }
}

impl Decrement for f32 {
    fn decr(&mut self) {
        let mut pattern = FloatBitPattern::from(*self);
        pattern.decr();
        *self = f32::from(pattern);
    }
}

fn main() {
    let initial = 0b111110111111111111111111111111;
    println!("pattern before 0.5: {:b}", FloatBitPattern::new(initial));
    let float = 0.49999;
    //println!("{:?}", float);
    let mut pattern: FloatBitPattern = float.into();
    //println!("{:x}", pattern);
    //pattern = -pattern;
    //println!("{:x}", pattern);
    let float: f32 = pattern.into();
    //println!("{:?}", float);

    let mut old_value = float;
    let mut old_diff = 0.0;
    for _i in 0..10 {
        pattern.incr();
        let value: f32 = f32::from(pattern);
        print!("{:.100}", value);
        let new_diff = value - old_value;
        if old_diff != new_diff {
            print!(", {:.100}", new_diff);
            old_diff = new_diff;
        }
        println!();
        old_value = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negation() {
        let initial = FloatBitPattern::from(2.0);
        let negated = initial.neg();
        assert_eq!(FloatBitPattern::from(-2.0).value, negated.value);
    }

    #[test]
    fn test_incr_decr_noboundary() {
        let initial = 0.1;
        let float_initial = initial;
        let mut float = float_initial;
        float.incr();
        float.decr();
        assert_eq!(float_initial, float);
    }

    #[test]
    fn test_incr_decr_boundary() {
        let pattern: u32 = 0b0_00001111_11111111111111111111111;
        let initial = 0.4999999701976776123046875000000;
        println!("{:b}", FloatBitPattern::from(initial).value);
        let float_initial = initial;
        let mut float = float_initial;
        float.incr();
        float.decr();
        assert_eq!(float_initial, float);
    }
}

