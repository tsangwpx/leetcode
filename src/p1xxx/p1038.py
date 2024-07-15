from leetcode_prelude import *


# Problem 1038
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def bstToGst(
        self,
        root: TreeNode,
    ) -> TreeNode:

        def dfs(node: TreeNode | None, base: int) -> int:
            if node is None:
                return base

            base = dfs(node.right, base)
            base += node.val
            node.val = base

            base = dfs(node.left, base)
            return base

        dfs(root, 0)

        return root
