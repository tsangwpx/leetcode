from leetcode_prelude import *


# Problem 530
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def getMinimumDifference(self, root: Optional[TreeNode]) -> int:
        sorted_valeus = []

        def inorder(node):
            if node is None:
                return
            inorder(node.left)
            sorted_valeus.append(node.val)
            inorder(node.right)

        inorder(root)

        return min(b - a for a, b in zip(sorted_valeus[:-1], sorted_valeus[1:]))
