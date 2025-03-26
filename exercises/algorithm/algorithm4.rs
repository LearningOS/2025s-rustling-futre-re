/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match &mut self.root {
            // 如果树为空，创建新的根节点
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }
            // 如果树不为空，则在现有树中插入
            Some(node) => {
                node.insert(value);
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        match &self.root {
            None => false, // 空树，找不到值
            Some(node) => {
                // 从根节点开始递归搜索
                let mut current = node;
                
                loop {
                    match value.cmp(&current.value) {
                        Ordering::Equal => return true, // 找到了值
                        Ordering::Less => {
                            // 如果要查找的值小于当前节点，向左子树搜索
                            match &current.left {
                                None => return false, // 左子树为空，找不到值
                                Some(left) => current = left, // 继续在左子树搜索
                            }
                        }
                        Ordering::Greater => {
                            // 如果要查找的值大于当前节点，向右子树搜索
                            match &current.right {
                                None => return false, // 右子树为空，找不到值
                                Some(right) => current = right, // 继续在右子树搜索
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Equal => {
                // 二叉搜索树通常不允许重复值，这里简单忽略
                return;
            }
            Ordering::Less => {
                // 如果值小于当前节点，插入到左子树
                match &mut self.left {
                    None => {
                        // 左子树为空，创建新节点
                        self.left = Some(Box::new(TreeNode::new(value)));
                    }
                    Some(node) => {
                        // 左子树不为空，递归插入
                        node.insert(value);
                    }
                }
            }
            Ordering::Greater => {
                // 如果值大于当前节点，插入到右子树
                match &mut self.right {
                    None => {
                        // 右子树为空，创建新节点
                        self.right = Some(Box::new(TreeNode::new(value)));
                    }
                    Some(node) => {
                        // 右子树不为空，递归插入
                        node.insert(value);
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


