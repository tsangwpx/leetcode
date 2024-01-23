from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# Problem 543
class Solution:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        def dfs(node):
            if node is None:
                return 0, 0

            legs = 0

            if node.left is None:
                left_max = left_partial = 0
            else:
                left_max, left_partial = dfs(node.left)
                legs += 1

            if node.right is None:
                right_max = right_partial = 0
            else:
                right_max, right_partial = dfs(node.right)
                legs += 1

            if legs == 0:
                return 0, 0

            node_partial = 1 + max(left_partial, right_partial)
            node_max = max(
                left_max,
                right_max,
                legs + left_partial + right_partial,
            )

            return node_max, node_partial

        return dfs(root)[0]
