from typing import Literal, Union
import numpy as np


INPUT_FILENAME = "input.txt"
TEST_INPUT_FILENAME = "test_input.txt"


def get_power_consumption_from_file(filename: str) -> int:
    """Returns the power consumption from an input file.

    Parameters
    ----------
    filename
        Input file with binaries.

    Returns
    -------
        Power consumption (gamma rate x epsilon rate).
    """

    gamma_rate_binary = []
    epsilon_rate_binary = []
    binary = np.genfromtxt(filename, delimiter=1, dtype=np.int8).transpose()

    for col in binary:
        n_zeros, n_ones = np.bincount(col)
        if n_zeros > n_ones:
            gamma_rate_binary.append(0)
            epsilon_rate_binary.append(1)
        else:
            gamma_rate_binary.append(1)
            epsilon_rate_binary.append(0)

    gamma_rate = int("".join(str(binary) for binary in gamma_rate_binary), base=2)
    epsilon_rate = int("".join(str(binary) for binary in epsilon_rate_binary), base=2)

    return gamma_rate * epsilon_rate


def get_ratings(filename: str):
    """Returns the oxygen ratings times the C02 ratings given a input file.

    Parameters
    ----------
    filename
        Input file with rows of bits.

    Returns
    -------
        The oxygen rating timex the C02 rating.
    """

    input_binaries = np.genfromtxt(filename, delimiter=1, dtype=np.int8)

    oxygen_bits = filter_binaries(input_binaries, "oxygen")
    C02_bits = filter_binaries(input_binaries, "c02")
    oxygen_rating = bits_array_to_decimal(oxygen_bits)
    C02_rating = bits_array_to_decimal(C02_bits)

    return oxygen_rating * C02_rating


def filter_binaries(
    binaries: np.ndarray, bit_criteria: Union[Literal["oxygen"], Literal["c02"]]
) -> np.ndarray:
    """Filters an array of binaries by a given criteria.

    Parameters
    ----------
    binaries
        Array of binaries with shape (`N_inputs`, 5).
    bit_criteria
        int representing either the oxygen criteria (1), or the C02 criteria (0).

    Returns
    -------
        Filtered array, with a single row remaining.
    """

    if bit_criteria not in ("oxygen", "c02"):
        raise ValueError('bit_criteria must be either "oxygen" or "c02"')

    n_bits = len(binaries[0])
    for i in range(n_bits):
        accepted_rows = []
        n_zeros, n_ones = np.bincount(binaries[:, i])

        if bit_criteria == "oxygen":
            if n_ones > n_zeros:
                criteria = 1
            elif n_zeros > n_ones:
                criteria = 0
            else:
                criteria = 1
        elif bit_criteria == "c02":
            if n_ones > n_zeros:
                criteria = 0
            elif n_zeros > n_ones:
                criteria = 1
            else:
                criteria = 0

        for row in binaries:
            if row[i] == criteria:
                accepted_rows.append(list(row))

        binaries = np.asarray(accepted_rows)

        # If we hit a single row before we finish looping over all bits in a
        # row, we break out of the loop and return the row.
        if not len(binaries) > 1:
            break

    return binaries.squeeze()


def bits_array_to_decimal(bits: np.ndarray) -> int:
    """Returns a decimal given an array of bits in integers."""

    return int("".join(str(bit) for bit in bits), base=2)


if __name__ == "__main__":
    power_consuption = get_power_consumption_from_file(INPUT_FILENAME)
    ratings = get_ratings(INPUT_FILENAME)

    print("Part1: Power consumption is:", power_consuption)
    print("Part2: The Oxygen rating times the C02 rating is:", ratings)
