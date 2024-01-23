from leetcode_prelude import *


# Problem 112
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def hasPathSum(self, root: Optional[TreeNode], targetSum: int) -> bool:
        if root is None:
            return False

        targetSum -= root.val

        if root.left is None and root.right is None:
            return targetSum == 0

        ok = False
        if not ok and root.left is not None:
            ok = self.hasPathSum(root.left, targetSum)

        if not ok and root.right is not None:
            ok = self.hasPathSum(root.right, targetSum)

        return ok
