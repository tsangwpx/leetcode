from leetcode_prelude import *


# Problem 129
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def sumNumbers(self, root: Optional[TreeNode]) -> int:
        total = 0

        def visit(node, prefix):
            nonlocal total

            if node is None:
                return

            prefix = prefix * 10 + node.val
            is_leaf = node.left is None and node.right is None
            if is_leaf:
                total += prefix
            else:
                visit(node.left, prefix)
                visit(node.right, prefix)

        visit(root, 0)

        return total

        pass
