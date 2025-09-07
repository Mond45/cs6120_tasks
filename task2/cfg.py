import json
from collections import defaultdict
from pprint import pprint
from sys import stdin
from typing import Any

from basic_blocks import get_basic_blocks


def find_block_with_labels(basic_blocks: list[Any], label: str):
    for i, block in enumerate(basic_blocks):
        if "label" in block[0] and block[0]["label"] == label:
            return i
    return -1


if __name__ == "__main__":
    program = json.load(stdin)
    functions = program["functions"]
    fn_names = [fn["name"] for fn in functions]
    fn_instrs = [fn["instrs"] for fn in functions]
    fn_basic_blocks = [list(get_basic_blocks(fn)) for fn in fn_instrs]

    # fn, block -> successor
    cfg = defaultdict(lambda: defaultdict(set))
    for (
        fn_name,
        fn_blocks,
    ) in zip(fn_names, fn_basic_blocks):
        for i, block in enumerate(fn_blocks):
            if block[-1]["op"] in ("ret"):
                pass
            elif block[-1]["op"] in ("jmp", "br"):
                cfg[fn_name][i].update(
                    [
                        find_block_with_labels(fn_blocks, label)
                        for label in block[-1]["labels"]
                    ]
                )
            elif i < len(fn_blocks) - 1:
                cfg[fn_name][i] = {i + 1}

    pprint([list(enumerate(fb)) for fb in fn_basic_blocks])
    pprint(cfg)
