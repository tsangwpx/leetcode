from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# Problem 437
class Solution:
    def pathSum(self, root: Optional[TreeNode], targetSum: int) -> int:
        from collections import Counter
        from bisect import bisect_left, bisect_right
        from heapq import merge

        def visit(node):
            # bottom up
            if node is None:
                return 0, []

            left_count, left_sums = visit(node.left)
            right_count, right_sums = visit(node.right)

            val = node.val
            # merged = list(merge([0], left_sums, right_sums))
            # merged = [val + s for s in merged]

            # merged = [val + s for s in merge([0], left_sums, right_sums)]

            start = bisect_left(merged, targetSum)
            stop = bisect_right(merged, targetSum, start)
            count = stop - start + left_count + right_count

            return count, merged

        return visit(root)[0]

    def pathSum4(self, root: Optional[TreeNode], targetSum: int) -> int:
        from collections import Counter

        def visit(node, partial_sum, is_root):
            if node is None:
                return 0

            count = 0
            partial_sum += node.val
            if partial_sum == targetSum:
                count += 1

            count += visit(node.left, partial_sum, False)
            count += visit(node.right, partial_sum, False)

            if is_root:
                count += visit(node.left, 0, True)
                count += visit(node.right, 0, True)

            return count

        return visit(root, 0, True)

    def pathSum3(self, root: Optional[TreeNode], targetSum: int) -> int:
        # This is not working because too slow
        from operator import eq
        from functools import partial

        def visit(node, partial_sum, is_new):
            if node is None:
                return

            val = node.val
            partial_sum += val
            yield partial_sum

            if node.left is not None:
                yield from visit(node.left, partial_sum, False)
                if is_new:
                    yield from visit(node.left, 0, True)

            if node.right is not None:
                yield from visit(node.right, partial_sum, False)
                if is_new:
                    yield from visit(node.right, 0, True)

        return sum(map(partial(eq, targetSum), visit(root, 0, True)))

    def pathSum2(self, root: Optional[TreeNode], targetSum: int) -> int:
        from collections import Counter

        def visit(node):
            if node is None:
                return 0, {}

            val = node.val
            counter = Counter()

            if node.left is not None:
                left_count, left_counter = visit(node.left)
                counter.update({s + val: count for s, count in left_counter.items()})
            else:
                left_count = 0

            if node.right is not None:
                right_count, right_counter = visit(node.right)
                counter.update({s + val: count for s, count in right_counter.items()})
            else:
                right_count = 0

            counter[val] += 1

            # print(f"{val} {sorted(list(counter.items()))} {left_count} {right_count}")

            return counter[targetSum] + right_count + left_count, counter

        return visit(root)[0]
