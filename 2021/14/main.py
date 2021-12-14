from typing import Dict, List, Tuple

import numpy as np


def get_puzzle_input(filename: str) -> Tuple[List[str], Dict[str, str]]:
    pair_insertions: Dict[str, str] = {}
    with open(filename, "r") as file:
        lines = file.readlines()
        template = list(lines[0].strip())
        for line in lines[2:]:
            pair, insertion = line.strip().split(" -> ")
            pair_insertions[pair] = insertion

    return template, pair_insertions


def get_initial_element_pair_count(
    template: List[str], pair_insertions: Dict[str, str]
) -> Dict[str, int]:

    pair_counts = {pair: 0 for pair in pair_insertions}
    for i in range(len(template) - 1):
        pair = "".join([template[i], template[i + 1]])
        if pair in pair_counts:
            pair_counts[pair] += 1

    return pair_counts


def get_element_pair_count(
    steps: int,
    pair_insertions: Dict[str, str],
    initial_pair_count: Dict[str, int],
) -> Dict[str, int]:

    pair_counts = initial_pair_count
    for _ in range(steps):
        new_pair_counts = {pair: 0 for pair in pair_insertions}
        for pair in pair_counts:
            insert = pair_insertions[pair]
            new_pair_counts[pair[0] + insert] += pair_counts[pair]
            new_pair_counts[insert + pair[1]] += pair_counts[pair]

        pair_counts = new_pair_counts

    return pair_counts


def get_unique_element_count(pair_counts: Dict[str, int]) -> Dict[str, int]:
    all_elements_in_pairs = "".join([pair for pair in pair_counts])
    unique_elements = np.unique(list(all_elements_in_pairs))

    unique_element_counts: Dict[str, List[int]] = {
        element: [0, 0] for element in unique_elements
    }

    for pair, count in pair_counts.items():
        first_element, second_element = pair
        unique_element_counts[first_element][0] += count
        unique_element_counts[second_element][1] += count

    return {
        element: max(unique_element_counts[element])
        for element in unique_element_counts
    }


if __name__ == "__main__":
    INPUT_FILENAME = "input.txt"
    template, pair_insertions = get_puzzle_input(filename=INPUT_FILENAME)
    initial_pair_count = get_initial_element_pair_count(
        template=template,
        pair_insertions=pair_insertions,
    )

    print("Part 1:")
    STEPS = 10
    pair_counts = get_element_pair_count(
        steps=STEPS,
        pair_insertions=pair_insertions,
        initial_pair_count=initial_pair_count,
    )
    unique_element_count = get_unique_element_count(pair_counts=pair_counts)
    diff = max(counts := unique_element_count.values()) - min(counts)
    print(
        f"After {STEPS} steps, the difference between the most and least common "
        f"elements are {diff}."
    )

    print("\nPart 2:")
    STEPS = 40
    pair_counts = get_element_pair_count(
        steps=STEPS,
        pair_insertions=pair_insertions,
        initial_pair_count=initial_pair_count,
    )
    unique_element_count = get_unique_element_count(pair_counts=pair_counts)
    diff = max(counts := unique_element_count.values()) - min(counts)
    print(
        f"After {STEPS} steps, the difference between the most and least common "
        f"elements are {diff}."
    )
