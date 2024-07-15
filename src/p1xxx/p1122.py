from leetcode_prelude import *


# Problem 1122
class Solution:
    def relativeSortArray(self, arr1: List[int], arr2: List[int]) -> List[int]:
        table = {item: i for i, item in enumerate(arr2)}
        size = len(arr2)
        arr1.sort(key=lambda s: (table.get(s, size + s), s))
        return arr1
