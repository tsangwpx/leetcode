from leetcode_prelude import *


# Problem 637
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def averageOfLevels(self, root: Optional[TreeNode]) -> List[float]:
        sum_and_counts = []

        def visit(node, depth):
            if node is None:
                return

            if depth == len(sum_and_counts):
                sum_and_counts.append((node.val, 1))
            else:
                acc, count = sum_and_counts[depth]
                sum_and_counts[depth] = (acc + node.val, count + 1)

            visit(node.left, depth + 1)
            visit(node.right, depth + 1)

        visit(root, 0)

        return [s / c for s, c in sum_and_counts]
