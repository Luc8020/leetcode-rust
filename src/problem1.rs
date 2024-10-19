use rand::Rng;
use std::collections::BinaryHeap;

pub fn start() {
    print!("This is the problem: \n
     Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
     The overall run time complexity should be O(log (m+n)).\n");

     let list1 = random_sorted_list(120);
     let list2 = random_sorted_list(100);
     println!("\nThe random generated lists are:\nList 1: {:?}\n \n List 2: {:?} \n", list1, list2);
     println!("Median: {}", find_median_sorted_arrays(&list1, &list2));
}

fn random_sorted_list(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();

    for _ in 0..n {
        let num = rng.gen_range(0..10000);
        heap.push(num);
    }

    heap.into_sorted_vec()
}


fn find_median_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> f64 {
    let (m, n) = (nums1.len(), nums2.len());
    
    if m > n {
        return find_median_sorted_arrays(nums2, nums1);
    }

    let total = m + n;
    let half = (total + 1) / 2;
    
    let mut left = 0;
    let mut right = m;

    while left <= right {
        let i = (left + right) / 2; 
        let j = half - i;           

        let nums1_left = if i == 0 { i32::MIN } else { nums1[i-1] };
        let nums1_right = if i == m { i32::MAX } else { nums1[i] };
        let nums2_left = if j == 0 { i32::MIN } else { nums2[j-1] };
        let nums2_right = if j == n { i32::MAX } else { nums2[j] };

        if nums1_left <= nums2_right && nums2_left <= nums1_right {
            if total % 2 == 0 {
                return (nums1_left.max(nums2_left) as f64 + 
                       nums1_right.min(nums2_right) as f64) / 2.0;
            } else {
                return nums1_left.max(nums2_left) as f64;
            }
        } else if nums1_left > nums2_right {
            right = i - 1;
        } else {
            left = i + 1;
        }
    }

    0.0
}