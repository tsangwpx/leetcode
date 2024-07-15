from leetcode_prelude import *


# Problem 2816
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def doubleIt(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head.val >= 5:
            head = ListNode(val=0, next=head)

        node = head

        while node.next is not None:
            node.val = (node.val * 10 + node.next.val) * 2 // 10 % 10
            node = node.next

        node.val = node.val * 2 % 10

        return head

    def doubleIt2(self, head: Optional[ListNode]) -> Optional[ListNode]:
        size = 0
        num = 0

        node = head
        while node is not None:
            size += 1
            num = num * 10 + node.val
            node = node.next

        if num == 0:
            return head

        num *= 2
        div = 10**size
        if num >= div:
            head = ListNode(next=head)
        else:
            div //= 10

        node = head
        while node is not None:
            q, r = divmod(num, div)
            div //= 10
            node.val = q
            num = r
            node = node.next

        return head
