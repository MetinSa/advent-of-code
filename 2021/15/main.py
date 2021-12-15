import heapq
from typing import List, Tuple

import numpy as np

MAX_VAL = 9
CAVERN_SIZE_MULTIPLIER = 5


def get_grid(filename: str) -> np.ndarray:
    grid: List[List[str]] = []
    with open(filename, "r") as file:
        for line in file:
            grid.append(list(line.strip()))

    return np.array(grid, dtype=int)


def get_larger_grid(grid: np.ndarray) -> np.ndarray:
    n_rows_grid, n_cols_grid = grid.shape
    larger_grid = np.zeros(
        shape=(
            n_rows_grid * CAVERN_SIZE_MULTIPLIER,
            n_cols_grid * CAVERN_SIZE_MULTIPLIER,
        ),
        dtype=int,
    )
    n_rows_larger_grid, n_cols_larger_grid = larger_grid.shape
    for row in range(n_rows_larger_grid):
        n_larger_row = row // n_rows_grid
        grid_row = row % n_rows_grid
        for col in range(n_cols_larger_grid):
            grid_col = col % n_cols_grid
            n_larger_col = col // n_cols_grid

            row_col_increase = n_larger_row + n_larger_col
            value = grid[grid_row, grid_col] + row_col_increase
            while value > MAX_VAL:
                value -= MAX_VAL

            larger_grid[row, col] = value

    return larger_grid


def get_neighbors(grid: np.ndarray, current_position: Tuple[int, int]):
    x, y = current_position
    neighbors: List[Tuple[int, int]] = []
    for xx, yy in [(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)]:
        if (0 <= xx < len(grid)) and (0 <= yy < len(grid)):
            neighbors.append((xx, yy))

    return neighbors


def find_smallest_cost(grid: np.ndarray):
    origin = (0, 0)
    destination = (len(grid) - 1, len(grid) - 1)
    costs = {}
    candidates = [(0, origin)]

    while True:
        current_cost, current_idx = heapq.heappop(candidates)
        if current_idx == destination:
            return current_cost

        neighbors = get_neighbors(grid, current_idx)
        for neighbor in neighbors:
            new_cost = current_cost + grid[neighbor]
            if neighbor in costs and costs[neighbor] <= new_cost:
                continue
            costs[neighbor] = new_cost
            heapq.heappush(candidates, (new_cost, neighbor))


if __name__ == "__main__":
    INPUT_FILENAME = "input.txt"
    print("Part 1:")
    grid = get_grid(filename=INPUT_FILENAME)
    cost = find_smallest_cost(grid)
    print(f"The lowest total risk of any path is {cost}.")

    print("\nPart 2:")
    larger_grid = get_larger_grid(grid)
    larger_cost = find_smallest_cost(larger_grid)
    print(f"The lowest total risk of any path is {larger_cost}.")
