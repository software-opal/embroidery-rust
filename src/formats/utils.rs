use std::io::Read;

pub fn read_to_iter<'a>(reader: &'a mut Read) -> Box<'a + Iterator<Item = u8>> {
    return _read_to_iter(reader);
}

fn _read_to_iter<'a>(reader: &'a mut Read) -> Box<'a + Iterator<Item = u8>> {
    return Box::new(
        reader
            .bytes()
            .take_while(|item| item.is_ok())
            .map(|item| item.unwrap()),
    );
}
