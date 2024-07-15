from leetcode_prelude import *


# Problem 2487
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def removeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        def rev_list(head: Optional[ListNode]) -> Optional[ListNode]:
            curr = head
            prev = None
            next_ = None

            while curr is not None:
                next_ = curr.next
                curr.next = prev
                prev = curr
                curr = next_

            return prev

        def filter_list(head: Optional[ListNode]) -> Optional[ListNode]:
            curr = head
            value = curr.val

            while curr.next is not None:
                next_: ListNode = curr.next
                if next_.val < value:
                    curr.next = next_.next
                else:
                    value = next_.val
                    curr = next_

            return head

        head = rev_list(head)
        head = filter_list(head)
        head = rev_list(head)
        return head
