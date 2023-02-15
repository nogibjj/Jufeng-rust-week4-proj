pub fn find_kth_smallest_element(nums1: &Vec<i32>, nums2: &Vec<i32>, left1: usize, left2: usize, k: usize) -> i32 {
    if nums1.len() == left1 {
        return nums2[left2 + k - 1];
    } else if nums2.len() == left2 {
        return nums1[left1 + k - 1];
    }
    if k == 1 {
        return nums1[left1].min(nums2[left2]);
    }
    let a = (k / 2) as i32;
    let b = nums1.len() as i32 - left1 as i32;
    let c = nums2.len() as i32 - left2 as i32;
    let removed_element_num = a.min(b.min(c)) as usize;
    if nums1[left1 + removed_element_num - 1] < nums2[left2 + removed_element_num - 1] {
        find_kth_smallest_element(nums1, nums2, left1 + removed_element_num, left2, k - removed_element_num)
    } else {
        find_kth_smallest_element(nums1, nums2, left1, left2 + removed_element_num, k - removed_element_num)
    }
}


pub fn find_median_sorted_arrays(nums1: &Vec<i32>, nums2: &Vec<i32>) -> f64 {
        let m = nums1.len() as i32;
        let n = nums2.len() as i32;
        if (m + n) % 2 == 0 {
            let x1 = find_kth_smallest_element(nums1, nums2, 0, 0, ((m + n) / 2) as usize);
            let x2 = find_kth_smallest_element(nums1, nums2, 0, 0, ((m + n) / 2 + 1) as usize);
            (x1 as f64 + x2 as f64) / 2.0
        } else {
            find_kth_smallest_element(nums1, nums2, 0, 0, ((m + n) / 2 + 1) as usize) as f64
        }
    }




fn main() {
    let nums1 = vec![1, 3, 5, 7, 9];
    let nums2 = vec![2, 4, 6, 8, 10];
    // print the result of find_median_sorted_arrays
    println!("{}", find_median_sorted_arrays(&nums1, &nums2));
}
