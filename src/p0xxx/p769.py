# Problem 769
class Solution:
    def maxChunksToSorted(self, arr: List[int]) -> int:
        count = 0
        maximum = 0

        for idx, item in enumerate(arr):
            maximum = max(maximum, item)
            if idx == maximum:
                # fixed point
                count += 1

        return count
