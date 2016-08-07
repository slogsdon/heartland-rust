extern crate xml;

use std::io::Write;
use xml::writer::{EventWriter, XmlEvent};

pub fn start_element<W: Write>(tag: &'static str, w: &mut EventWriter<W>) {
    w.write(XmlEvent::start_element(tag)).unwrap();
}

pub fn end_element<W: Write>(w: &mut EventWriter<W>) {
    w.write(XmlEvent::end_element()).unwrap();
}

pub fn write_value<W: Write>(tag: &'static str, value: &String, w: &mut EventWriter<W>) {
    start_element(tag, w);
    w.write(value.as_str()).unwrap();
    end_element(w);
}

pub fn maybe_write_value<W: Write>(tag: &'static str, value: &Option<String>, w: &mut EventWriter<W>) {
    match *value {
        Some(ref v) => {
            write_value(tag, &v, w);
        }
        None => {}
    }
}