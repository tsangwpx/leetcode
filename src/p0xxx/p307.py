from typing import List


class NumArray:

    def __init__(self, nums: List[int]):
        self.size = len(nums)
        self.bit = [0] * (self.size + 1)
        self.nums = [0] * self.size

        for i, n in enumerate(nums):
            self.update(i, n)

    def update(self, index: int, val: int) -> None:
        dv = val - self.nums[index]
        self.nums[index] = val

        index += 1
        while index <= self.size:
            self.bit[index] += dv
            index += index & (-index)

        print("update", self.bit)

    def sumRange(self, left: int, right: int) -> int:
        def get_sum(idx: int) -> int:
            idx += 1
            res = 0
            while idx > 0:
                res += self.bit[idx]
                idx -= idx & -idx
            return res

        print("sumRange", left, get_sum(left - 1), right, get_sum(right), self.bit)
        return get_sum(right) - get_sum(left - 1)


# Your NumArray object will be instantiated and called as such:
# obj = NumArray(nums)
# obj.update(index,val)
# param_2 = obj.sumRange(left,right)
