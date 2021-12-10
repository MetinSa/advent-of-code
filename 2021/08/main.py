from enum import Enum
from itertools import permutations
from typing import List, Dict, Tuple

import numpy as np


class Digits(Enum):
    ZERO = [True, True, True, False, True, True, True]
    ONE = [False, False, True, False, False, True, False]
    TWO = [True, False, True, True, True, False, True]
    THREE = [True, False, True, True, False, True, True]
    FOUR = [False, True, True, True, False, True, False]
    FIVE = [True, True, False, True, False, True, True]
    SIX = [True, True, False, True, True, True, True]
    SEVEN = [True, False, True, False, False, True, False]
    EIGHT = [True, True, True, True, True, True, True]
    NINE = [True, True, True, True, False, True, True]


DIGITS_TO_INT = {digit: idx for idx, digit in enumerate(Digits)}
DIGIT_SEGMENT_LENGHTS: Dict[Digits, int] = {digit: sum(digit.value) for digit in Digits}
UNIQUE_DIGIT_SEGMENT_LENGHTS: Dict[Digits, int] = {
    digit: sum(digit.value)
    for digit in Digits
    if list(DIGIT_SEGMENT_LENGHTS.values()).count(sum(digit.value)) == 1
}


def get_unique_encoded_digits(encoded_digits: np.ndarray) -> List[Tuple[str, Digits]]:
    unique_digits: List[Tuple[str, Digits]] = []
    for digits in encoded_digits:
        segment_length = len(digits)
        for key, value in UNIQUE_DIGIT_SEGMENT_LENGHTS.items():
            if segment_length == value:
                unique_digits.append((digits, key))
                break
    return unique_digits


def get_all_possible_configs(unique_digits: List[Tuple[str, Digits]]):
    configs = np.array(list(permutations("abcdefg", 7)))

    for unique_digit in unique_digits:
        code, digit = unique_digit
        letters = list(code)
        mask = np.array(digit.value)
        permuts = list(permutations(letters, len(letters)))
        valid_perms = []
        for perm in permuts:
            new_mask = np.all(
                (configs[:, mask] == perm) == [True for _ in range(len(perm))],
                axis=1,
            )
            valid_perms.extend(configs[new_mask])

        configs = np.array(valid_perms)

    return configs


def get_correct_config(configs: np.ndarray, encoded_digits: List[str]) -> np.ndarray:
    for config in configs:
        try:
            for encoded_digit in encoded_digits:
                code = list(encoded_digit)
                mask = np.array([False for _ in range(len(config))])
                for digit in code:
                    mask[np.argwhere(config == digit)[0]] = True
                Digits(mask.tolist())

        except ValueError:
            # All digits do not conform with current config. We try next.
            continue

        return config

    raise ValueError("no config matches the encoded digits.")


def decode_encoded_digits(config: np.ndarray, encoded_digits: List[str]):
    digits: List[Digits] = []
    for encoded_digit in encoded_digits:
        code = list(encoded_digit)
        mask = np.array([False for _ in range(len(config))])
        for digit in code:
            mask[np.argwhere(config == digit)[0]] = True

        digits.append(Digits(mask.tolist()))

    return digits


def get_digits_from_signal(signal: np.ndarray) -> int:
    first_ten_encoded_digits = signal[:10]
    final_four_encoded_digits = signal[11:]
    unique_encoded_digits = get_unique_encoded_digits(first_ten_encoded_digits)
    unique_encoded_digits = sorted(unique_encoded_digits, key=lambda x: len(x[0]))

    configs = get_all_possible_configs(unique_digits=unique_encoded_digits)
    correct_config = get_correct_config(
        configs=configs, encoded_digits=first_ten_encoded_digits
    )

    decoded_final_four_digits = decode_encoded_digits(
        config=correct_config, encoded_digits=final_four_encoded_digits
    )

    return int(
        "".join(str(DIGITS_TO_INT[digit]) for digit in decoded_final_four_digits)
    )


def parse_input(filename: str) -> np.ndarray:
    return np.loadtxt(filename, dtype="str")


if __name__ == "__main__":
    INPUT_FILENAME = "input.txt"

    signals = parse_input(INPUT_FILENAME)
    print("Part 1:")
    unique_digits = [get_unique_encoded_digits(signal[11:]) for signal in signals]
    total_unique_digits = 0
    for digits in unique_digits:
        total_unique_digits += len(digits)
    print(f"Digits with unique segment numbers appear {total_unique_digits} times.")

    print("\nPart 2:")
    accumulated_digits = 0
    for signal in signals:
        accumulated_digits += get_digits_from_signal(signal)
    print(f"Accumulated digits from the signal is {accumulated_digits}")
