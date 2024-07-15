from leetcode_prelude import *


# Problem 1457
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def pseudoPalindromicPaths(self, root: Optional[TreeNode]) -> int:
        def dfs(node: Optional[TreeNode], depth: int, seen: int):
            if node is None:
                return 0

            bit = 1 << node.val
            seen ^= bit
            count = dfs(node.left, depth + 1, seen)
            count += dfs(node.right, depth + 1, seen)

            # print(f"{node.val} {depth} {count} {seen}")

            if node.left is None and node.right is None:
                if depth % 2 == 0:
                    if seen == 0:
                        count += 1
                else:
                    if seen.bit_count() == 1:
                        count += 1

            return count

        return dfs(root, 1, 0)
