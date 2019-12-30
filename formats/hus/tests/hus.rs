use embroidery_lib::format::PatternReader;
use embroidery_lib::prelude::*;

use embroidery_fmt_hus::HusVipPatternReader;

use std::io::Cursor;
use std::collections::BTreeMap;

#[test]
fn test_hus_file_load() {
    let data: &[u8] = include_bytes!("test_data/Embroidermodder.hus");
    let loader = HusVipPatternReader {};

    assert!(loader.is_loadable(&mut Cursor::new(data)).unwrap());
    let pattern = loader.read_pattern(&mut Cursor::new(data)).unwrap();

    let cgs = pattern.color_groups;
    assert_eq!(cgs.len(), 1);
    assert_eq!(
        cgs[0].thread,
        Some(Thread {
            color: Color {
                red: 0,
                green: 0,
                blue: 127
            },
            name: "Dark Blue".to_string(),
            code: "HUS:13".to_string(),
            attributes: BTreeMap::new(),
        })
    );

    let sgs = &cgs[0].stitch_groups;
    assert_eq!(sgs.len(), 2);
    let sg = &sgs[0];
    assert_eq!(sg.cut, true);
    assert_eq!(sg.trim, true);
    assert_eq!(sg.stitches[0], Stitch::new(-88.9, -12.7));
    assert_eq!(sg.stitches.last(), Some(&Stitch::new(111.4, -21.1)));
}

// #[test]
// fn test_star_hus_file_load() {
//     let data: &[u8] = include_bytes!("test_data/Star.hus");
//     let pattern = check_star_file_load(data);
// }
#[test]
fn test_star_vip_file_load() {
    use difference::Changeset;
    let loader = HusVipPatternReader {};
    let vip_data: &[u8] = include_bytes!("test_data/Star.vip");
    let hus_data: &[u8] = include_bytes!("test_data/Star.hus");
    let vip_pattern = loader.read_pattern(&mut Cursor::new(vip_data)).unwrap();
    let hus_pattern = loader.read_pattern(&mut Cursor::new(hus_data)).unwrap();

    println!(
        "{}",
        Changeset::new(&format!("{:#?}", hus_pattern), &format!("{:#?}", vip_pattern), "\n",)
    );
    panic!()
}

fn check_star_file_load(data: &[u8]) -> Pattern {
    let loader = HusVipPatternReader {};

    // assert!(loader.is_loadable(&mut Cursor::new(data)).unwrap());
    let pattern = loader.read_pattern(&mut Cursor::new(data)).unwrap();

    println!("{:?}", pattern);

    let cgs = pattern.color_groups.clone();
    assert_eq!(cgs.len(), 2);
    assert_eq!(
        cgs[0].thread,
        Some(Thread {
            color: Color {
                red: 0,
                green: 0,
                blue: 127
            },
            name: "Dark Blue".to_string(),
            code: "HUS:13".to_string(),
            attributes: BTreeMap::new(),
        })
    );

    let sgs = &cgs[0].stitch_groups;
    assert_eq!(sgs.len(), 2);
    let sg = &sgs[0];
    assert_eq!(sg.cut, true);
    assert_eq!(sg.trim, true);
    assert_eq!(sg.stitches[0], Stitch::new(-88.9, -12.7));
    assert_eq!(sg.stitches.last(), Some(&Stitch::new(111.4, -21.1)));
    pattern
}
