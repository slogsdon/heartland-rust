extern crate xml;

use std::io::Write;
use xml::writer::{EventWriter, XmlEvent};

use super::super::abstractions::traits::Writeable;

pub fn start_element<W: Write>(tag: &'static str, w: &mut EventWriter<W>) {
    w.write(XmlEvent::start_element(tag)).unwrap();
}

pub fn end_element<W: Write>(w: &mut EventWriter<W>) {
    w.write(XmlEvent::end_element()).unwrap();
}

pub fn write_value<W: Write, Wb: Writeable>(tag: &'static str, value: &Wb, w: &mut EventWriter<W>) {
    start_element(tag, w);
    w.write(value.prepare()).unwrap();
    end_element(w);
}

pub fn maybe_write_value<W: Write, Wb: Writeable>(tag: &'static str, value: &Option<Wb>, w: &mut EventWriter<W>) {
    if let Some(ref v) = *value {
        write_value(tag, v, w);
    }
}
