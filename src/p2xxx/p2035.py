import heapq
from bisect import bisect_left, bisect_right
from typing import List, Tuple


def mindist(
    apart: List[int],
    astart: int,
    astop: int,
    asum: int,
    bpart: List[int],
    bstart: int,
    bstop: int,
    bsum: int,
    n: int,
    table: List[Tuple[int, int, int]],
    recursion: int,
) -> int:
    if recursion > 30:
        return asum + bsum

    minimum = step = bsum - asum
    # regardless of odd and even "ret", find the index just before it
    key = (step, -1, -1)
    start = bisect_left(table, key)
    if start == n * n:
        start -= 1

    # print('step', step, 'key', key, start, table[start] if start >= 0 else None)
    # print('asum', asum, 'bsum', bsum, astart, bstop)

    # target = bisect.bisect_left(table, key)
    for i in range(start, -1, -1):
        double_qmp, ai, bi = table[i]

        if double_qmp > step:
            # Violate the assumption that bsum >= asum
            continue
        elif not astart <= ai < astop or not bstart <= bi < bstop:
            continue
        elif double_qmp == step:
            return 0  # done

        qmp = double_qmp // 2
        asum1 = asum + qmp
        bsum1 = bsum - qmp
        minimum = min(minimum, bsum1 - asum1)

        if minimum == 0:
            return 0

        # print('rec', recursion, 'loop', start - i, double_qmp)
        # print('ai', ai, apart[ai], asum1, 'bi', bi, apart[bi], bsum1)

        if ai + 1 < n and bi > 0:
            minimum = min(
                minimum,
                mindist(
                    apart,
                    ai + 1,
                    n,
                    asum1,
                    bpart,
                    0,
                    bi,
                    bsum1,
                    n,
                    table,
                    recursion + 1,
                ),
            )
            if minimum == 0:
                return 0

    return minimum


class Solution:
    def minimumDifference(self, nums: List[int]) -> int:
        return two_buckets(nums)


def make_table(partition: List[int]) -> List[Tuple[int, int]]:
    """
    Form all combinations and return a list of (number count, sum of numbers)
    without repeated entries
    """
    slen = len(partition)
    pending = [(0, 0, 0)]
    table = set()

    while pending:
        start, count, partial_sum = pending.pop()
        table.add((count, partial_sum))
        count += 1

        for i in range(start, slen):
            pending.append((i + 1, count, partial_sum + partition[i]))

    return sorted(table)


def two_buckets(nums: List[int]) -> int:
    """
    fsum = asum + bsum
    diff = abs(bsum - asum) = diff(fsum - 2asum)

    Split nums into two buckets.
    If we pick K numbers from the left one, then we must pick N - K from the right one.
    Each bucket stores (K, sum of K numbers), its size is 2 ** N
    These K numbers and N - K numbers form one of the two arrays.
    The sum of another array can be easily calculated by above formulae.

    The new minimum should smaller than current one
    current > abs(fsum - 2 sum0 - 2 sum1)

    -current < fsum - 2sum0 - 2sum1 < current
    -current - fsum + 2sum0 < -2sum1 < current - fsum + 2sum0
    -(fsum + current) / 2 + sum0 < -sum1 < -(fsum - current) / 2 + sum0
    (fsum - current)/2 - sum0 < sum1 < (fsum + current) / 2 + sum0
    which limits the picking range from bucket B

    Strategy:
    1. iterate the left buckets
    2. pick the possible peer from the right
    3. Add their partial sums to obtain asum, and compute the absolute difference of sums
    4. Find the minimum

    """
    n = len(nums) // 2
    apart = nums[:n]
    bpart = nums[n:]
    asum = sum(apart)
    bsum = sum(bpart)
    fsum = asum + bsum

    atable = make_table(apart)
    btable = make_table(bpart)

    minimum = abs(bsum - asum)

    for count, sum0 in atable:
        peer = n - count
        peer_sum_min = (fsum - minimum) // 2 - sum0
        peer_start = bisect_left(btable, (peer, peer_sum_min))

        peer_sum_max = (fsum + minimum) // 2 - sum0 + 1
        peer_stop = bisect_right(btable, (peer, peer_sum_max), peer_start)

        # print('count', count, 'minimum', minimum, 'peer', peer_start, peer_stop, peer_stop - peer_start)
        peer_idx = peer_start

        while peer_idx < peer_stop:
            rem, sum1 = btable[peer_idx]
            peer_idx += 1

            if rem == peer:
                update_min = abs(fsum - (sum0 + sum1) * 2)
                if update_min < minimum:
                    minimum = update_min
                    # print('before', peer_idx, peer_stop, peer_stop - peer_idx)
                    # peer_idx = max(peer_idx, (fsum - minimum) // 2 - sum0)
                    # peer_stop = min(peer_stop, (fsum + minimum) // 2 - sum0 + 1)
                    # print('after', peer_idx, peer_stop, peer_stop - peer_idx)
                    continue
            elif rem > peer:
                break

        if minimum == 0:
            return 0

    return minimum


