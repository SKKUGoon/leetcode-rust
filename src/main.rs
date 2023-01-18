use std::collections::HashMap;

#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, n) in nums.iter().enumerate() {
        let rem = target - n;
        let rem_slc = &nums[(i+1)..];
        let index = rem_slc.iter()
            .position(|&r| r == rem);
        match index {
            None => {
                println!("no matching index for {} in {:?}", n, rem_slc);
                continue
            },
            Some(pos) => {
                println!("matching index for {} in {:?}", n, rem_slc);
                return vec![i as i32, (i + 1 + pos) as i32]
            },
        }
    }
    return vec![-1, -1]
}

#[allow(dead_code)]
macro_rules! logarithm {
    ($val: expr, $base: expr, $type: ty) => {
        ($val as f32).log($base) as $type
    };
}

#[allow(dead_code)]
fn get_int_pos(x: i64, pos: i64) -> i64 {
    x % 10i64.pow((pos+1) as u32) / 10i64.pow(pos as u32)
}

#[allow(dead_code)]
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false
    }

    let isize = logarithm!(x, 10f32, i64);
    let iter_thr: Vec<i64> = (0..isize+1).collect();
    let iter_thr_rev: Vec<i64> = iter_thr.clone().into_iter().rev().collect();

    for (f, b) in iter_thr_rev.into_iter().zip(iter_thr) {
        let front = get_int_pos(x as i64, f);
        let back = get_int_pos(x as i64, b);
        if front != back {
            return false
        }
    }
    true
}

#[allow(dead_code)]
pub fn roman_to_int(s: String) -> i32 {
    let mut roman_char = HashMap::from([
        ("I", 1), ("V", 5), ("X", 10), ("L", 50),
        ("C", 100), ("D", 500), ("M", 1000),
    ]);

    let mut res = 0i32;
    for i in s.chars().rev().into_iter() {
        let i_str = &i.to_string()[..];
        match i_str {
            "V" => roman_char.insert("I", -1),
            "X" => roman_char.insert("I", -1),
            "L" => roman_char.insert("X", -10),
            "C" => roman_char.insert("X", -10),
            "D" => roman_char.insert("C", -100),
            "M" => roman_char.insert("C", -100),
            _ => None,
        };
        
        let roman_num = roman_char.get(i_str).unwrap();
        
        res += roman_num;
    }
    return res
}

#[allow(dead_code)]
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 1 {
        return strs[0].to_owned()
    }

    let standard = &strs[0];

    let mut res = 0;
    let mut eof = false;
    // a ab
    for j in 0..standard.len() {  // 0
        let std_c = standard.chars().nth(j).unwrap();  // a
        
        for i in strs.iter() {
            let c = i.chars().nth(j);
            match c {
                None => {
                    eof = true;
                    break
                },
                Some(val) => {
                    if val != std_c {
                        eof = true;
                        break
                    };
                }
            }
        }
        
        if eof {
            res = j;
            break 
        } else {
            res = j + 1;
        };
    }
    return standard[0..res].to_owned()
}

#[allow(dead_code)]
pub fn is_valid(s: String) -> bool {
    if s.len() <= 1 {
        return false
    }

    let bracket_pair = HashMap::from([
        (')', '('), ('}', '{'), (']', '['),
    ]);
    let mut bracket_open = HashMap::from([
        ('(', 0), ('{', 0), ('[', 0)
    ]);
    let mut open_q = vec!['s'];
    
    for i in s.chars().into_iter() {
        let is_open = bracket_open.get_mut(&i);
        match is_open {
            None => {
                // It's a closing bracket. Find its `starting pair` 
                let my_pair = bracket_pair.get(&i).unwrap();
                // pop from open_q and compare
                let recent = open_q.pop().unwrap();
                
                if recent == 's' {
                    // never even opened
                    return false
                } else if recent != *my_pair {
                    // wrong closing order
                    return false
                } else {
                    // log minus one
                    let opened = bracket_open.get(my_pair).unwrap();
                    if *opened == 0 {
                        // too much closing
                        return false
                    }
                    *bracket_open.get_mut(my_pair).unwrap() += -1;
                }
            },
            Some(v) => {
                // It's a starting bracket. 
                // log plus one, and log character into the queue
                *v += 1;
                open_q.push(i);
            }
        }
    }
    if open_q.len() > 1 {
        return false
    }
    return true
}

