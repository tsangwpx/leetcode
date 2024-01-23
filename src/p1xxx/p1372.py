from leetcode_prelude import *


# Problem 1372
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def longestZigZag(self, root: Optional[TreeNode]) -> int:
        longest = 0

        def dfs(node, dist):
            # If dist < 0, the next node of this node is left
            # abs(dist) is the longest path if left exist!
            # If dist > 0, the next node is on right
            nonlocal longest
            if node is None:
                return

            if node.left is None:
                longest = max(longest, abs(dist) - 1)
            else:
                if dist < 0:
                    dfs(node.left, -dist + 1)
                else:
                    dfs(node.left, 2)

            if node.right is None:
                longest = max(longest, abs(dist) - 1)
            else:
                if dist < 0:
                    dfs(node.right, -2)
                else:
                    dfs(node.right, -dist - 1)

        dfs(root, 1)
        dfs(root, -1)

        return longest
