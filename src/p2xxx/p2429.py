from leetcode_prelude import *


# Problem 2429
class Solution:
    def minimizeXor(self, num1: int, num2: int) -> int:
        # how many bits we need to set
        bit_count = num2.bit_count()

        # where is the largest bit + 1
        bit_length = num1.bit_length()
        x = 0

        # print(bin(num1)[2:])

        while bit_count and num1 and bit_length > bit_count:
            # turn off the largest bit
            mask = 1 << (bit_length - 1)

            x ^= mask
            num1 ^= mask

            # print(bin(num1)[2:])
            # print(bin(x)[2:])

            bit_count -= 1
            bit_length = num1.bit_length()

            # bit_length -= 1
            # mask >>= 1

            # while mask and (num1 & mask) == 0:
            #     mask >>= 1
            #     bit_length -= 1

        # now, bit_count = 0 or we have too many bits
        # we cant set any bit higher, so set the lowest bits

        if bit_count >= bit_length:
            x |= (1 << bit_count) - 1

        # print(bin(x)[2:])

        return x
