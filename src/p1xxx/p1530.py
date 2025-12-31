from leetcode_prelude import *


# Problem 1530
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def countPairs(
        self,
        root: TreeNode,
        distance: int,
    ) -> int:
        res = 0

        def dfs(node: TreeNode | None) -> list[int]:
            nonlocal res

            if node is None:
                return [0] * (distance + 1)

            if node.left is None and node.right is None:
                children = [0] * (distance + 1)
                children[1] += 1
                return children

            left = dfs(node.left)
            right = dfs(node.right)
            count = 0

            for x in range(distance + 1):
                # x = 0...distance
                for y in range(distance - x + 1):
                    # y = 0 .. distance - x
                    count += left[x] * right[y]

            children = [0] * (distance + 1)

            for i in range(1, distance + 1):
                children[i] += left[i - 1]
                children[i] += right[i - 1]

            # print(node.val, count, left, right)
            res += count

            return children

        dfs(root)

        return res
