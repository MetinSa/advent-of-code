from __future__ import annotations
from dataclasses import dataclass, field
from typing import List, Optional, Tuple
from math import prod


def read_input(filename: str) -> str:
    with open(filename, "r") as file:
        hex = file.read().strip()

    return (bin(int(hex, base=16))[2:]).zfill(4 * len(hex))


def binary_to_number(binary: str) -> int:
    return int(binary, base=2)


def get_literal_and_remainder(bits: str) -> Tuple[int, str]:
    remainder = bits
    finished = False
    digits = ""
    while not finished:
        finished = remainder[0] == "0"
        digit_chunk = remainder[1:5]
        remainder = remainder[5:]
        digits = digits + digit_chunk

    return binary_to_number(digits), remainder


@dataclass
class Packet:
    version: int
    type_id: int
    literal: Optional[int] = None
    sub_packets: List[Packet] = field(default_factory=list)


def get_packet_and_remainder(bits) -> Tuple[Packet, str]:
    version = int(bits[0:3], 2)
    type_id = int(bits[3:6], 2)

    if type_id == 4:
        literal, remainder = get_literal_and_remainder(bits[6:])
        packet = Packet(version=version, type_id=type_id, literal=literal)

        return packet, remainder

    packet = Packet(version=version, type_id=type_id)

    return get_packet_with_sub_packets(bits, packet)


def get_packet_with_sub_packets(bits: str, packet: Packet) -> Tuple[Packet, str]:
    length_type_id = bits[6]
    if length_type_id == "0":
        sub_packets_len = binary_to_number(bits[7 : 7 + 15])
        sub_remaineder = bits[7 + 15 : 7 + 15 + sub_packets_len]
        bit_remaineder = bits[7 + 15 + sub_packets_len :]

        while len(sub_remaineder) > 0:
            sub_packet, sub_remaineder = get_packet_and_remainder(bits=sub_remaineder)
            packet.sub_packets.append(sub_packet)

    else:
        n_subpackets = binary_to_number(bits[7 : 7 + 11])
        bit_remaineder = bits[7 + 11 :]
        for _ in range(n_subpackets):
            sub_packet, bit_remaineder = get_packet_and_remainder(bits=bit_remaineder)
            packet.sub_packets.append(sub_packet)

    return packet, bit_remaineder


def get_version_sum(packet: Packet) -> int:
    return packet.version + sum(
        get_version_sum(sub_packet) for sub_packet in packet.sub_packets
    )


def get_operator_version_sum(packet: Packet) -> int:
    type_id = packet.type_id
    subpackets = (
        get_operator_version_sum(subpacket) for subpacket in packet.sub_packets
    )
    if type_id == 0:
        return sum(subpackets)
    elif type_id == 1:
        return prod(subpackets)
    elif type_id == 2:
        return min(subpackets)
    elif type_id == 3:
        return max(subpackets)
    elif type_id == 4:
        if not packet.literal:
            raise ValueError("packet had type_id 4 but isnt a literal")
        return packet.literal
    elif type_id == 5:
        return int(
            get_operator_version_sum(packet.sub_packets[0])
            > get_operator_version_sum(packet.sub_packets[1])
        )
    elif type_id == 6:
        return int(
            get_operator_version_sum(packet.sub_packets[0])
            < get_operator_version_sum(packet.sub_packets[1])
        )
    elif type_id == 7:
        return int(
            get_operator_version_sum(packet.sub_packets[0])
            == get_operator_version_sum(packet.sub_packets[1])
        )
    else:
        raise ValueError("type_id must be in of 0 to 7")


if __name__ == "__main__":
    INPUT_FILE = "input.txt"
    input_binary = read_input(INPUT_FILE)
    packet, _ = get_packet_and_remainder(input_binary)
    print("Part 1:")
    version_sum = get_version_sum(packet)
    print(f"The version sum is {version_sum}")

    print("\nPart 2:")
    version_sum = get_operator_version_sum(packet)
    print(f"The version operated sum is {version_sum}")