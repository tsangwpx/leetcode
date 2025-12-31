from leetcode_prelude import *


# Problem 2458
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def treeQueries(
        self,
        root: Optional[TreeNode],
        queries: List[int],
    ) -> List[int]:

        # What is N? the number of nodes.
        # height in which a node rooted
        node_heights: list[int] = []

        def dfs(node: TreeNode | None, parent: int) -> int:
            """Compute node_heights; return height rooted at a node"""

            if node is None:
                return 0

            idx = node.val
            size = len(node_heights)
            if idx >= size:
                extra = (-1,) * (idx + 1 - size)
                node_heights.extend(extra)

            h = 0
            if node.left is not None:
                h = max(h, dfs(node.left, idx) + 1)

            if node.right is not None:
                h = max(h, dfs(node.right, idx) + 1)

            node_heights[idx] = h

            return node_heights[idx]

        dfs(root, -1)

        removed_heights = [-1] * len(node_heights)

        def dfs2(node: TreeNode | None, depth: int, removed_height: int) -> None:
            """Compute removed_heights"""
            if node is None:
                return None

            idx = node.val
            removed_heights[idx] = max(0, depth - 1, removed_height)
            # print(idx, depth, removed_height)

            if node.left is not None and node.right is not None:
                left_height = depth + node_heights[node.left.val] + 1
                right_height = depth + node_heights[node.right.val] + 1
                dfs2(node.left, depth + 1, max(removed_height, right_height))
                dfs2(node.right, depth + 1, max(removed_height, left_height))
            else:
                dfs2(node.left, depth + 1, removed_height)
                dfs2(node.right, depth + 1, removed_height)

        dfs2(root, 0, 0)
        # print("height", node_heights)
        # print("removed", removed_heights)

        res = [0] * len(queries)
        for i, removed_idx in enumerate(queries):
            res[i] = removed_heights[removed_idx]

        return res
