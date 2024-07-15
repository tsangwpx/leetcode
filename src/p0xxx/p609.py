from leetcode_prelude import *


# Problem 609
class Solution:
    def findDuplicate(self, paths: List[str]) -> List[List[str]]:
        from collections import defaultdict

        table: dict[str, list[str]] = defaultdict(list)

        for line in paths:
            parts = line.split(" ")
            prefix = parts[0]

            for file_txt in parts[1:]:
                name, _, txt = file_txt[:-1].partition("(")

                table[txt.rstrip(")")].append((prefix, name))

        return [[f"{a}/{b}" for a, b in v] for v in table.values() if len(v) >= 2]
