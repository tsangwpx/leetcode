from leetcode_prelude import *


# Problem 1609
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isEvenOddTree(self, root: Optional[TreeNode]) -> bool:
        row = [root]
        even = True
        while True:
            values = [s.val for s in row]

            if even:
                valid = all(s % 2 == 1 for s in values) and all(
                    a < b for a, b in zip(values[:-1], values[1:])
                )
            else:
                valid = all(s % 2 == 0 for s in values) and all(
                    a > b for a, b in zip(values[:-1], values[1:])
                )

            if not valid:
                return False

            next_rows = []
            for node in row:
                if node.left is not None:
                    next_rows.append(node.left)
                if node.right is not None:
                    next_rows.append(node.right)

            row = next_rows
            even = not even

            if not row:
                break

        return True
