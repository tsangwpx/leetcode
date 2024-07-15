from typing import List


class Solution:
    def wiggleMaxLength(self, nums: List[int]) -> int:
        up = 1
        down = 1
        for prev, curr in zip(nums[:-1], nums[1:]):
            if curr > prev:
                up = down + 1
            elif curr < prev:
                down = up + 1
        return max(up, down)


if __name__ == "__main__":
    print(
        Solution().scheduleCourse([[100, 200], [200, 1300], [1000, 1250], [2000, 3200]])
    )
    print(Solution().scheduleCourse([[1, 2]]))
    print(Solution().scheduleCourse([[3, 2], [4, 3]]))
