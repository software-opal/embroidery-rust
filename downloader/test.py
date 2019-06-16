import pathlib
import struct
import enum
folders = [
    pathlib.Path("downloads").resolve(),
    pathlib.Path("../crawler/output").resolve(),
    pathlib.Path("../../../Downloads").resolve(),
]

@enum.unique
class StitchFlags(enum.Enum):
    NORMAL = enum.auto()
    JUMP = enum.auto()

def nice_hex(raw):
    if len(raw) > 100:
        return f"{pretty_hex(raw[:50])}...({len(raw) - 100} bytes)...{pretty_hex(raw[-50:])}"
    elif len(raw) < 10:
        return f"{pretty_hex(raw)}({raw!r})"
    else:
        return pretty_hex(raw)


def pretty_hex(raw):
    return " ".join(f"{b:02X}" for b in raw)


class File:
    def __init__(self, data):
        self.data = data
        self.i = 0

    def assert_bytes(self, check):
        f_bytes = self.get_bytes(len(check))
        assert f_bytes == check, f"{nice_hex(f_bytes)!r} !== {nice_hex(check)!r}"

    def get_bytes(self, length):
        i = self.i
        bytes = self.data[i : i + length]
        # print(f"\t\tRead({i}/{length}) | {nice_hex(bytes)}")
        assert (
            len(bytes) == length
        ), f"data[{i}:{i+len(bytes)}] = {nice_hex(bytes)!r}. Expected {length} bytes; got {len(bytes)}"
        self.i += length
        return bytes

    def get_le_uint(self, length):
        int_data = self.get_bytes(length)
        int_data = bytes(reversed(int_data))
        return int(int_data.hex(), base=16)
    def get_le_int(self, length):
        bitlength = length * 8
        i = self.get_le_uint(length)
        if i & (1 << (bitlength -1)):
            i = -((2**bitlength) - i)
        return i

    def remaining_bytes(self):
        return len(self.data) - self.i

    def read_length_bytes(self, length_bytes):
        block_len = self.get_le_uint(length_bytes)
        block_data = self.get_bytes(block_len)
        return block_data

    def assert_exhausted(self):
        assert self.remaining_bytes() == 0, nice_hex(self.data[self.i :])

def test():
    a = File(bytes.fromhex('FF 80'))
    b = a.get_le_int(1)
    c = a.get_le_int(1)
    assert [b, c] == [-1, -128], f'{[b, c]}'
test()
del test



def parse_thread(thread_data):
    thread_data = File(thread_data)
    thread = {}
    for field in ["brand", "code", "name"]:
        thread[field] = thread_data.read_length_bytes(2)
    assert thread_data.remaining_bytes() == 7
    thread_data.assert_bytes(bytes.fromhex("08 28"))
    thread["color"] = tuple(thread_data.get_bytes(3))
    thread_data.assert_bytes(bytes.fromhex("00 00"))
    # print("Thread: ", thread)


def parse_info(info_data):
    info_data = File(info_data)
    info_data.assert_bytes(b"nttn")
    info_data.assert_bytes(b"\0" * 4)
    parse_nttn(info_data.get_bytes(27))
    info_data.assert_bytes(bytes.fromhex('00'))
    unknown = info_data.get_bytes(8)
    info_data.assert_bytes(bytes.fromhex('01'))
    info_data.assert_exhausted()


def parse_nttn(nttn_data):
    nttn_data = File(nttn_data)
    assert nttn_data.remaining_bytes() == 16 + 11
    block_len = nttn_data.assert_bytes(bytes.fromhex("0E 00 00 00"))
    ntes_len = nttn_data.assert_bytes(bytes.fromhex("02 00"))
    nttn_data.assert_bytes(b"ntes")
    ntes_data = nttn_data.assert_bytes(bytes.fromhex("00 00"))
    nttn_data.assert_bytes(b"stgs")
    stgs_data = nttn_data.get_bytes(11)
    nttn_data.assert_exhausted()


def parse_hoop(hoop_data):
    hoop = {}
    hoop_data = File(hoop_data)
    hoop["name"] = hoop_data.read_length_bytes(2)
    hoop["machine"] = hoop_data.read_length_bytes(2)
    hoop["flag1"] = hoop_data.get_bytes(1)[0]
    assert hoop["flag1"] in [0x00, 0x01], hoop["flag1"]
    hoop["size"] = (hoop_data.get_le_uint(2), hoop_data.get_le_uint(2))
    hoop["flag2"] = hoop_data.get_bytes(1)[0]
    assert hoop["flag2"] in [0x00, 0x01], hoop["flag2"]
    hoop_data.assert_exhausted()
    # print("Hoop: ", hoop)


def parse_stitches(stitches_data):
    stitches_data = File(stitches_data)
    stitches = []
    while stitches_data.remaining_bytes():
        x, y = stitches_data.get_le_int(1), stitches_data.get_le_int(1)
        # +ve x is to the right
        # +ve y is down
        if (x, y) == (-128, 1):
            # Jump/Cut/Move
            x, y = stitches_data.get_le_int(2), stitches_data.get_le_int(2)
            stitches.append((x/10., y/10., StitchFlags.JUMP))
        else:
            stitches.append((x/10., y/10., StitchFlags.NORMAL))
    print(stitches)



def parse_background(background_data):
    pass


def parse(file):
    data = File(file.read_bytes())
    data.assert_bytes(b"%Vp4%")
    data.assert_bytes(bytes.fromhex("01 00 00 00"))
    data.assert_bytes(b"Msa")
    data.assert_bytes(bytes.fromhex("0A DF 74 29 3C 87 6B 44 2C 84 2F 00 3C 7C F7 E7 A0"))
    while data.remaining_bytes():
        block_name = data.get_bytes(4)
        block_extra = data.get_bytes(4)  # data.assert_bytes( b"\0\0\0\0")
        print(block_name, block_extra)
        if block_name in [b"sbds", b"sbdn"]:
            file_remaining = data.get_le_uint(4)
            assert (
                data.remaining_bytes() == file_remaining
            ), f"{data.remaining_bytes()} == {file_remaining}"
            if block_name == b"sbdn":
                data.assert_bytes(b"\0" * 17)
            else:
                data.assert_bytes(bytes.fromhex("01 00"))
            # nttn = data.get_bytes(5)
        elif block_name == b"nttn":
            parse_nttn(data.get_bytes(27))
        else:
            block_len = data.get_le_uint(4)
            block_data = data.get_bytes(block_len)
            if block_name == b"info":
                block_data += data.get_bytes(1)
                parse_info(block_data)
            elif block_name == b"hoop":
                block_data += data.get_bytes(1)
                parse_hoop(block_data)
            elif block_name == b"thrd":
                parse_thread(block_data)
                stitches_len = data.get_le_uint(4)
                stitches_data = data.get_bytes(stitches_len)
                if data.remaining_bytes():
                    stitches_data += data.get_bytes(4)
                parse_stitches(stitches_data)
                print(nice_hex(stitches_data))
            elif block_name == b"bkgd":
                parse_background(block_data)
            else:
                assert False, f"Unknown {block_name} / {block_len}"
    return


def get_vp4_files():
    for path in folders:
        for vp4_file in path.glob("**/*.vp4"):
            # if 'ballons' in str(vp4_file):
            yield vp4_file


def main():
    files = sorted(get_vp4_files(), key=lambda p: p.stat().st_size, reverse=True)
    for vp4_file in files[-2:]:
        print(vp4_file, vp4_file.stat().st_size)
        parse(vp4_file)
        # return


if __name__ == "__main__":
    main()
