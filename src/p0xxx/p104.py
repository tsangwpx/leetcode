from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# Problem 104
class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        def visit(node):
            if node is None:
                return 0
            return 1 + max(visit(node.left), visit(node.right))
        return visit(root)