// 21. Unable to solve
// List of List - 
// How to solve Option<Box<...>>


// Basic Memoization
#[allow(dead_code)]
fn fib(n: u128) -> u128 {
    if n <=1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2)
    }
}

#[allow(dead_code)]
fn fib_memo(n: u128) -> u128 {
    let mut memo: Vec<u128> = Vec::new();
    for i in 0..(n+1) {
        if i <= 1 {
            memo.push(1);
            continue
        }
        let fib_num = memo[i as usize -1] + memo[i as usize - 2];
        memo.push(fib_num);
    }

    return memo.pop().unwrap()
}

#[allow(dead_code)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // Comb through sorted list, save each number
    // If the number is the same remove
    let mut isdup: Option<&i32> = None;
    let smaller = nums[0] - 1;
    let mut changed = 0i32;
    for i in nums.iter_mut() {
        match isdup {
            None => {
                isdup = Some(i);
                changed += 1
            },
            Some(val) => {
                if i != val {
                    // Different number. Update `isdup`
                    isdup = Some(i);
                    changed += 1
                } else {
                    // Same number. Update it with `smaller`
                    *i = smaller;
                }
            }
        }
    }

    // Duplicate numbers are all transformed into `smaller`
    // Sort and erase
    nums.sort();
    nums.retain(|&x| x != smaller);
    return changed

    // There's a dedicated function!
    // nums.dedup();
    // nums.len() as i32
}

#[allow(dead_code)]
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    return nums.len() as i32
}

#[allow(dead_code)]
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    for (i, val) in nums.iter().enumerate() {
        if val == &target {
            return i as i32
        } else if val > &target {
            return i as i32
        } 
    };
    return nums.len() as i32
}

#[allow(dead_code)]
pub fn length_of_last_word(s: String) -> i32 {
    let mut continuous = 0i32;
    let mut blank = false;
    for i in s.chars().into_iter() {
        if i == ' ' {
            blank = true;
        } else {
            if blank {
                continuous = 0;
            }
            blank = false;
            continuous += 1;
        }
    }
    return continuous
}

#[allow(dead_code)]
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut push_one = 0usize;
    let mut result: Vec<i32> = Vec::new();
    for (i, val) in digits.iter().rev().enumerate() {
        if i == push_one {
            if val + 1 == 10 {
                result.push(0);
                push_one += 1 as usize;
            } else {
                result.push(val + 1);
            }
            continue
        }
        result.push(*val);
    }
    if push_one == result.len() {
        result.push(1);
    }
    return result.into_iter().rev().collect::<Vec<i32>>()
}

#[allow(dead_code)]
pub fn my_sqrt(x: i32) -> i32 {
    if x <= 1 {
        return x
    }
    let mut i = 1u128;
    let mut sq = i * i;
    while x as u128 >= sq {
        i += 1;
        sq = i * i;
    }
    return (i - 1) as i32
}

// Time efficient answer
// pub fn my_sqrt(x: i32) -> i32 {
//     let (mut low, mut high) = (0_u64, x as u64 / 2 + 1);
//     while low < high {
//         let mid = low + (high - low + 1) / 2;
//         if mid * mid > x as u64 { high = mid - 1; } else { low = mid; }
//     }

//     low as i32
// }


#[allow(dead_code)]
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut res: Vec<i32> = Vec::new();
    let (mut id1, mut id2) = (0i32, 0i32);
    loop {
        if id1 == m {
            // end of nums1. push remaining elements of nums2 to result
            res.extend_from_slice(&nums2[id2 as usize..n as usize]);
            break
        } 
        if id2 == n {
            res.extend_from_slice(&nums1[id1 as usize..m as usize]);
            break
        }
        if nums1[id1 as usize] < nums2[id2 as usize] {
            res.push(nums1[id1 as usize]);
            id1 += 1;
        } else {
            res.push(nums2[id2 as usize]);
            id2 += 1;
        }


    }
    println!("{:?}", res);
    for (i, v) in res.iter_mut().enumerate() {
        nums1[i] = *v;
    }
    println!("{:?}", nums1);
}

