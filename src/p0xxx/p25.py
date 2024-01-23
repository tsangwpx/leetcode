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


# Problem 25
class Solution:
    """
    This implementation is too slow! but work.
    """

    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        if k == 1:
            return head

        head = ListNode(next=head)
        # P 1 2 3 4 5 6 7
        # P 3 2 1 6 5 4 7

        def remove_node(pivot) -> Optional[ListNode]:
            res = pivot.next
            if res is not None:
                pivot.next = res.next

            res.next = None
            return res

        def insert_node(pivot, node):
            node.next = pivot.next
            pivot.next = node

        def skip_node(node, count) -> Optional[ListNode]:
            for _ in range(count):
                if node is None:
                    return None
                node = node.next

            return node

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

        def desc_node(node):
            return f"Node(val={node.val}, next={node.next.val if node.next is not None else False})"

        pivot = head

        for _ in range(5001):
            enough = skip_node(pivot, k)
            if enough is None:
                # Not enough node between
                break

            right_skip = k - 1
            left_skip = 0
            # 0 ... k - 1
            # 1 ... k - 2
            # 2 ... k - 3
            # k ... k - (k + 1) = -1??

            # k = 3
            # (0, 2), (1, 1)!!!
            # k = 4: (0, 3), (1, 2)
            # k = 5: (0, 4), (1, 3), (2, 2)

            while left_skip < right_skip:
                # P 0 1
                # P 1, left = 0
                # P, right = 1
                # P, 0
                # P, 1

                left_pivot = skip_node(pivot, left_skip)
                left = remove_node(left_pivot)

                # since left is removed, skip one less
                right_pivot = skip_node(left_pivot, right_skip - left_skip - 1)
                right = remove_node(right_pivot)

                # now left_pivot and right_pivot may be the same
                # always insert left before right
                # print(
                #     desc_node(left),
                #     desc_node(right),
                #     desc_node(left_pivot),
                #     desc_node(right_pivot),
                # )
                insert_node(right_pivot, left)
                insert_node(left_pivot, right)

                if left_skip == 0:
                    # this is the start of next reverse if any
                    next_pivot = left

                left_skip += 1
                right_skip -= 1

            pivot = next_pivot

        return head.next
