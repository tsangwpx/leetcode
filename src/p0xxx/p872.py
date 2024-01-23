from leetcode_prelude import *


# Problem 872
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def leafSimilar(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> bool:
        def dfs(node, seq):
            if node is None:
                return

            is_leaf = node.left is None and node.right is None
            if is_leaf:
                seq.append(node.val)
            else:
                dfs(node.left, seq)
                dfs(node.right, seq)

        seq1 = []
        dfs(root1, seq1)

        seq2 = []
        dfs(root2, seq2)

        return seq1 == seq2
