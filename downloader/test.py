

import pathlib
import struct

downloads = pathlib.Path('downloads')


def pretty_hex(raw):
    return ' '.join(f'{b:02X}' for b in raw)


def parse_thread(thread_data):
    i = 0
    data = {}
    for field in ['thread_type', 'thread_num', 'color_name']:
        (length, ) = struct.unpack('<H', thread_data[i:i+2])
        i += 2
        value = thread_data[i:i+length]
        i += length
        data[field] = value
    assert len(thread_data[i:]) == 7
    assert thread_data[i:i+2] == bytes([0x08, 0x28]), thread_data[i:i+2].hex()
    assert thread_data[i+5:] == bytes([0x00, 0x00]), thread_data[i+5:] .hex()
    data['color'] = tuple(i for i in thread_data[i+2:i+5])
    # print(data)
    # print(pretty_hex(thread_data[i:i+2]), '|', pretty_hex(thread_data[i+5:]))




def parse(file):
    data = file.read_bytes()
    try:
        i = 221
        while i < len(data):
            # prev, i = i, data.index(b'thrd', i + 4)
            # print("I: ", i)
            assert data[i:i+8] == b'thrd\0\0\0\0', f'{data[i:i+8].hex()} | {i}'
            i += 8
            (data_len, ) = struct.unpack('<I', data[i:i+4])
            i += 4
            thread_data = data[i:i+data_len]
            i+=data_len
            parse_thread(thread_data)
            (data_len, ) = struct.unpack('<I', data[i:i+4])
            i += 4
            print(f"{struct.unpack('<I', data[i:i+4])[0]:032b}")
            i += 4

            # print(f"DL: {data_len:X}")
            # print(data[i + data_len:i+data_len+4])
            i += data_len
            # print(pretty_hex(thread_data))
            # print(f'   | {file!s: <50} | {prev: ^6} | {(i-prev): ^6X} | {i: ^6}')
        assert (i - 4) == len(data), f'{i} == {len(data)}'
        # assert i == len(data), f'{i} == {len(data)}'
    except ValueError:
        # No more threads found
        pass


def main():
    for vp4_file in downloads.glob('**/*.vp4'):
        if 'ballons' in str(vp4_file):
            print(vp4_file)
            parse(vp4_file)


if __name__ == '__main__':
    main()
