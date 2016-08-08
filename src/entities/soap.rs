extern crate xml;

use std::io::Write;
use xml::writer::{EventWriter, XmlEvent};

use super::super::abstractions::traits::Transaction;
use super::super::util::xml::{start_element, end_element};

pub struct SoapEnvelope<T: Transaction> {
    pub header: Option<SoapHeader>,
    pub body: Option<SoapBody<T>>,
}

impl<T: Transaction> Transaction for SoapEnvelope<T> {
    fn write_xml<W: Write>(&self, w: &mut EventWriter<W>) {
        w.write(XmlEvent::start_element("soap:Envelope")
                .ns("soap", "http://schemas.xmlsoap.org/soap/envelope/")
                .ns("xsd", "http://www.w3.org/2001/XMLSchema")
                .ns("xsi", "http://www.w3.org/2001/XMLSchema-instance")
                .ns("", "http://Hps.Exchange.PosGateway"))
            .unwrap();

        if let Some(ref header) = self.header {
            header.write_xml(w);
        }

        if let Some(ref body) = self.body {
            body.write_xml(w);
        }

        end_element(w);
    }
}

pub struct SoapHeader {}

impl Transaction for SoapHeader {
    fn write_xml<W: Write>(&self, w: &mut EventWriter<W>) {
        start_element("soap:Header", w);
        end_element(w);
    }
}

pub struct SoapBody<T: Transaction> {
    pub contents: Option<T>,
}

impl<T: Transaction> Transaction for SoapBody<T> {
    fn write_xml<W: Write>(&self, w: &mut EventWriter<W>) {
        start_element("soap:Body", w);

        if let Some(ref contents) = self.contents {
            contents.write_xml(w);
        }

        end_element(w);
    }
}