def minimum_distance(nums: List[int]):
    """
    First, split sorted nums into two parts, `apart` and `bpart`.
    In the algorithm, the invariant is kept: sum(bpart) >= sum(apart)

    A: a1 a2 a3 ... an
    B: b1 b2 b3 ... bn

    Each time item `p` from A is exchanged with item `q` from B.
    We have the following relationship, which limit the possible exchange pairs.

    step0 = bsum0 - asum0
    Let qmp = q - p
    asum1 = asum0 - p + q = asum0 + qmp
    bsum1 = bsum0 - q + p = bsum0 - qmp
    step1 = bsum1 - asum1 = step0 - 2qmp
    step1 = step0 - 2qmp >= 0
    which this the new minimum distance after exchange p and q

    If step1 = 0, the minimum absolute difference of sums is 0.

    A table of 2qmp is formed to look up the possible (p, q) index pair quickly
    by binary search allowed qmp, which must smaller than step0


    There are two indices
    Partition A: astart is the start index (inclusive) to pick `q`.
    Partition B: bstop is the stop index (inclusive) to pick `q`.
    """

    nums.sort()
    n = len(nums) // 2
    apart = nums[:n]
    bpart = nums[n:]
    table = []

    for i in range(n):
        # if i and apart[i] == apart[i - 1]:
        #     continue

        for j in range(n - 1, -1, -1):
            # if j + 1 < n and bpart[j] == bpart[j + 1]:
            #     continue

            if apart[i] == bpart[j]:
                continue
            table.append(((bpart[j] - apart[i]) * 2, i, j))

    table.sort()

    n = len(apart)
    nsq = n * n
    asum = sum(apart)
    bsum = sum(bpart)
    fsum = asum + bsum

    minimum = bsum - asum
    pq = [(bsum - asum, asum, 0, n)]

    while pq:
        step0, asum0, astart, bstop = heapq.heappop(pq)
        key = (step0, -1, -1)
        i = bisect_left(table, key)
        if i == nsq:
            i -= 1

        for i in range(i, -1, -1):
            two_qmp, ai, bi = table[i]
            if two_qmp > step0 or astart > ai or bstop <= bi:
                continue
            elif two_qmp == step0:
                return 0

            asum1 = asum0 + two_qmp // 2
            step1 = step0 - two_qmp  # step = bsum - asum = abs difference
            minimum = min(minimum, step1)

            if ai + 1 < n and bi > 0:
                heapq.heappush(pq, (step1, asum1, ai + 1, bi))

    # print('count', count)
    return minimum


def run(nums):
    soln = Solution()
    return soln.minimumDifference(nums)


def main():
    inputs = [
        (
            [3, 9, 7, 3],
            # ans: 2
        ),
        (
            [2, -1, 0, 4, -2, -9],
            # ans: 0
        ),
        (
            [76, 8, 45, 20, 74, 84, 28, 1],
            # ans: 2
        ),
        (
            [-68, 55, -23, 13, -20, -14],
            # ans: 3
        ),
        # (
        #     [7772197, 4460211, -7641449, -8856364, 546755, -3673029, 527497, -9392076, 3130315, -5309187, -4781283, 5919119, 3093450, 1132720, 6380128, -3954678, -1651499, -7944388, -3056827, 1610628,
        #      7711173, 6595873, 302974, 7656726, -2572679, 0, 2121026, -5743797, -8897395, -9699694],
        #   # ans: 1
        # ),
        (
            [
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
                10000000,
                -10000000,
                -10000000,
                -10000000,
                -10000000,
            ],
            # ans: 20000000
        ),
    ]

    for (nums,) in inputs:
        print("result", run(nums))


if __name__ == "__main__":
    main()
