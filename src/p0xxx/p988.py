from leetcode_prelude import *


# Problem 988
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    TABLE = [chr(s + ord("a")) for s in range(0, 26)]

    def smallestFromLeaf(self, root: Optional[TreeNode]) -> str:
        table = self.TABLE

        def dfs(node: TreeNode, suffix: str) -> str:
            suffix = table[node.val] + suffix

            if node.left is None and node.right is None:
                return suffix
            elif node.left is None:
                return dfs(node.right, suffix)
            elif node.right is None:
                return dfs(node.left, suffix)
            else:
                left = dfs(node.left, suffix)
                right = dfs(node.right, suffix)
                print(left, right)
                return min(left, right)

        return dfs(root, "")
