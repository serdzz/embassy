#!/usr/bin/env python3
"""Repair the enumerated values in a TI-derived SVD so that svd2rust can generate from it.

TI's device descriptions have two defects that a code generator cannot live with, because svd2rust
turns each field into a Rust enum:

  * **Duplicate names.** RTC_C's RTCSSEL calls both encoding 0 and encoding 1 "LFXT", and both 2 and
    3 "RT1PS", because those encodings really do select the same clock. Good documentation, illegal
    Rust. Renamed to NAME_<value>, so both stay reachable.

  * **Duplicate values.** SAPH_A's PGCTL.TONE gives both "disabled" and "enabled" the value 0. That
    is a typo — the field is one bit and its own description says "PGCTL.TONE = 1". Where a one-bit
    field has exactly one encoding free, it is given to the duplicate; anything less clear-cut is
    dropped with a warning rather than guessed at.

Every repair is printed, because each one is a place where this crate stops matching what TI shipped.

    python3 tools/fix-svd-enums.py in.svd out.svd
"""

import sys
import xml.etree.ElementTree as ET


def fix_field(field):
    """Repair one field's enumerated values. Returns a list of what was done."""
    values = field.find("enumeratedValues")
    if values is None:
        return []

    name_el = field.find("name")
    field_name = name_el.text if name_el is not None else "?"
    width_el = field.find("bitWidth")
    width = int(width_el.text) if width_el is not None and width_el.text else 1

    notes = []
    seen_names = set()
    used = set()

    for entry in list(values.findall("enumeratedValue")):
        name = entry.find("name")
        number = entry.find("value")
        if name is None or name.text is None or number is None or number.text is None:
            continue

        # A repeated name: keep both, tell them apart by their encoding.
        if name.text in seen_names:
            new = f"{name.text}_{number.text}"
            notes.append(f"{field_name}: renamed duplicate name {name.text} to {new}")
            name.text = new
        seen_names.add(name.text)

        # A repeated encoding: one of the two is wrong, and only sometimes is it obvious which.
        value = int(number.text, 0)
        if value not in used:
            used.add(value)
            continue

        free = [v for v in range(1 << width) if v not in used]
        if len(free) == 1:
            number.text = str(free[0])
            used.add(free[0])
            notes.append(
                f"{field_name}: {name.text} had value {value}, which was taken; "
                f"gave it the only free encoding {free[0]}"
            )
        else:
            values.remove(entry)
            notes.append(
                f"{field_name}: dropped {name.text}, value {value} is already used and "
                f"{len(free)} encodings are free — too ambiguous to guess"
            )

    return notes


def main(path_in, path_out):
    tree = ET.parse(path_in)
    notes = []
    for field in tree.iter("field"):
        notes += fix_field(field)

    for note in notes:
        print(note)
    print(f"{len(notes)} repairs")

    tree.write(path_out, encoding="utf-8", xml_declaration=True)


if __name__ == "__main__":
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    main(sys.argv[1], sys.argv[2])
