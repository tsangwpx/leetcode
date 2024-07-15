from leetcode_prelude import *


# Problem 1488
class Solution:
    def avoidFlood(self, rains: List[int]) -> List[int]:
        from bisect import bisect_right

        full = {}

        unused_dries = []

        ans = [-1] * len(rains)

        for today, lake in enumerate(rains):
            if lake == 0:
                unused_dries.append(today)
                continue

            if lake not in full:
                full[lake] = today
                continue

            past = full[lake]
            dry_idx = bisect_right(unused_dries, past)
            if dry_idx >= len(unused_dries):
                # Impossible
                return []

            dry_day = unused_dries.pop(dry_idx)

            ans[dry_day] = lake
            full[lake] = today

        for day in unused_dries:
            ans[day] = 1

        return ans

    def avoidFlood2(self, rains: List[int]) -> List[int]:
        from heapq import heappop, heappush

        full = {}

        front_heap = []  # max heap
        back_heap = []  # min heap

        ans = [-1] * len(rains)

        for today, lake in enumerate(rains):
            if lake == 0:
                heappush(back_heap, today)
                continue

            if lake not in full:
                full[lake] = today
                continue

            past = full[lake]

            while front_heap and -front_heap[0] > past:
                heappush(back_heap, -heappop(front_heap))

            while back_heap and back_heap[0] < past:
                heappush(front_heap, -heappop(back_heap))

            if not back_heap:
                return []

            dry_day = heappop(back_heap)

            print(f"Day {today} dry {lake} at {dry_day}")

            ans[dry_day] = lake
            full[lake] = today

        back_heap.extend([-s for s in front_heap])
        for day in back_heap:
            ans[day] = 1

        return ans
