from leetcode_prelude import *


# Problem 1792
class Solution:
    def maxAverageRatio(
        self,
        classes: List[List[int]],
        extraStudents: int,
    ) -> float:
        """

        ratio[i] = pass[i] / class[i]
        avg_ratio = SUM_i {ratio[i]} / N = SUM_i { pass[i] / class[i] } / N

        let f(x) = (c1 + x) / (c2 + x)
            f'(x) = ( (c2 + x) - (c1 + x)) / (c2 + x)^2
                  = (c2 - c1) / (c2 + x) ^ 2
            and c2 >= c1
            so f'(x) >= 0 always
            f''(x) = -2 (c2 - c1) / (c2 + x) ^ 3 < 0
            so f'(x) is decreasing strictly except c2 = c1.

        numerical stability:
            repeating modifying the average ratio may reduce its precision.
            since the f(x) is increasing, we may bump pass_count and class_size
            and compute the average value in the final step.
        """
        from heapq import heappop, heappush, heapify

        def profit(pass_count: int, class_size: int) -> float:
            return (pass_count + 1) / (class_size + 1) - pass_count / class_size

        n = len(classes)

        avg_ratio = 0.0
        pq = []

        for pass_count, class_size in classes:
            if pass_count == class_size:
                avg_ratio += pass_count / class_size / n
            else:
                pq.append((-profit(pass_count, class_size), pass_count, class_size))

        if pq:
            heapify(pq)

            for _ in range(extraStudents):
                _, pass_count, class_size = heappop(pq)

                pass_count += 1
                class_size += 1
                heappush(pq, (-profit(pass_count, class_size), pass_count, class_size))

            for _, pass_count, class_size in pq:
                avg_ratio += pass_count / class_size / n

        return avg_ratio
