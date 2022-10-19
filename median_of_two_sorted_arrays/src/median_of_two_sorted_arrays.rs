fn median(nums: &Vec<i32>) -> (f64, usize) {
    let len = nums.len();
    let mid = len / 2;

    if len % 2 == 0 {
        return ((nums[mid - 1] + nums[mid]) as f64 / 2.0, mid - 1);
    }

    (nums[mid] as f64, mid)
}

pub struct Solution {}
impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        loop {
            if nums1.len() == 0 {
                return median(&nums2).0;
            }
            if nums2.len() == 0 {
                return median(&nums1).0;
            }

            let (med1, mid1) = median(&nums1);
            let (med2, mid2) = median(&nums2);

            if med1 == med2 {
                return med1;
            }

            if nums1.len() <= 2 || nums2.len() <= 2 {
                let mut nums = nums1;
                nums.extend(nums2);
                nums.sort();
                return median(&nums).0;
            }

            let chop = if mid1 < mid2 { mid1 } else { mid2 };
            if med1 < med2 {
                nums1.drain(..chop);
                nums2.drain((nums2.len() - chop)..);
            } else {
                nums1.drain((nums1.len() - chop)..);
                nums2.drain(..chop);
            }
        }
    }
}
