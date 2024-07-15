from leetcode_prelude import *


# Problem 2196
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def createBinaryTree(self, descriptions: List[List[int]]) -> Optional[TreeNode]:
        table: dict[int, TreeNode] = {}
        possible_roots: set[int] = set()

        for parent, child, is_left in descriptions:
            possible_roots.discard(child)

            parent_node = table.get(parent)
            if parent_node is None:
                parent_node = TreeNode(parent)
                possible_roots.add(parent)
                table[parent] = parent_node

            child_node = table.get(child)
            if child_node is None:
                child_node = TreeNode(child)
                table[child] = child_node

            if is_left:
                parent_node.left = child_node
            else:
                parent_node.right = child_node

        assert len(possible_roots) == 1, possible_roots
        root = possible_roots.pop()
        return table[root]
