from leetcode_prelude import *


# Problem 513
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def findBottomLeftValue(self, root: Optional[TreeNode]) -> int:
        assert root is not None

        prev_rows = [root]

        while True:
            rows = []

            for node in prev_rows:
                if node.left is not None:
                    rows.append(node.left)

                if node.right is not None:
                    rows.append(node.right)

            if rows:
                prev_rows = rows
                continue

            return prev_rows[0].val

        raise AssertionError("unreachable")
