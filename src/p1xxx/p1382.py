from leetcode_prelude import *


# Problem 1382
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def balanceBST(self, root: TreeNode) -> TreeNode:
        """
        Check out DSW algorithm for an inplace-modification solution.
        """

        def sorted_list(result: list[int], node: TreeNode | None):
            # in order bfs
            if node is None:
                return

            sorted_list(result, node.left)
            result.append(node.val)
            sorted_list(result, node.right)

        items = []
        sorted_list(items, root)

        def create_bst(items: list[int], start: int, stop: int) -> TreeNode | None:
            mid = (start + stop) // 2

            if mid >= stop:
                return None

            return TreeNode(
                items[mid],
                create_bst(items, start, mid),
                create_bst(items, mid + 1, stop),
            )

        return create_bst(items, 0, len(items))
