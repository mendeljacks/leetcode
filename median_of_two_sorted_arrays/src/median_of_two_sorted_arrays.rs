pub struct Solution {}

fn median(nums: &Vec<i32>) -> (f64, usize) {
    let len = nums.len();
    let mid = len / 2;

    if len % 2 == 0 {
        return ((nums[mid - 1] + nums[mid]) as f64 / 2.0, mid - 1);
    }

    (nums[mid] as f64, mid)
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums1 = nums1;
        let mut nums2 = nums2;

        loop {
            let len1 = nums1.len();
            let len2 = nums2.len();

            if len1 == 0 {
                return median(&nums2).0;
            }
            if len2 == 0 {
                return median(&nums1).0;
            }

            let (med1, mid1) = median(&nums1);
            let (med2, mid2) = median(&nums2);

            if med1 == med2 {
                return med1;
            }

            if len1 <= 2 || len2 <= 2 {
                let mut nums = nums1;
                nums.extend(nums2);
                nums.sort();
                return median(&nums).0;
            }

            let chop = if mid1 < mid2 { mid1 } else { mid2 };
            if med1 < med2 {
                nums1.drain(..chop);
                nums2.drain((len2 - chop)..);
            } else {
                nums1.drain((len1 - chop)..);
                nums2.drain(..chop);
            }
        }
    }
}
