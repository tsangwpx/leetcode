from typing import List


class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        left1 = 0
        size1 = len(nums1)

        mid1 = left1 + size1 // 2

        left2 = 0
        size2 = len(nums2)
        mid2 = left2 + size2 // 2

        val1 = nums1[mid1]
        val2 = nums2[mid2]

        print('num1', left1, size1, len(nums1), 'val', mid1, val1)
