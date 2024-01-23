from leetcode_prelude import *


# Problem 106
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def buildTree(self, inorder: List[int], postorder: List[int]) -> Optional[TreeNode]:
        def build_node(in_start, in_stop, post_start, post_stop):
            if in_start == in_stop:
                return None

            post_stop -= 1
            val = postorder[post_stop]
            print(in_start, in_stop, post_start, post_stop, val)

            if post_start == post_stop:
                return TreeNode(val=val)

            in_mid = inorder.index(val, in_start, in_stop)
            post_mid = in_mid - in_start + post_start

            left = build_node(in_start, in_mid, post_start, post_mid)
            right = build_node(in_mid + 1, in_stop, post_mid, post_stop)
            return TreeNode(val=val, left=left, right=right)

        return build_node(0, len(inorder), 0, len(postorder))
