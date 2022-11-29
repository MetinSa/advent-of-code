from ast import literal_eval
from typing import List


def read_numbers(filename: str) -> List[list]:
    with open(filename, "r") as file:
        numbers: List[list] = [literal_eval(line) for line in file]

    return numbers


def reduce(numbers: List[list]) -> List[list]:
    keep_reducing = True
    while keep_reducing:
        for item in numbers:
            if isinstance(item, list):
                for sub_item in item:
                    if isinstance(sub_item, list):
                        for prev_idx, sub_sub_item in enumerate(sub_item):
                            if isinstance(sub_sub_item, list):
                                for idx, sub_sub_sub_item in enumerate(sub_sub_item):
                                    if isinstance(sub_sub_sub_item, list):
                                        print("before", numbers)

                                        left_add, right_add = sub_sub_sub_item
                                        # if prev_idx != 0:
                                            # left_item = sub_item[prev_idx - 1][idx]
                                            # print(left_item)
                                            # print(left_item)
                                        if prev_idx != 0 or prev_idx != (len(sub_item) - 1):
                                            sub_sub_item[prev_idx + 1][0] += right_add
                                            sub_sub_item[prev_idx - 1][1] += left_add
                                            
                                        sub_sub_item[idx] = 0

                                    continue
                            keep_reducing = False
                    keep_reducing = False
            keep_reducing = False

    print("after", numbers)


def add_numbers(numbers: List[list]) -> List[list]:
    number = numbers.pop(0)
    while numbers:
        number_to_add = numbers.pop(0)
        number = [number] + [number_to_add]
        # print("num",number)
        reduce(number)

if __name__ == "__main__":
    INPUT_FILE = "test_input.txt"
    numbers = read_numbers(INPUT_FILE)
    add_numbers(numbers)
