from leetcode_prelude import *


# Problem 2751
class Solution:
    def survivedRobotsHealths(
        self,
        positions: List[int],
        healths: List[int],
        directions: str,
    ) -> List[int]:
        indices = list(range(len(positions)))
        indices.sort(key=lambda s: positions[s])

        stack = []
        for idx in indices:
            dir_ = directions[idx]
            # print(idx, dir_, stack, healths)

            if dir_ == "R":
                stack.append(idx)
            else:
                hp = healths[idx]

                while stack and hp > 0:
                    idx2 = stack[-1]
                    hp2 = healths[idx2]

                    if hp > hp2:
                        hp -= 1
                        hp2 = 0
                    elif hp == hp2:
                        hp = 0
                        hp2 = 0
                    else:
                        # hp < hp2
                        hp = 0
                        hp2 -= 1

                    healths[idx2] = hp2
                    if hp2 <= 0:
                        stack.pop()

                healths[idx] = hp
            # print("END", stack, healths)

        # print(healths)

        return [s for s in healths if s > 0]
