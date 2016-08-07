extern crate xml;

use std::io::Write;
use xml::writer::EventWriter;

pub trait Transaction {
    fn write_xml<W: Write>(&self, w: &mut EventWriter<W>);
}

pub trait Writeable {
    fn prepare(&self) -> &str;
}

impl Writeable for String {
    fn prepare(&self) -> &str { self }
}

impl Writeable for &'static str {
    fn prepare(&self) -> &str {
        self
    }
}
