//! See README.md for the problem statement.

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut lo = 0;
    let mut hi = nums.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    -1
}
