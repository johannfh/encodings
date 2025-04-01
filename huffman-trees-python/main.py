import heapq
from collections import defaultdict


class HuffmanNode:

    value: str | None
    frequency: int

    left: "HuffmanNode | None"

    right: "HuffmanNode | None"

    def __init__(self, value: str | None, frequency: int) -> None:
        self.value = value
        self.frequency = frequency
        self.left = None
        self.right = None

    def __lt__(self, other: "HuffmanNode") -> bool:
        return self.frequency < other.frequency
    
    def __str__(self) -> str:
        return self.format(2)
    
    def format(self, indent: int) -> str:
        return f"({self.value}: {self.frequency})"
    
    def debug(self, indent: int, level=0):
        print(" " * indent * level + str(self))
        if self.left:
            self.left.debug(indent, level + 1)
        if self.right:
            self.right.debug(indent, level + 1)

type FrequencyMap = dict[str, int]


def create_frequency_map(input_str: str) -> FrequencyMap:
    frequency_map: defaultdict[str, int] = defaultdict(int)

    for char in input_str:
        frequency_map[char] += 1

    return frequency_map


def create_huffman_tree(input_str: str) -> HuffmanNode:

    freq_map = create_frequency_map(input_str)

    print(freq_map)

    priority_queue = [HuffmanNode(char, freq) for char, freq in freq_map.items()]
    heapq.heapify(priority_queue)

    while len(priority_queue) > 1:
        left_node = heapq.heappop(priority_queue)
        right_node = heapq.heappop(priority_queue)
        print(left_node, right_node)

        merged_node = HuffmanNode(None, left_node.frequency + right_node.frequency)
        merged_node.left = left_node
        merged_node.right = right_node
        print(f"merged: {merged_node.left} and {merged_node.right} to {merged_node}")

        heapq.heappush(priority_queue, merged_node)

    return priority_queue[0]

x = "mississippi"

huffman_tree = create_huffman_tree(x)

huffman_tree.debug(2)


create_huffman_tree("aaaaabbbbcccdde").debug(2)