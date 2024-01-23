from typing import List, Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


# Definition for a Node.
class Node:
    def __init__(self, x: int, next: "Node" = None, random: "Node" = None):
        self.val = int(x)
        self.next = next
        self.random = random


# Problem 148
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def sortList2(self, head: Optional[ListNode]) -> Optional[ListNode]:
        import operator

        hack = []
        node = head
        while node is not None:
            hack.append(node)
            node = node.next
        hack.sort(key=operator.attrgetter("val"))
        head = ListNode()
        tail = head
        for node in hack:
            tail.next = node
            tail = tail.next
        tail.next = None

        return head.next

    def sortList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        def to_list(node):
            res = []
            seen = set()
            while node is not None:
                id_ = id(node)
                if id_ in seen:
                    res.append("cycle")
                    res.append(node.val)
                    break
                seen.add(id_)
                res.append(node.val)
                node = node.next
            return res

        head = ListNode(next=head)

        part_size = 1
        while True:
            new_head = ListNode()
            new_tail = new_head

            rest = ListNode()
            rest.next = head.next

            cmp_count = 0

            while True:
                if rest.next is None:
                    break

                # print("rest", to_list(rest)[0:20])
                # reset fast and slow
                slow = rest
                for _ in range(part_size):
                    slow = slow.next
                    if slow is None:
                        break

                if slow is None or slow.next is None:
                    # left partition is always sorted, right partition is empty
                    # Move all in rest to new_tail
                    new_tail.next = rest.next
                    rest.next = None
                    break

                fast = slow
                for _ in range(part_size):
                    fast = fast.next
                    if fast is None:
                        break

                # split sublist between rest and slow (inclusive) to left
                # split sublist between slow and fast (inclusive) to right
                left_head = rest.next
                right_head = slow.next

                # advacne rest pointer
                rest.next = fast.next if fast else None

                # ending above two sublists
                slow.next = None
                if fast:
                    fast.next = None

                # print("left", to_list(left_head))
                # print("righ", to_list(right_head))

                cmp_count += 1

                # Compare left and right and put it in new_tail
                while left_head and right_head:
                    if left_head.val <= right_head.val:
                        # Dont reorder
                        new_tail.next = left_head
                        left_head = left_head.next
                        new_tail = new_tail.next
                        new_tail.next = None
                    else:
                        # Dont reorder
                        new_tail.next = right_head
                        right_head = right_head.next
                        new_tail = new_tail.next
                        new_tail.next = None

                if left_head is None:
                    left_head = right_head

                while left_head is not None:
                    # Dont reorder
                    new_tail.next = left_head
                    left_head = left_head.next
                    new_tail = new_tail.next
                    new_tail.next = None

            if cmp_count == 0:
                return new_head.next

            head = new_head
            part_size *= 2

        raise AssertionError("unreachable")

    def sortList2(self, head: Optional[ListNode]) -> Optional[ListNode]:
        head = ListNode(next=head)

        while True:
            popped = False
            node = head.next

            while node and node.next:
                if node.val > node.next.val:
                    # pop node.next, and
                    node_next = node.next
                    node.next = node_next.next

                    # insert it after head
                    node_next.next = head.next
                    head.next = node_next
                    popped = True
                else:
                    node = node.next

            if not popped:
                return head.next

        raise AssertionError("unreachable")
