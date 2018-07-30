use embroidery_rust::formats::dst::DstPatternLoader;
use embroidery_rust::formats::traits::PatternLoader;
use embroidery_rust::pattern::PatternAttribute;
use embroidery_rust::pattern::Stitch;

#[test]
fn test_file_load() {
    let mut data: &[u8] = include_bytes!("test_data/OSHLogo.dst");
    let loader = DstPatternLoader {};

    let pattern = loader.read_pattern(&mut data).unwrap();
    assert_eq!(pattern.name, "OSHLogo");
    assert_eq!(
        pattern.attributes,
        vec![PatternAttribute::Title("OSHLogo".to_owned())],
    );
    assert_eq!(pattern.color_groups.len(), 3);

    {
        let cg0 = &pattern.color_groups[0];
        assert_eq!(cg0.thread, None);
        assert_eq!(cg0.stitch_groups.len(), 1);
        let sg0 = &cg0.stitch_groups[0];
        assert_eq!(sg0.trim, true);
        assert_eq!(sg0.stitches.len(), 1275);
        assert_eq!(sg0.stitches[0], Stitch { x: 0., y: 0. });
        assert_eq!(sg0.stitches[1], Stitch { x: 23., y: 8. });
        assert_eq!(sg0.stitches[2], Stitch { x: 43., y: 8. });
        assert_eq!(
            sg0.stitches[1274],
            Stitch {
                x: -143.0,
                y: -60.0
            }
        );
    }
    {
        let cg1 = &pattern.color_groups[1];
        assert_eq!(cg1.thread, None);
        assert_eq!(cg1.stitch_groups.len(), 1);
        let sg1 = &cg1.stitch_groups[0];
        assert_eq!(sg1.trim, true);
        assert_eq!(sg1.stitches.len(), 944);
        assert_eq!(
            sg1.stitches[0],
            Stitch {
                x: -143.0,
                y: -60.0
            }
        );
    }
    {
        let cg2 = &pattern.color_groups[2];
        println!("{:?}", cg2);
        assert_eq!(cg2.thread, None);
        assert_eq!(cg2.stitch_groups.len(), 2);
        let sg2_0 = &cg2.stitch_groups[0];
        assert_eq!(sg2_0.trim, true);
        assert_eq!(sg2_0.stitches.len(), 94);
        let sg2_1 = &cg2.stitch_groups[1];
        assert_eq!(sg2_1.trim, true);
        assert_eq!(sg2_1.stitches.len(), 1487);
    }
}