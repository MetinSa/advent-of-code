from typing import Sequence

import numpy as np
from numpy.lib.stride_tricks import sliding_window_view

MEASUREMENTS_FILENAME = "depth_measurements.txt"
WINDOW_SIZE = 3


def get_n_increasing_measurements(measurements: Sequence) -> int:
    """Returns the number of increasing numbers from a sequence.

    Parameters
    ----------
    measurements
        A sequence of measurements.

    Returns
    -------
        number of increasing numbers in the sequence.
    """

    return np.sum(np.diff(measurements) > 0)


def get_n_increasing_sliding_windows(measurements: Sequence) -> int:
    """Returns the number of increasing sliding windows from a sequence.

    Parameters
    ----------
    measurements
        A sequence of measurements.
    window_size
        length of the sliding window.

    Returns
    -------
        number of increasing sliding windows in the sequence.
    """

    sliding_window = sliding_window_view(measurements, window_shape=WINDOW_SIZE)
    sliding_window_mean = np.sum(sliding_window, axis=1)

    return np.sum(np.diff(sliding_window_mean) > 0)


def main() -> None:

    measurements = np.loadtxt(MEASUREMENTS_FILENAME)
    n_increasing_measurements = get_n_increasing_measurements(measurements)
    n_increasing_sliding_window_measurements = get_n_increasing_sliding_windows(
        measurements
    )
    print(
        f"n_increasing_measurements: {n_increasing_measurements}\n"
        f"n_increasing_sliding_window_measurements: {n_increasing_sliding_window_measurements}",
    )


if __name__ == "__main__":
    main()