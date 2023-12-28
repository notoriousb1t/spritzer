from attr import dataclass
from typing import List


@dataclass
class DamageRow:
    green_mail: int
    blue_mail: int
    red_mail: int


@dataclass
class DamageTable:
    link_damage_rows: List[DamageRow]
