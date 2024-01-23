import heapq
from typing import List


class Solution:
    def isPossible(self, target: List[int]) -> bool:
        size = len(target)
        if size == 1:
            return target[0] == 1
        assert size >= 2, "bad input"

        total = sum(target)
        pq = [-s for s in target]
        heapq.heapify(pq)

        largest = -heapq.heappop(pq)

        # the total must be at least the sum of all 1s
        while total > size:
            # Given a sequence at t=1: [a1 a2 ... an], non-decreasing
            # What should it be at t=0?
            # an = R + x and total = R + an, where R = a1 + a2 + ... + a{n-1} and x is the replaced item at t=0
            # thus, x = total - 2R
            # this keeps pushing and popping the queue
            # above give some insight but slow performance in some cases
            # If the item at the same position is replaced twice or more consecutively,
            # t=t1: a1 a2 ... ak' ... an
            # t=1: a1 a2 ... ak .. an
            # ak is replaced t1 times and become ak'
            # total(t=1) = R + ak
            # total(t=2) = 2(R + ak) - (ak) = 2R + ak
            # total(t=3) = 2(2R + ak) - (R + ak) = 3R + ak
            # total(t=4) = 2(3R + ak) - (2R + ak) = 4R + ak
            # ...
            # total(t=t1) = pR + ak, for some positive p
            # now, the largest item at t=t1 must not be part of R, and largest > R
            # total(t=t1) = R + largest = pR + ak
            # largest = (p - 1)R + ak.....(eq1), and p - 1 >= 1 because of the above argument
            # so ak can be obtained by %
            # What if another position is replaced between t=1 to t=t1?
            # From the reverse view, (from t=t1 to t=1),
            # R is changed, so ak is still the position just before the change (leave as exercise :D)

            rest = total - largest
            replaced = largest % rest
            if replaced == 0:
                # this implies replaced == rest. The only allowed case is [1, 1]
                # If size is larger than 2 or any item is larger than 1, then only one of the following is true.
                # case 1: replace an item in the same position twice or more. (R >= 2, and new item > R)
                # case 2: replace an item in the new position (R >= 2, and new item < R)
                # thus, rest must be 1 or the target is invalid
                if rest == 1:
                    return True
                return False

            # the new item (largest) must be strictly greater than replaced item
            if replaced >= largest:
                return False

            total = total - largest + replaced

            # pop the largest from pq
            largest = -heapq.heappushpop(pq, -replaced)

        return largest == 1 and total == size and all(s == -1 for s in pq)


def main():
    print(Solution().isPossible([1, 1000000000]))
    # return
    print(Solution().isPossible([2, 4, 6, 7, 8]))
    print(Solution().isPossible([1, 2, 3, 4, 5]))
    print(Solution().isPossible([9, 3, 5]))
    print(Solution().isPossible([1, 1, 1, 2]))
    print(Solution().isPossible([8, 5]))


if __name__ == '__main__':
    main()
