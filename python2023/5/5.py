
from pathlib import Path
from typing import Self

from pydantic import BaseModel


class Map(BaseModel):
    source_start: int
    dest_start: int
    length: int

    def checked_map(self, x: int) -> int | None:
        if self.source_start > x or self.source_start + self.length < x:
            return None
        return x + (self.dest_start - self.source_start)


class Maps(BaseModel):
    mappings: list[Map]

    def map(self, x) -> int:
        for map in self.mappings:
            if ret := map.checked_map(x):
                return ret
        return x

    def merge(self, later: Self) -> None:
        new_mappings = []
        index = 0
        for old_map in self.mappings:
            if next := later.get_next(index):
                pass
            else:
                new_mappings

    def get_next(self, x) -> Map:
        return self.mappings[]

#   |------------|
#      |------------|
#   |-||---------||-|



example = """seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"""


def min_loc(input: str) -> int:
    sections = input.split("\n\n")

    seeds = [int(istr) for istr in sections[0].split(": ")[1].split(" ")]
    maps_raw = [section.split("\n")[1:] for section in sections[1:]]
    maps = [Maps(mappings=[
        Map(
            dest_start=int(map.split(" ")[0]),
            source_start=int(map.split(" ")[1]),
            length=int(map.split(" ")[2]),
        )
        for map in mapping if map])
        for mapping in maps_raw
    ]
    return min(seed_to_loc(seed, maps) for seed in seeds)

def min_loc_ranged(input: str) -> int:
    sections = input.split("\n\n")

    seeds = [int(istr) for istr in sections[0].split(": ")[1].split(" ")]
    seeds = [(seeds[i], seeds[i+1]) for i in range(0, len(seeds), 2)]
    maps_raw = [section.split("\n")[1:] for section in sections[1:]]
    maps = [Maps(mappings=[
        Map(
            dest_start=int(map.split(" ")[0]),
            source_start=int(map.split(" ")[1]),
            length=int(map.split(" ")[2]),
        )
        for map in mapping if map])
        for mapping in maps_raw
    ]
    return min(seed_to_loc(seed, maps) for seed_range in seeds for seed in range(seed_range[0], sum(seed_range)))


def seed_to_loc(seed: int, maps: list[Maps]) -> int:
    for map in maps:
        # print(seed, "->", map.map(seed), map)
        seed = map.map(seed)
    return seed


print(min_loc(example))
print(min_loc((Path(__file__).parent / "data").read_text()))
print(min_loc_ranged(example))
print(min_loc_ranged((Path(__file__).parent / "data").read_text()))
