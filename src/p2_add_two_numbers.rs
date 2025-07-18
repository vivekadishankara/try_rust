// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn to_list(vector: &Vec<i32>) -> Option<Box<ListNode>>{
    let mut curr = None;
    for &i in vector.iter().rev() {
        let mut new_node = ListNode::new(i);
        new_node.next = curr;
        curr = Some(Box::new(new_node));
    }
    curr
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut carry = 0;
        let mut result = None;
        let mut curr = &mut result;


        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += (&node).val;
                l1 = &(&node).next;
            }

            if let Some(node) = l2 {
                sum += (&node).val;
                l2 = &(&node).next;
            }
            carry = sum / 10;
            *curr = Some(Box::new(ListNode::new(sum%10)));
            curr = &mut curr.as_mut().unwrap().next;
        }
        result
    }
}

#[cfg(test)]
mod tests {
  use crate::p2_add_two_numbers::to_list;

use super::Solution;

  #[test]
  fn test_2() {
    assert_eq!(Solution::add_two_numbers(to_list(&vec![2,4,3]), to_list(&vec![5,6,4])), to_list(&vec![7,0,8]));
    assert_eq!(Solution::add_two_numbers(to_list(&vec![0]), to_list(&vec![0])), to_list(&vec![0]));
    assert_eq!(Solution::add_two_numbers(to_list(&vec![9,9,9,9,9,9,9]), to_list(&vec![9,9,9,9])), to_list(&vec![8,9,9,9,0,0,0,1]));
  }
}