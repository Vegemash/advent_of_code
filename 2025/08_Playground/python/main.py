from pathlib import Path
from pydantic import BaseModel


class Connection(BaseModel):
    length_squared: int
    a_indx: int
    b_indx: int


class Cord(BaseModel):
    x: int
    y: int
    z: int

    def square_dist(self, other: "Cord") -> int:
        return (
            (self.x - other.x) ** 2 + (self.y - other.y) ** 2 + (self.z - other.z) ** 2
        )


def parse_input(pinput: str) -> tuple[list[Connection], int]:
    lines: list[str] = pinput.splitlines()
    boxes: list[Cord] = []
    connections: list[Connection] = []
    for line in lines:
        x, y, z = (int(c) for c in line.split(","))
        boxes.append(Cord(x=x, y=y, z=z))
    # [print(i, b) for i, b in enumerate(boxes)]

    for a_indx in range(len(boxes)):
        for b_indx in range(a_indx + 1, len(boxes)):
            assert a_indx != b_indx, f"{a_indx=}, {b_indx=}"
            connections.append(
                Connection(
                    length_squared=boxes[a_indx].square_dist(boxes[b_indx]),
                    a_indx=a_indx,
                    b_indx=b_indx,
                )
            )

    return connections, len(boxes)


def part_1(pinput: str, con_count: int) -> int:
    connections: list[Connection]
    box_count: int
    connections, box_count = parse_input(pinput)
    groups: dict[int, set[int]] = {b: {b} for b in range(box_count)}
    connections = sorted(connections, key=lambda x: x.length_squared)
    for _ in range(con_count):
        con = connections.pop(0)
        a_group = -1
        b_group = -1
        # Find the groups
        for k, v in groups.items():
            if con.a_indx in v:
                a_group = k
                break
        # if con.b_indx in groups[a_group]:
        #     raise Exception()
        for k, v in groups.items():
            if con.b_indx in v:
                b_group = k
                break
        # print(con)
        # print(groups)
        # merge the groups
        groups[a_group] = groups[a_group] | groups[b_group]
        # print(groups[a_group])
        # delete the higher group
        if b_group != a_group:
            del groups[b_group]
        # remove all invalid connections
        # valid_connections = []
        # for con in connections:
        #     if not (con.a_indx in groups[a_group] and con.b_indx in groups[a_group]):
        #         valid_connections.append(con)
        # connections = valid_connections
        print(f"connections len = {len(connections)} with {len(groups)} groups")

    group_lens = sorted([len(v) for v in groups.values()], reverse=True)
    return group_lens[0] * group_lens[1] * group_lens[2]


# def part_2(pinput: str) -> int:
#     start, lines = parse_input(pinput)
#
#     # forward pass collecting which splitters are active
#     beams: dict[int, int] = {start: 1}
#     for splitters in lines:
#         new_beams = beams.copy()
#         for beam, timelines in beams.items():
#             if beam in splitters:
#                 new_beams[beam - 1] = new_beams.setdefault(beam - 1, 0) + timelines
#                 new_beams[beam + 1] = new_beams.setdefault(beam + 1, 0) + timelines
#                 del new_beams[beam]
#         beams = new_beams.copy()
#
#     return sum(beams.values())


def main():
    print("Part 1: ", part_1(Path("../input1.txt").read_text(), 1000))
    # print("Part 2: ", part_2(Path("../input1.txt").read_text()))
    #


if __name__ == "__main__":
    main()
