from leetcode_prelude import *


# Problem 3130
class Solution:
    def numberOfStableArrays(self, zero: int, one: int, limit: int) -> int:
        MOD = 10**9 + 7

        length = zero + one

        # dp[zero_count][one_count] = number of sub-arrays ending in zero and ones
        dp = [[[0, 0] for _ in range(one + 1)] for _ in range(zero + 1)]

        for k in range(1, min(zero, limit) + 1):
            dp[k][0][0] = 1

        for k in range(1, min(one, limit) + 1):
            dp[0][k][1] = 1

        for num0 in range(1, zero + 1):
            for num1 in range(1, one + 1):
                count0 = sum(dp[num0 - 1][num1])
                count1 = sum(dp[num0][num1 - 1])

                if num0 > limit:
                    count0 -= dp[num0 - limit - 1][num1][1]

                if num1 > limit:
                    count1 -= dp[num0][num1 - limit - 1][0]

                dp[num0][num1][0] = count0 % MOD
                dp[num0][num1][1] = count1 % MOD

        return sum(dp[zero][one]) % MOD

    def numberOfStableArrays2(self, zero: int, one: int, limit: int) -> int:
        MOD = 10**9 + 7

        length = zero + one

        # dp[size][zero_count] = number of sub-arrays ending in zero and ones
        dp = [[[0, 0] for _ in range(zero + 1)] for _ in range(length + 1)]
        dp[1][0][1] = 1
        dp[1][1][0] = 1

        for size in range(2, min(limit, length) + 1):
            # the last element is zero
            for zero_count in range(1, zero + 1):
                if size < zero_count:
                    continue

                count = dp[size - 1][zero_count - 1][0]
                count += dp[size - 1][zero_count - 1][1]

                dp[size][zero_count][0] = count % MOD
                # print(size, zero_count, "0", count)

            # the last element is one
            for zero_count in range(0, zero + 1):
                one_count = size - zero_count
                if one_count <= 0:
                    continue

                count = dp[size - 1][zero_count][0]
                count += dp[size - 1][zero_count][1]

                dp[size][zero_count][1] = count % MOD
                # print(size, zero_count, "1", count)

            # print(size, dp[size])

        # print("sep")

        for size in range(limit + 1, length + 1):
            # the last element is zero
            for zero_count in range(1, zero + 1):
                if size < zero_count:
                    continue

                count = 0

                for offset in range(1, limit + 1):
                    if size - offset < 0 or zero_count - offset < 0:
                        continue

                    count += dp[size - offset][zero_count - offset][1]

                dp[size][zero_count][0] = count % MOD
                # print(size, zero_count, "0", count)

            # the last element is one
            for zero_count in range(0, zero + 1):
                one_count = size - zero_count
                if one_count <= 0:
                    continue

                count = 0

                for offset in range(1, limit + 1):
                    if size - offset < 0:
                        continue

                    count += dp[size - offset][zero_count][0]

                dp[size][zero_count][1] = count % MOD
                # print(size, zero_count, "1", count)

            # print(size, dp[size])

        return sum(dp[length][zero]) % MOD
