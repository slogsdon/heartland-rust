extern crate xml;

use std::io::Write;
use xml::writer::EventWriter;

pub trait Transaction {
    fn write_xml<W: Write>(&self, w: &mut EventWriter<W>);
}
