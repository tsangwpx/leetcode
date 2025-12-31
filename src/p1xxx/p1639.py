from leetcode_prelude import *


# Problem 1639
class Solution:
    def numWays(self, words: List[str], target: str) -> int:
        """
        DP

        1. Record character frequency in each word positions.
        2. Define dp[target_size][word_size] be the number of ways
            to form target[0:target_size]
            using the first word_size character in words.
        3. Shape of dp is (len(target) + 1, word_len + 1).
            So, the dp index offset by +1 relative to the word and the target index.
            Initially, dp[0][0] = 1, and dp[i][j] = 0 otherwise.
        4. dp[i + 1][j + 1] = sum(dp[i][0:j]) * w
            where the number of words such that word[j] == target[i],
            which is computed in the step (1).
        5. The sum can be fused into the j-loop. It reduces the time complexity
            from O(NK**2) to O(NK) where N = len(target), K = len(words[0])

        """
        from collections import Counter

        MOD = 10**9 + 7

        word_len = len(words[0])
        counters: list[dict[str, int]] = [Counter() for _ in range(word_len)]

        for seq in words:
            for idx, ch in enumerate(seq):
                counters[idx][ch] += 1

        dp = [[0] * (word_len + 1) for _ in range(len(target) + 1)]
        dp[0][0] = 1

        # print(dp[0])

        for i, ch in enumerate(target):
            prev = i
            curr = i + 1

            start = i
            stop = min(word_len, word_len - (len(target) - i) + 1)

            prev_sum = sum(dp[prev][0:start])

            # print(start, stop)

            for j in range(start, stop):
                prev_sum += dp[prev][j]

                if counters[j][ch] == 0:
                    continue

                idx = j + 1

                dp[curr][idx] += prev_sum * counters[j][ch]
                dp[curr][idx] %= MOD

            # print(dp[curr])

        return sum(dp[len(target)]) % MOD
