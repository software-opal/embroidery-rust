use embroidery_lib::format::PatternFormat;

use embroidery_fmt_vp3::Vp3PatternFormat;

#[test]
fn test_send_vf3_file_read() {
    let mut data: &[u8] = include_bytes!("test_data/vf3/Send.vf3");
    let loader = Vp3PatternFormat::default().reader().unwrap();
    loader.read_pattern(&mut data).unwrap();
}
