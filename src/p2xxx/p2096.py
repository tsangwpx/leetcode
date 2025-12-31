from leetcode_prelude import *


# Problem 2096
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def getDirections(
        self,
        root: Optional[TreeNode],
        startValue: int,
        destValue: int,
    ) -> str:
        def dfs(node: TreeNode | None, path: list[str], target: int) -> bool:
            if node is None:
                return False

            if node.val == target:
                return True

            path.append("L")
            done = dfs(node.left, path, target)
            if done:
                return done

            path[-1] = "R"
            done = dfs(node.right, path, target)
            if done:
                return done

            path.pop()
            return False

        start_path = []
        dfs(root, start_path, startValue)

        dest_path = []
        dfs(root, dest_path, destValue)

        for skip, (left, right) in enumerate(zip(start_path, dest_path)):
            if left != right:
                break
        else:
            skip = min(len(start_path), len(dest_path))

        start_path = start_path[skip:]
        dest_path = dest_path[skip:]

        res = "U" * len(start_path)
        res += "".join(dest_path)

        return res
