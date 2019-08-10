use crate::pattern::Pattern;
use crate::stitch::Stitch;
use crate::stitch::Thread;

pub enum StitchInfo<'a> {
    Color(&'a Option<Thread>, &'a Stitch),
    Cut(&'a Stitch),
    End(&'a Stitch),
    Jump(&'a Stitch),
    Stitch(&'a Stitch),
}

pub const ZERO_STITCH: Stitch = Stitch::new(0.0, 0.0);

pub fn build_stitch_list<'a>(pattern: &'a Pattern) -> Vec<StitchInfo<'a>> {
    let mut re = vec![];
    let mut last_stitch = &ZERO_STITCH;
    let mut threads = vec![];

    for cg in &pattern.color_groups {
        threads.push(&cg.thread);
        for sg in &cg.stitch_groups {
            let mut iter = sg.stitches.iter();

            if let Some(s) = iter.next() {
                if threads.is_empty() {
                    re.push(StitchInfo::Jump(s));
                } else {
                    for thread in threads.drain(..) {
                        re.push(StitchInfo::Color(thread, s));
                    }
                }
                last_stitch = s;
                for s in iter {
                    re.push(StitchInfo::Stitch(s));
                    last_stitch = s;
                }
                if sg.cut {
                    re.push(StitchInfo::Cut(last_stitch));
                }
            }
        }
    }
    for thread in threads.drain(..) {
        re.push(StitchInfo::Color(thread, last_stitch));
    }
    re.push(StitchInfo::End(last_stitch));
    re
}
