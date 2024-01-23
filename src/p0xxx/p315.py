from heapq import heappop, heappush
from typing import List


class Solution:
    def countSmaller(self, nums: List[int]) -> List[int]:
        counts = [0] * len(nums)
        indices = list(range(len(nums)))
        indices2 = indices.copy()

        def mergesort(start, stop, indices, indices2):
            count = stop - start
            if count <= 1:
                return

                # print('mergesort', start, stop, nums[start:stop], counts[start:stop])
            mid = (start + stop) // 2

            mergesort(start, mid, indices2, indices)
            mergesort(mid, stop, indices2, indices)

            # print('merge idx', left, right)
            # print('merge num', [nums[s] for s in left], [nums[s] for s in right])

            smaller_count = 0
            i, i_stop = start, mid
            j, j_stop = mid, stop
            k, k_stop = start, stop

            while i < i_stop and j < j_stop:
                a = nums[indices2[i]]
                b = nums[indices2[j]]

                if a <= b:
                    indices[k] = indices2[i]
                    counts[indices2[i]] += smaller_count
                    k += 1
                    i += 1
                else:
                    indices[k] = indices2[j]
                    smaller_count += 1
                    k += 1
                    j += 1

            while i < i_stop:
                indices[k] = indices2[i]
                counts[indices2[i]] += smaller_count
                k += 1
                i += 1

            while j < j_stop:
                indices[k] = indices2[j]
                k += 1
                j += 1

        mergesort(0, len(nums), indices, indices2)

        return counts

    def countSmaller5(self, nums: List[int]) -> List[int]:
        # Binary Index Tree
        size = len(nums)
        tree = [0] * (size + 1)

        sorted_compact_nums = sorted(set(nums))
        table = {s: i for i, s in enumerate(sorted_compact_nums)}

        def update(idx: int, val: int):
            idx += 1
            while idx <= size:
                tree[idx] += val
                idx += idx & (-idx)

        def sum_to(idx: int):
            partial = 0
            idx += 1
            while idx > 0:
                partial += tree[idx]
                idx -= idx & (-idx)

            return partial

        print('compact', sorted_compact_nums)
        print('tree', tree)

        res = []
        for i in range(len(nums))[::-1]:
            idx = table[nums[i]]
            res.append(sum_to(idx - 1))
            update(idx, 1)

            print('compact', sorted_compact_nums)
            print('tree', tree)

        res.reverse()
        return res

    def countSmaller4(self, nums: List[int]) -> List[int]:
        counts = [0] * len(nums)

        def mergesort(start, stop):
            count = stop - start
            if count <= 1:
                return [start] if count > 0 else []

            # print('mergesort', start, stop, nums[start:stop], counts[start:stop])
            mid = (start + stop) // 2

            left = mergesort(start, mid)
            right = mergesort(mid, stop)

            # print('merge idx', left, right)
            # print('merge num', [nums[s] for s in left], [nums[s] for s in right])

            smaller_count = 0
            indices = []
            i, i_stop = 0, len(left)
            j, j_stop = 0, len(right)

            while i < i_stop and j < j_stop:
                a = nums[left[i]]
                b = nums[right[j]]

                if a <= b:
                    indices.append(left[i])
                    counts[left[i]] += smaller_count
                    i += 1
                else:
                    indices.append(right[j])
                    smaller_count += 1
                    j += 1

            while i < i_stop:
                indices.append(left[i])
                counts[left[i]] += smaller_count
                i += 1

            while j < j_stop:
                indices.append(right[j])
                j += 1

            return indices

        mergesort(0, len(nums))

        return counts

    def countSmaller3(self, nums: List[int]) -> List[int]:
        counts = [0] * len(nums)

        indices = list(range(len(nums)))
        indices.sort(key=lambda s: nums[s])
        incomplete = [True] * len(nums)
        print(indices)

        for idx in indices:
            for i in range(idx):
                if incomplete[i]:
                    counts[i] += 1

            incomplete[idx] = False

        return counts

    def countSmaller2(self, nums: List[int]) -> List[int]:
        counts = [0] * len(nums)
        stack = []  # increasing from head to tail
        maxheap = []

        for i in range(len(nums))[::-1]:
            number = nums[i]

            while stack and stack[-1] < number:
                heappush(maxheap, -stack.pop())

            while maxheap and -maxheap[0] >= number:
                stack.append(-heappop(maxheap))

            # print(i, number, stack, maxheap)
            counts[i] = len(maxheap)
            heappush(maxheap, -number)

        return counts
