from leetcode_prelude import *


# Problem 328
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def oddEvenList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode(next=head)
        odd_origin = odd_tail = ListNode()
        even_origin = even_tail = ListNode()

        while dummy.next is not None:
            odd_node = dummy.next
            even_node = odd_node.next if odd_node.next is not None else None
            dummy.next = even_node.next if even_node is not None else None

            odd_tail.next = odd_node
            odd_tail = odd_node

            even_tail.next = even_node
            even_tail = even_node  # may be None

        odd_tail.next = even_origin.next

        return odd_origin.next
