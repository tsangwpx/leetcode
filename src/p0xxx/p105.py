from typing import List, Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        indexes = {s: i for i, s in enumerate(inorder)}

        def make_node(pre_start: int, pre_stop: int, in_start: int, in_stop: int) -> Optional[TreeNode]:
            if pre_start == pre_stop:
                return None

            val = preorder[pre_start]
            in_idx = indexes[val]
            left_size = in_idx - in_start
            right_size = in_stop - in_idx - 1

            left_pre_start = pre_start + 1
            left_in_start = in_start

            right_pre_start = left_pre_start + left_size
            right_in_start = in_idx + 1

            left = make_node(left_pre_start, left_pre_start + left_size, left_in_start, left_in_start + left_size)
            right = make_node(right_pre_start, right_pre_start + right_size, right_in_start, right_in_start + right_size)

            node = TreeNode(val, left, right)
            return node

        return make_node(0, len(preorder), 0, len(preorder))


def main():
    print(Solution().isPossible([1, 1000000000]))
    # return
    print(Solution().isPossible([2, 4, 6, 7, 8]))
    print(Solution().isPossible([1, 2, 3, 4, 5]))
    print(Solution().isPossible([9, 3, 5]))
    print(Solution().isPossible([1, 1, 1, 2]))
    print(Solution().isPossible([8, 5]))


if __name__ == '__main__':
    main()
