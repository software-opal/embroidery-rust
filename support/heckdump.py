import sys
import pathlib

def group(iterable, size, *, type=tuple):
    group = []
    for v in iterable:
        group.append(v)
        if len(group) >= size:
            yield type(group)
            group = []
    if group:
        yield type(group)

def hexify(byte_arr: bytes) -> str:
    return ' '.join(group(byte_arr.hex(), 2, type=''.join))

def safe(byte_arr: bytes) -> str:
    return ''.join(
        chr(b) if 0x1f < b < 0x7f else '¿'
        for b in byte_arr
    )

def main():
    for filename in sys.argv[1:]:
        filebytes = pathlib.Path(filename).read_bytes()
        rows = list(group(filebytes, 16, type=bytes))

        for i, row in enumerate(rows):
            left, right = row[:8], row[8:]
            hex = f'{hexify(left)}  {hexify(right)}'
            hex = hex.ljust(16 * 3 + 1)
            print(
                f'{i:05X}0 | {hex} | {safe(left)} {safe(right)}'
            )
        print(f'{len(filebytes):06X} | ')


if __name__ == '__main__':
    main()
