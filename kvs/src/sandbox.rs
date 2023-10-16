// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut dp = vec![false; s.len() + 1];

    dp[0] = true;

    for i in 1..=s.len() {
        for word in word_dict.iter() {
            if i >= word.len() && s[i - word.len()..i] == *word {
                dp[i] = dp[i] || dp[i - word.len()];
            }
        }
    }

    dp[s.len()]
}
