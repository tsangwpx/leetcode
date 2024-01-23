from leetcode_prelude import *


# Problem 82
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        head = ListNode(val=None, next=head)

        prev = head

        while prev.next is not None:
            node = prev.next
            val = node.val

            next_node = node.next

            if next_node is None or next_node.val != val:
                prev = prev.next
                continue

            while next_node is not None and next_node.val == val:
                next_node = next_node.next

            prev.next = next_node

        return head.next
