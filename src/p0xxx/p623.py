from leetcode_prelude import *


# Problem 623
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def addOneRow(
        self,
        root: Optional[TreeNode],
        val: int,
        depth: int,
    ) -> Optional[TreeNode]:
        root = TreeNode(left=root)

        parents = [root]
        for _ in range(depth - 1):
            new_nodes = []

            for node in parents:
                if node.left is not None:
                    new_nodes.append(node.left)
                if node.right is not None:
                    new_nodes.append(node.right)

            parents = new_nodes

        # print([s.val for s in parents])

        for node in parents:
            node.left = TreeNode(val=val, left=node.left)
            node.right = TreeNode(val=val, right=node.right)

        return root.left
