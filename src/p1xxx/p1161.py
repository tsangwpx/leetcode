from leetcode_prelude import *


# Problem 1161
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def maxLevelSum(self, root: Optional[TreeNode]) -> int:
        nodes = [root]
        children = []
        level = 1

        maximal_level = 1
        maximal_value = root.val

        while nodes:
            acc = 0
            for node in nodes:
                acc += node.val

                if node.left is not None:
                    children.append(node.left)

                if node.right is not None:
                    children.append(node.right)

            if acc > maximal_value:
                maximal_level = level
                maximal_value = acc

            nodes, children = children, nodes
            children.clear()
            level += 1

        return maximal_level
