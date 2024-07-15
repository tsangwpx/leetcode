from leetcode_prelude import *


# Problem 632
class Solution:
    def smallestRange(self, nums: List[List[int]]) -> List[int]:
        """
        1. Take the smallest item from each list and put them into a min heap.
        2. Remove the one item from the heap and add the one larger back to heap.

        The heap is maintained to contains exactly one item from each list.
        The range of the heap is the possible smallest width.
        We pick the best heap range as the result.
        """
        from heapq import heappop, heappush

        res_left = 2**31
        res_right = -(2**31)

        heap = []
        # heap_left is heap[0]
        heap_right = -(2**31)

        for i, row in enumerate(nums):
            heappush(heap, (row[0], i, 0))

            heap_right = max(heap_right, row[0])

            res_left = min(res_left, row[0])
            res_right = max(res_right, row[-1])

        while True:
            heap_left, arr_idx, item_idx = heappop(heap)

            delta = (heap_right - heap_left) - (res_right - res_left)

            # print(res_left, res_right, heap_left, heap_right, delta)
            if delta < 0 or (delta == 0 and heap_left < res_left):
                res_left = heap_left
                res_right = heap_right

            arr = nums[arr_idx]
            item_idx += 1
            if item_idx >= len(arr):
                break

            item = arr[item_idx]
            heap_right = max(heap_right, item)
            heappush(heap, (item, arr_idx, item_idx))

        return [res_left, res_right]
