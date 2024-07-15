from leetcode_prelude import *


# Problem 237
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None
class Solution:
    def deleteNode(self, node):
        """
        :type node: ListNode
        :rtype: void Do not return anything, modify node in-place instead.
        """
        next_ = node.next
        nnext = next_.next
        node.val = next_.val
        node.next = nnext
