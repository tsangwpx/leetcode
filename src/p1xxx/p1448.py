from leetcode_prelude import *


# Problem 1448
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        def count_good(node, maximum):
            if node is None:
                return 0

            maximum = max(node.val, maximum)
            score = 1 if node.val >= maximum else 0
            score += count_good(node.left, maximum)
            score += count_good(node.right, maximum)
            return score

        return count_good(root, -(10**4))
