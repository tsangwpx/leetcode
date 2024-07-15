from leetcode_prelude import *


# Problem 19
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        dummy = ListNode(next=head)

        node = dummy.next
        for _ in range(n):
            # Since len(head) <= n, node must not be None
            node = node.next

        pivot = dummy
        while node is not None:
            node = node.next
            pivot = pivot.next

        removed = pivot.next
        pivot.next = removed.next

        return dummy.next
