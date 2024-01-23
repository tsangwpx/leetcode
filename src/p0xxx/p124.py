from typing import Optional

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

# Problem 124
class Solution:
    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        def recurse(node):
            if node.left is not None:
                left_partial, left_max = recurse(node.left)
            else:
                left_partial = left_max = -1000
            
            if node.right is not None:
                right_partial, right_max = recurse(node.right)
            else:
                right_partial = right_max = -1000
            
            node_partial = max(node.val, left_partial + node.val, right_partial + node.val)
            node_max = max(node_partial, left_max, right_max, left_partial + node.val + right_partial)

            return node_partial, node_max

        _, maximum = recurse(root)
        return maximum