//Author lawakesh mule
//email id: 

// Function to check if a string is a palindrome
fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

// Function to find the index of the first occurrence of a number in a sorted array
fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

// Function to find the shortest word in a string
fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|w| w.len())
}

// Function to check if a number is prime
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Function to find the median of a sorted array
fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        (arr[mid_left] + arr[mid_right]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

// Function to find the longest common prefix of a set of strings
fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let mut prefix = String::new();
    let mut iter = strings.iter().map(|s| s.chars());
    while let Some(ch) = iter.next().unwrap().next() {
        if iter.clone().all(|mut chars| chars.next() == Some(ch)) {
            prefix.push(ch);
        } else {
            break;
        }
    }
    prefix
}

// Function to find the kth smallest element in an array
fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort();
        Some(sorted_arr[k - 1])
    } else {
        None
    }
}

// Definition of a binary tree node
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// Function to calculate the maximum depth of a binary tree
fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

// Function to reverse a string
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// Function to check if a number is prime
fn is_prime_rust(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Function to merge two sorted arrays
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&arr1[i..]);
    result.extend_from_slice(&arr2[j..]);
    result
}

// Function to find the maximum subarray sum
fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = 0;
    let mut current_sum = 0;
    for &num in arr {
        current_sum = (current_sum + num).max(0);
        max_sum = max_sum.max(current_sum);
    }
    max_sum
}

fn main() {
    // Example usage of implemented functions
    println!("Is 'radar' a palindrome? {}", is_palindrome("radar"));
    println!("Index of first occurrence of 5: {:?}", first_occurrence(&[1, 2, 3, 4, 5, 5, 6], 5));
    println!("Shortest word in 'hello world': {:?}", shortest_word("hello world"));
    println!("Is 17 prime? {}", is_prime(17));
    println!("Median of [1, 3, 5, 7, 9]: {}", median(&[1, 3, 5, 7, 9]));
    println!("Longest common prefix: {}", longest_common_prefix(&["flower", "flow", "flight"]));
    println!("3rd smallest element: {:?}", kth_smallest(&[4, 2, 1, 3, 5], 3));
    
    // Example of a binary tree and finding its depth
    let root = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode::new(9))),
        right: Some(Box::new(TreeNode {
            val: 20,
            left: Some(Box::new(TreeNode::new(15))),
            right: Some(Box::new(TreeNode::new(7))),
        })),
    }));
    println!("Maximum depth of the binary tree: {}", max_depth(root));

    println!("Reverse of 'hello': {}", reverse_string("hello"));
    println!("Is 23 prime? {}", is_prime_rust(23));
    println!("Merged sorted arrays: {:?}", merge_sorted_arrays(&[1, 3, 5], &[2, 4, 6]));
    println!("Maximum subarray sum: {}", max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]));
}
