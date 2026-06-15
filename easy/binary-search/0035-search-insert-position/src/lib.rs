//! See README.md for the problem statement.

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if target < nums[0] {
        return 0;
    } else if target > nums[nums.len()-1] {
        return nums.len() as i32;
    }

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
    lo as i32
}
