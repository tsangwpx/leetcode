from leetcode_prelude import *


# Problem 1604
class Solution:
    def alertNames(self, keyName: List[str], keyTime: List[str]) -> List[str]:
        from collections import defaultdict, deque

        keyTime = [int(s[0:2]) * 60 + int(s[3:5]) for s in keyTime]
        pairs = sorted(zip(keyTime, keyName))

        suspicious = set()
        access_times = defaultdict(lambda: deque(maxlen=3))

        for time, name in pairs:
            records = access_times[name]

            records.append(time)
            if len(records) >= 3 and records[0] + 60 >= time:
                suspicious.add(name)

        return sorted(suspicious)
