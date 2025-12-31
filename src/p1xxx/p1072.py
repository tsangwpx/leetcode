from leetcode_prelude import *


# Problem 1072
class Solution:
    def maxEqualRowsAfterFlips(self, matrix: List[List[int]]) -> int:
        """
        1. Column flips affect all rows in a matrix.
            Rows with similar bit-configuration result in all-one or all-zero bits
        3. We may choose flip a row and will not change the results
        2. We may compute a presentation string/int of a row
        4. So if the leading cell has 1, the row is flipped
        """
        from collections import Counter

        counter = Counter()

        for row in matrix:
            if row[0]:
                h = tuple([1 - s for s in row])
            else:
                h = tuple(row)

            counter[h] += 1

        # print(sorted(counter.items()))

        return max(counter.values())
