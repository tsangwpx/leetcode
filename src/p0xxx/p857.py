from leetcode_prelude import *


# Problem 857
class Solution:
    def mincostToHireWorkers(
        self,
        quality: List[int],
        wage: List[int],
        k: int,
    ) -> float:
        from heapq import heappop, heappush

        # Firstly, there at least one worker earning its expected wage.
        # Otherwise, the result is not the minimum.
        #
        # Let ratioX = wageX / qualityX for this particular worker, called X.
        # This ratio determines the cost scale.
        #
        # Secondly, cost to workers is given by
        # cost = ratio * quality
        #
        # We iterate all workers sorted by non-decreasing ratio.
        # The total cost is calculated by
        # total_cost = ratio_i * total_quality
        # Note that ratio_i is non-decreasing.
        # The worker in each iteration is exactly the worker X.
        # Otherwise, this worker will earn less than the expectation.
        #
        # As we add more worker to the heap, the heap size grow k workers.
        # We remove the worker with the largest quality to minimize the cost.

        workers = [(w / q, q, w) for q, w in zip(quality, wage)]
        workers.sort()

        heap = []
        qsum = 0
        res = float("inf")

        for r, q, w in workers:
            heappush(heap, -q)
            qsum += q

            if len(heap) > k:
                # We may push and pop the last worker.
                # It is fine because qsum is unchanged and ratio is larger
                # so the cost is unchanged after minimization.
                q2 = -heappop(heap)
                qsum -= q2

            if len(heap) == k:
                res = min(res, qsum * r)

        return res
