//! See README.md for the problem statement.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Carry {
    One,
    Zero,
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    use Carry::*;
    let mut digits = digits;
    let mut carry = Carry::One;
    for digit in digits.iter_mut().rev() {
        match (carry, &mut *digit) {
            (One, 9) => {
                *digit = 0;
            }
            (One, n) => {
                *n += 1;
                carry = Zero;
            }
            (Zero, _) => {
                // do nothing
            }
        }
    }

    if carry == One {
        digits.push(1);
        let l = digits.len();
        digits.swap(0, l-1);
    }

    digits
}
