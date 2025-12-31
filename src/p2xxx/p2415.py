from leetcode_prelude import *


# Problem 2415
class Solution:
    def reverseOddLevels(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        def dfs(level: int, left: TreeNode | None, right: TreeNode | None) -> None:
            if left is None or right is None:
                return

            if level % 2 != 0:
                left.val, right.val = right.val, left.val

            dfs(level + 1, left.left, right.right)
            dfs(level + 1, left.right, right.left)

        dfs(1, root.left, root.right)

        return root
