from leetcode_prelude import *


# Problem 979
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def distributeCoins(self, root: Optional[TreeNode]) -> int:
        total_move = 0

        def post(node: Optional[TreeNode]) -> int:
            nonlocal total_move

            if node is None:
                return 0

            left_sum = post(node.left)
            right_sum = post(node.right)

            node_sum = node.val + left_sum + right_sum - 1

            total_move += abs(left_sum) + abs(right_sum)

            return node_sum

        post(root)

        return total_move