// Efficient looking answer
// pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
//     nums1.truncate(m as usize);
//     nums1.extend_from_slice(nums2);
//     nums1.sort();
// }

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
#[allow(dead_code)]
use std::rc::Rc;
use std::cell::RefCell;
// impl Solution {
#[allow(dead_code)]
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn h(node: &Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
        if let Some(v) = node {
            let v = v.borrow();

            // traverse left
            h(&v.left, ret);
            // push root
            ret.push(v.val);
            // traverse right
            h(&v.right, ret);
        }
    }

    let mut ret: Vec<i32> = Vec::new();
    if let Some(v) = root {
        h(&Some(v), &mut ret);
    }
    return ret
}
// }

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn h(node: &Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<String>, rl: char) {
        // if let Enum Component = Enum
        if let Some(v) = node {
            let v = v.borrow();
            h(&v.left, ret, 'l');
            let val = format!("{}{}", rl, v.val);
            ret.push(val);
            h(&v.right, ret, 'r');
        } 
    }
    let mut ret1: Vec<String> = Vec::new();
    let mut ret2: Vec<String> = Vec::new();
    if let Some(v) = p {
        h(&Some(v), &mut ret1, 'c');
    };
    if let Some(v) = q {
        h(&Some(v), &mut ret2, 'c');
    };

    if ret1.len() != ret2.len() {
        return false
    };

    // Check ret1 -> ret2
    for i in ret1.iter().zip(&ret2) {
        let (ele1, ele2) = i;
        if ele1 != ele2 {
            return false
        }
    };

    return true
}

#[allow(dead_code)]
fn if_lets() {
    // If `match` is exhaustive to write, we can use if let
    let number = Some(7);
    let letter: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}", i);
    }
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number");
    }
    // in the same way it can match any enums
}

pub fn sum_index_generate(target: usize) -> Vec<Vec<usize>> {
    let s: Vec<usize> = (1..=target).collect();
    let mut result: Vec<Vec<usize>> = Vec::new();
    for i in s.into_iter().enumerate() {
        result.push(vec![i.0, i.1]);
    }
    return result
}

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut memo: Vec<Vec<i32>> = Vec::new();
    for i in 0..num_rows {
        if i == 0 {
            memo.push(vec![1])
        } else if i == 1 {
            memo.push(vec![1, 1])
        } else {
            let sum_idx = sum_index_generate((i-1) as usize);
            let bef = &memo[(i-1) as usize];
            
            // Iterate
            let mut row_result = vec![1];
            for idx in sum_idx.iter() {
                println!("{:?}", idx);
                row_result.push(bef[idx[0]] + bef[idx[1]]);
            }
            row_result.push(1);

            memo.push(row_result);
        }
    }
    return memo
}

use std::cmp;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut current_min: Option<&i32> = None;
    let mut profit: i32 = 0;
    for stock in prices.iter() {
        let prc = stock;
        match current_min {
            None => {
                current_min = Some(prc);
                profit = cmp::max(profit, 0);
            },
            Some(p) => {
                if p > prc {
                    // found a smaller price
                    current_min = Some(prc);
                    profit = cmp::max(profit, 0);
                } else {
                    profit = cmp::max(profit, prc-p);
                }
            }
        }
    }
    return profit
}


fn main() {
    // Tree problem memo
    // let mut tree1 = TreeNode::new(1);
    // let tree1l = TreeNode::new(1);
    // tree1.left = Some(Rc::new(RefCell::new(tree1l)));
    
    // let mut tree2 = TreeNode::new(1);
    // let tree2r = TreeNode::new(1);
    // tree2.right = Some(Rc::new(RefCell::new(tree2r)));
    
    let test1 = vec![7, 1, 5, 3, 6, 4];
    let test2 = vec![7, 2, 5, 3, 6, 1];
    let test3 = vec![7, 6, 5, 4, 4, 3];
    let t1 = max_profit(test1);
    let t2 = max_profit(test2);
    let t3 = max_profit(test3);
    println!("{:?}", t1);
    println!("{:?}", t2);
    println!("{:?}", t3);
}
