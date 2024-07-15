from leetcode_prelude import *


# Problem 1648
class Solution:
    def maxProfit(self, inventory: List[int], orders: int) -> int:
        from heapq import heapify, heappop, heappush

        MOD = 10**9 + 7

        # ball count (each color) and color count in max heap
        pq = [(-s, 1) for s in inventory]
        heapify(pq)

        res = 0

        while pq and orders >= 1:
            neg_count, color_count = heappop(pq)

            # merge color count with the same ball count
            while pq and pq[0][0] == neg_count:
                _, color_count2 = heappop(pq)
                color_count += color_count2

            if neg_count >= 0 or color_count <= 0:
                # discard zero entry, or break the loop
                continue

            ball_count = -neg_count
            lower_bound = -pq[0][0] if pq else 0  # exclusive

            # "simple" math to update the ball count and color count
            consumed = min(orders, (ball_count - lower_bound) * color_count)
            q, r = divmod(consumed, color_count)

            # print(f"{ball_count} balls -> {lower_bound}; {color_count} colors")
            # print(f"{res=}; {orders=}, {consumed=}; {q=}, {r=}")

            orders -= consumed

            if q:
                # SUM OF AS: (START + LAST) * (NUM OF TERMS) // 2
                seqsum = (ball_count * 2 - q + 1) * q // 2
                res = (res + seqsum * color_count) % MOD
                ball_count -= q

            if r:
                # push the ball not used in remainder
                heappush(pq, (-ball_count, color_count - r))

                res = (res + ball_count * r) % MOD
                ball_count -= 1

                # push the ball used in remainder
                heappush(pq, (-ball_count, r))
            else:
                heappush(pq, (-ball_count, color_count))

        return res
