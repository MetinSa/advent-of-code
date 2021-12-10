from math import prod
from typing import Tuple

import numpy as np
from scipy.ndimage import label


MAX_HEIGHTS = 9


def get_heightmap(filename: str) -> np.ndarray:
    heightmap = []
    with open(filename, "r") as file:
        for line in file:
            line = line.strip()
            heightmap.append(list(line))

    return np.array(heightmap, dtype=int)


def get_basins(heightmap: np.ndarray) -> Tuple[np.ndarray, int]:
    basin_mask = heightmap != MAX_HEIGHTS
    basins, n_basins = label(basin_mask)

    return basins, n_basins


def get_lowpoints(heightmap: np.ndarray, basins: np.ndarray, n_basins: int):

    return [min(heightmap[basins == i]) for i in range(1, n_basins + 1)]


def get_prod_largest_basin_sizes(basins: np.ndarray, n_basins: int) -> int:
    basin_indicies = [(basins == i).nonzero() for i in range(1, n_basins + 1)]
    sizes = [len(indicies[0]) for indicies in basin_indicies]
    three_largest_sizes = sorted(sizes, reverse=True)[:3]

    return prod(three_largest_sizes)


if __name__ == "__main__":
    INPUT_FILENAME = "input.txt"

    heightmap = get_heightmap(INPUT_FILENAME)
    basins, n_basins = get_basins(heightmap)
    lowpoints = get_lowpoints(heightmap, basins, n_basins)

    print("Part 1:")
    sum_of_risk_levels = 0
    for lowpoint in lowpoints:
        sum_of_risk_levels += 1 + lowpoint
    print(f"Sum of risk levels are {sum_of_risk_levels}")

    print("\nPart 2:")
    basin_prod = get_prod_largest_basin_sizes(basins, n_basins)
    print(f"The product of the sizes of the three largest basins are {basin_prod}")
