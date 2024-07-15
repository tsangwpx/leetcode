from leetcode_prelude import *


# Problem 460
class Node:
    def __init__(
        self,
        key: int,
        value: int,
    ) -> None:
        self.key = key
        self.value = value
        self.freq = 1


class FreqList:
    def __init__(self, freq: int) -> None:
        self.freq = freq
        self.nodes: dict[int, Node] = {}
        self.prev: FreqList | None = None
        self.next: FreqList | None = None


def dump_lists(head: FreqList | None) -> list[int]:
    def list_nodes(nodes: dict[int, Node]) -> str:
        parts = [f"{key}->{node.value}" for key, node in nodes.items()]

        return ",".join(parts)

    res = []
    seen = set()
    node = head
    while node is not None:
        key = id(node)
        if key in seen:
            res.append("dup")
            res.append(node.freq)
            break

        seen.add(key)
        res.append(node.freq)
        res.append(node.prev.freq if node.prev else "None")
        res.append(list_nodes(node.nodes))
        node = node.next

    return res


class LFUCache:
    def __init__(self, capacity: int):

        self.capacity = capacity

        self.nodes: dict[int, Node] = {}
        self.freq_lists: dict[int, FreqList] = {}
        self.freq_head = None

    def _remove_freq_list(self, freq: int, freq_list: FreqList):
        # print("removed list", freq, freq_list is self.freq_head)

        assert not freq_list.nodes
        # remove freq list if empty
        prev, next_ = freq_list.prev, freq_list.next

        # print(
        #     "around",
        #     prev.freq if prev else "None",
        #     next_.freq if next_ else "None",
        #     freq_list is next_,
        # )

        self.freq_lists.pop(freq)

        if prev is not None:
            prev.next = next_

        if next_ is not None:
            next_.prev = prev

        if freq_list is self.freq_head:
            self.freq_head = next_

    def _promote(self, key: int, node: Node):
        old_freq = node.freq
        node.freq += 1
        new_freq = node.freq

        # print("promote", key, node.value, old_freq, new_freq)

        old_freq_list = self.freq_lists[old_freq]
        new_freq_list = self.freq_lists.get(new_freq)

        if new_freq_list is None:
            # print("create", new_freq)
            next_freq_list = old_freq_list.next

            new_freq_list = FreqList(new_freq)
            new_freq_list.prev = old_freq_list
            new_freq_list.next = next_freq_list
            old_freq_list.next = new_freq_list
            if next_freq_list is not None:
                next_freq_list.prev = new_freq_list

            self.freq_lists[new_freq] = new_freq_list

        old_freq_list.nodes.pop(key)
        new_freq_list.nodes[key] = node

        if not old_freq_list.nodes:
            self._remove_freq_list(old_freq, old_freq_list)

    def get(self, key: int) -> int:
        # print("get", key)
        node = self.nodes.get(key)

        if node is None:
            return -1

        self._promote(key, node)

        # print(dump_lists(self.freq_head))
        return node.value

    def put(self, key: int, value: int) -> None:
        # print("put", key, value)
        node = self.nodes.get(key)

        if node is not None:
            node.value = value
            self._promote(key, node)
            return

        if len(self.nodes) >= self.capacity:
            removed_key = next(iter(self.freq_head.nodes))
            removed_node = self.nodes.pop(removed_key)
            # print("removed", key, removed_node.freq)

            removed_list = self.freq_lists[removed_node.freq]
            removed_list.nodes.pop(removed_key)

            if not removed_list.nodes:
                self._remove_freq_list(removed_node.freq, removed_list)

        node = Node(key, value)
        self.nodes[key] = node

        freq_list = self.freq_lists.get(node.freq)
        if freq_list is None:
            freq_next = self.freq_head

            freq_list = FreqList(1)
            freq_list.next = freq_next

            if freq_next:
                freq_next.prev = freq_list

            self.freq_head = freq_list
            self.freq_lists[node.freq] = freq_list

        freq_list.nodes[key] = node

        # print(dump_lists(self.freq_head))


# Your LFUCache object will be instantiated and called as such:
# obj = LFUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)
