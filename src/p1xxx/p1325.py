from leetcode_prelude import *


# Problem 1325
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def removeLeafNodes(
        self,
        root: Optional[TreeNode],
        target: int,
    ) -> Optional[TreeNode]:

        def dfs(node: Optional[TreeNode]) -> bool:
            if node is None:
                return True

            if node.left is not None and dfs(node.left):
                node.left = None

            if node.right is not None and dfs(node.right):
                node.right = None

            if node.left is None and node.right is None and node.val == target:
                return True

            return False

        removed = dfs(root)
        return None if removed else root
