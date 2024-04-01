// 1. Implement a function that checks whether a given string is a palindrome or not.
fn is_palindrome(s: &str) -> bool {
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

// 2. Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.
fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

// 3. Given a string of words, implement a function that returns the shortest word in the string.
fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

// 4. Implement a function that checks whether a given number is prime or not.
fn is_prime(n: u64) -> bool {
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

// 5. Given a sorted array of integers, implement a function that returns the median of the array.
fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

// 6. Implement a function that finds the longest common prefix of a given set of strings.
fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let first_str = &strs[0];
    let mut prefix = String::new();
    'outer: for (i, c) in first_str.chars().enumerate() {
        for s in strs.iter().skip(1) {
            if let Some(sc) = s.chars().nth(i) {
                if sc != c {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        prefix.push(c);
    }
    prefix
}

// 7. Implement a function that returns the kth smallest element in a given array.
fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    sorted_arr.get(k - 1).cloned()
}

// 8. Given a binary tree, implement a function that returns the maximum depth of the tree.
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
    
    fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
        match root {
            None => 0,
            Some(node) => 1 + Self::max_depth(node.left).max(Self::max_depth(node.right)),
        }
    }
}

// 9. Reverse a string in Rust
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// 10. Check if a number is prime in Rust
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

// 11. Merge two sorted arrays in Rust
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&arr1[i..]);
    merged.extend_from_slice(&arr2[j..]);
    merged
}

// 12. Find the maximum subarray sum in Rust
fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_ending_here = 0;
    let mut max_so_far = i32::MIN;
    for &num in arr {
        max_ending_here = max_ending_here.max(0) + num;
        max_so_far = max_so_far.max(max_ending_here);
    }
    max_so_far
}

fn main() {
    // Test cases
    assert_eq!(is_palindrome("racecar"), true);
    assert_eq!(first_occurrence(&[1, 2, 3, 4, 5], 3), Some(2));
    assert_eq!(shortest_word("The quick brown fox jumps"), Some("fox"));
    assert_eq!(is_prime(17), true);
    assert_eq!(median(&[1, 2, 3, 4, 5]), 3.0);
    assert_eq!(longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl");
    assert_eq!(kth_smallest_element(&[4, 2, 1, 3, 5], 3), Some(3));
    let root = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode::new(9))),
        right: Some(Box::new(TreeNode {
            val: 20,
            left: Some(Box::new(TreeNode::new(15))),
            right: Some(Box::new(TreeNode::new(7))),
        })),
    }));
    assert_eq!(TreeNode::max_depth(root), 3);
    assert_eq!(reverse_string("Hello"), "olleH");
    assert_eq!(is_prime_rust(17), true);
    assert_eq!(merge_sorted_arrays(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
}
