# Problem 689
class Solution:
    def maxSumOfThreeSubarrays(
        self,
        nums: List[int],
        k: int,
    ) -> List[int]:
        """
        Three pointer solution stolen from JaRail

        This solution create a sliding window consisting 3 partitions with size k each.

        As the loop iterate from 3k to the end, elements enter and leave the partitions,
        the maximum sum of the left partition is found.

        By fixing the middle partition, we can find the maximum sum of the left and the middle
        partitions and save their indices by combining the best index of the left partition
        and the current middle index.

        Similarly, fixing the right partition, the maximum sum of the three partitions can be found
        and save their indices, which is the solution to the problem.

        """

        u = 0
        v = u + k
        w = v + k

        sum_u = sum(nums[u:v])
        sum_v = sum(nums[v:w])
        sum_w = sum(nums[w : w + k])

        best_u = [u]
        best_uv = [u, v]
        best_uvw = [u, v, w]
        best_sum_u = sum_u
        best_sum_uv = best_sum_u + sum_v
        best_sum_uvw = best_sum_uv + sum_w

        for i in range(k + k + k, len(nums)):
            w = i - k
            v = w - k
            u = v - k

            sum_u += nums[v] - nums[u]
            sum_v += nums[w] - nums[v]
            sum_w += nums[i] - nums[w]

            u += 1
            v += 1
            w += 1

            if sum_u > best_sum_u:
                best_u = [u]
                best_sum_u = sum_u

            if sum_v + best_sum_u > best_sum_uv:
                best_uv = best_u + [v]
                best_sum_uv = best_sum_u + sum_v

            if sum_w + best_sum_uv > best_sum_uvw:
                best_uvw = best_uv + [w]
                best_sum_uvw = best_sum_uv + sum_w

        return best_uvw
