use anyhow::anyhow;
use phf::phf_map;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Segment {
    Static,
    Local,
    Argument,
    This,
    That,
    Temp,
    Constant,
    Pointer,
}

static FROMSTR_MAP: phf::Map<&'static str, Segment> = phf_map! {
    "static" => Segment::Static,
    "local" => Segment::Local,
    "argument" => Segment::Argument,
    "this" => Segment::This,
    "that" => Segment::That,
    "temp" => Segment::Temp,
    "constant" => Segment::Constant,
    "pointer" => Segment::Pointer
};

impl FromStr for Segment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FROMSTR_MAP
            .get(s)
            .map(|x| *x)
            .ok_or(anyhow!("Failed to parse segment!"))
    }
}
