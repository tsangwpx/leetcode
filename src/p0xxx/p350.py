from leetcode_prelude import *


# Problem 450
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def deleteNode(self, root: Optional[TreeNode], key: int) -> Optional[TreeNode]:
        if root is None:
            return None

        if root.val > key:
            root.left = self.deleteNode(root.left, key)
            return root
        elif root.val < key:
            root.right = self.deleteNode(root.right, key)
            return root

        # When at most one child exist,
        if root.left is None:
            return root.right
        elif root.right is None:
            return root.left

        # Find successor from the right subtree
        # which is the smallest node of the subtree
        parent = root
        successor = root.right
        while successor.left is not None:
            parent = successor
            successor = successor.left

        # Instead of delete root
        # Replace root's val with the successor's
        root.val = successor.val

        # In case the successor is root's direct children
        # Replace root's right node
        # otherwise, replace the left node of the parent of the successor
        if parent is root:
            parent.right = successor.right
        else:
            parent.left = successor.right

        return root
