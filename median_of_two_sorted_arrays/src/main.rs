mod median_of_two_sorted_arrays;
use median_of_two_sorted_arrays::Solution;

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    let result = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("expect 2 = {}", result);

    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    let result = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("expect 2.5 = {}", result);

    let nums1 = vec![1, 2];
    let nums2 = vec![-1, 3];
    let result = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("expect 1.5 = {}", result);

    let nums1 = vec![1, 2];
    let nums2 = vec![1, 2, 3];
    let result = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("expect 2 = {}", result);

    let nums1 = vec![1, 5, 6];
    let nums2 = vec![2, 3, 4, 7, 8];
    let result = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("expect 4.5 = {}", result);
}
