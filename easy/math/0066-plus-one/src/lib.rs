//! See README.md for the problem statement.

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    for digit in digits.iter_mut().rev() {
        if *digit == 9 {
            *digit = 0;
        } else {
            *digit += 1;
            return digits;
        }
    }

    digits.insert(0, 1); // Si llegamos aquí es porque todos eran 9, todos quedaron en 0
    digits
}
