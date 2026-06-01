//! See README.md for the problem statement.

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    if nums.len() == 1 {
        return 1;
    }

    let mut write = 0;
    let mut i = 1;

    while i <= nums.len() - 1 {
        if nums[i] > nums[write] {
            write += 1;
            nums.swap(write, i);
            i = write + 1;
        } else {
            i += 1;
        }
    }

    write as i32 + 1
}
