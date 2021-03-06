pub mod err;
pub mod part;

pub use self::err::{TexelError, Result};

use self::part::Part;
use std::fmt;

#[derive(Copy, Clone, Debug, Default)]
pub struct Texel {
    part: Part,
    glyph: char,
}

impl Texel {
    pub fn new(part: &str, glyph: char) -> Result<Self> {
        if let '\u{e000}'...'\u{efff}' = glyph {
            match Part::new(part) {
                Err(why) => Err(TexelError::PartFail(why)),
                Ok(part) => Ok(
                    Texel {
                        part: part,
                        glyph: glyph,
                    }
                ),
            }
        } else {
            Err(TexelError::ForbiddenGlyph(glyph))
        }
    }

    /// The accessor method `get_part` returns the Texel Part.
    pub fn get_part(&self) -> &Part {
        &self.part
    }

    /// The accessor method `get_glyph` returns the Texel Glyph.
    pub fn get_glyph(&self) -> char {
        self.glyph
    }

    pub fn set_glyph(&mut self, glyph: char) {
        self.glyph = glyph;
    }
}

impl PartialEq for Texel {
    fn eq(&self, rhs: &Texel) -> bool {
        self.part.eq(&rhs.part)
    }
}

impl fmt::Display for Texel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_glyph())
    }
}
