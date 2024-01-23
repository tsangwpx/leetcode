from leetcode_prelude import *


# Problem 1531
class Solution:
    def getLengthOfOptimalCompression(self, s: str, k: int) -> int:
        dp = [[9999] * (k + 1) for _ in range(len(s) + 1)]
        dp[0][:] = [0] * (k + 1)

        count2len = [0] * 101
        count2len[1] = 1
        count2len[2:10] = [2] * 8
        count2len[10:100] = [3] * 90
        count2len[100] = 4

        for i in range(len(s) + 1):
            for j in range(0, k + 1):
                length = dp[i][j]

                if j > 0:
                    length = min(length, dp[i - 1][j - 1])

                start = i - 1
                stop = i
                deleted = 0
                ch = s[start]

                length = min(length, dp[start][j - deleted] + 1)

                for start in range(stop - 1, -1, -1):
                    if s[start] != ch:
                        if deleted >= j:
                            break
                        deleted += 1

                    count = stop - start - deleted
                    encoded_len = count2len[count]

                    length = min(length, dp[start][j - deleted] + encoded_len)

                dp[i][j] = length

        return dp[len(s)][k]
