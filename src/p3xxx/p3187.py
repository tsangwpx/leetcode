from leetcode_prelude import *


# Problem 3187
class Solution:
    def countOfPeaks(
        self,
        nums: List[int],
        queries: List[List[int]],
    ) -> List[int]:
        size = len(nums)

        tree = [0] * (4 * (size - 2))

        def build(node: int, left: int, right: int) -> None:
            if left == right:
                if nums[left] > nums[left - 1] and nums[left] > nums[left + 1]:
                    tree[node] = 1
                else:
                    tree[node] = 0
            else:
                mid = (left + right) // 2
                build(node * 2, left, mid)
                build(node * 2 + 1, mid + 1, right)
                tree[node] = tree[node * 2] + tree[node * 2 + 1]

            # print("build", node, left, right, tree[node])

        def sum_range(node: int, tl: int, tr: int, ql: int, qr: int) -> int:
            if ql > qr:
                return 0

            if tl == ql and tr == qr:
                return tree[node]

            tm = (tl + tr) // 2

            total = sum_range(node * 2, tl, tm, ql, min(qr, tm))
            total += sum_range(node * 2 + 1, tm + 1, tr, max(ql, tm + 1), qr)

            return total

        def update(node: int, left: int, right: int, idx: int):
            if left == right:
                if nums[left] > nums[left - 1] and nums[left] > nums[left + 1]:
                    tree[node] = 1
                else:
                    tree[node] = 0
            else:
                mid = (left + right) // 2
                if idx <= mid + 1:
                    update(node * 2, left, mid, idx)

                if idx >= mid:
                    update(node * 2 + 1, mid + 1, right, idx)

                tree[node] = tree[node * 2] + tree[node * 2 + 1]

            # print("update", node, left, right, idx, tree[node])

        build(1, 1, size - 2)
        # print("tree", tree)

        res = []
        for row in queries:
            if row[0] == 1:
                _, left, right = row
                qsum = sum_range(1, 1, size - 2, left + 1, right - 1)
                res.append(qsum)

            elif row[0] == 2:
                _, idx, val = row
                nums[idx] = val
                update(1, 1, size - 2, idx)

                # print(row, tree)

        return res
