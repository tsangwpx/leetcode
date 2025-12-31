from leetcode_prelude import *


# Problem 1233
class Solution:
    def removeSubfolders(self, folders: List[str]) -> List[str]:
        """
        First, store all path in a set for fast prefix checking

        Second, iterate the set, and for each path, check all their proper prefixes not existing
    
        Time complexity: O(N)
        """
        import re

        def normalize(s: str) -> str:
            return re.sub(r"/+", "/", s.rstrip("/")) or "/"

        folders = set(normalize(s) for s in folders)

        def get_prefixes(path: str) -> list[str]:
            result = []

            pos = 1
            while (pos := path.find("/", pos)) >= 0:
                result.append(path[0:pos])
                pos += 1

            return result

        result = []

        for path in folders:
            if all(p not in folders for p in get_prefixes(path)):
                result.append(path)

        return result
