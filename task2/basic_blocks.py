import json
from pprint import pprint
from sys import stdin


def get_basic_blocks(instrs):
    current = []
    for instr in instrs:
        if "label" in instr:
            if len(current) > 0:
                yield current
            current = [instr]
        elif "op" in instr and instr["op"] in ("jmp", "br", "ret"):
            current.append(instr)
            yield current
            current = []
        else:
            current.append(instr)
    if current:
        yield current


if __name__ == "__main__":
    program = json.load(stdin)
    functions = program["functions"]
    fn_instrs = [fn["instrs"] for fn in functions]
    fn_basic_blocks = [list(get_basic_blocks(fn)) for fn in fn_instrs]
    for fn_blocks in fn_basic_blocks:
        for block in fn_blocks:
            print(block)
        print()
