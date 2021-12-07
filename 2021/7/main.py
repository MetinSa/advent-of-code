from typing import Literal, Tuple, Union, Protocol

import numpy as np


INPUT_FILENAME = "input.txt"

FuelRateType = Union[Literal["linear"], Literal["triangular"]]


def get_minimum_fuel_required(
    positions: np.ndarray, fuel_rate: FuelRateType
) -> Tuple[int, int]:
    std = positions.std()
    mean = positions.mean()

    min_new_position = round(mean - std)
    max_new_position = round(mean + std)
    new_positions = range(min_new_position, max_new_position)

    fuel_function = get_fuel_function(fuel_rate)
    fuel = fuel_function(old_positions=positions, new_positions=new_positions)

    min_fuel = fuel.min()
    new_position = new_positions[fuel.argmin()]

    return min_fuel, new_position


class FuelStrategy(Protocol):
    def __call__(self, old_positions: np.ndarray, new_positions: range) -> np.ndarray:
        ...


def fuel_linear(old_positions: np.ndarray, new_positions: range) -> np.ndarray:
    fuel = np.zeros(len(new_positions), dtype=int)
    for idx, new_position in enumerate(new_positions):
        fuel[idx] = abs(old_positions - new_position).sum()

    return fuel


def fuel_triangular(old_positions: np.ndarray, new_positions: range) -> np.ndarray:
    fuel = np.zeros(len(new_positions), dtype=int)
    for idx, new_position in enumerate(new_positions):
        n = abs(old_positions - new_position)
        fuel[idx] = ((n ** 2 + n) / 2).sum().round()

    return fuel


def get_fuel_function(fuel_rate: FuelRateType) -> FuelStrategy:
    if fuel_rate == "linear":
        return fuel_linear
    elif fuel_rate == "triangular":
        return fuel_triangular

    raise NotImplementedError


def read_horizontal_position(filename: str) -> np.ndarray:
    return np.loadtxt(filename, delimiter=",", dtype=int)


if __name__ == "__main__":
    positions = read_horizontal_position(INPUT_FILENAME)

    print("Part 1:")
    min_fuel, new_position = get_minimum_fuel_required(
        positions=positions, fuel_rate="linear"
    )
    print(
        f"Minimum fuel ({min_fuel}) is used when aligning in horizontal "
        f"position {new_position}."
    )

    print("Part 2:")
    min_fuel, new_position = get_minimum_fuel_required(
        positions=positions, fuel_rate="triangular"
    )
    print(
        f"Minimum fuel ({min_fuel}) is used when aligning in horizontal "
        f"position {new_position}."
    )
