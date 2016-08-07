extern crate hyper;
extern crate xml;
pub mod abstractions;
pub mod entities;
mod util;

use hyper::client::Client;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use std::io::Read;
use std::str;
use xml::writer::{EmitterConfig, XmlEvent};

use abstractions::traits::Transaction;
use util::xml::{start_element, end_element, write_value};

pub fn connect<T: Transaction>(t: T) {
    let url = "https://cert.api2.heartlandportico.com/Hps.Exchange.PosGateway/PosGatewayService.\
               asmx?wsdl";
    let body = build_xml(t);

    let client = Client::new();
    let builder = client.post(url)
        .body(str::from_utf8(&body).unwrap())
        .header(ContentType(Mime(TopLevel::Text,
                                 SubLevel::Xml,
                                 vec![(Attr::Charset, Value::Utf8)])));

    match builder.send() {
        Ok(mut res) => {
            if res.status == hyper::Ok {
                let mut buf = String::new();
                let _ = res.read_to_string(&mut buf)
                    .map_err(|_| print!("Failed to read response body"))
                    .map(|_| {
                        print!("code: {}\nbody: {}", res.status, buf);
                    });
            } else {
                let mut buf = String::new();
                let _ = res.read_to_string(&mut buf)
                    .map_err(|_| print!("Failed to read response body"))
                    .map(|_| {
                        print!("Request failed with {}\n{}", res.status, buf);
                    });
            }
        }
        _ => {
            print!("Failed to get a response");
        }
    }
}

pub fn build_xml<T: Transaction>(t: T) -> Vec<u8> {
    let mut b = Vec::new();

    {
        let mut w = EmitterConfig::new()
            .write_document_declaration(true)
            .perform_indent(true)
            .create_writer(&mut b);

        w.write(XmlEvent::start_element("soap:Envelope")
                .ns("soap", "http://schemas.xmlsoap.org/soap/envelope/")
                .ns("xsd", "http://www.w3.org/2001/XMLSchema")
                .ns("xsi", "http://www.w3.org/2001/XMLSchema-instance")
                .ns("", "http://Hps.Exchange.PosGateway"))
            .unwrap();
        start_element("soap:Body", &mut w);
        start_element("PosRequest", &mut w);
        start_element("Ver1.0", &mut w);
        start_element("Header", &mut w);

        write_value("SecretAPIKey", &String::from("skapi_cert_MT2PAQB-9VQA5Z1mOXQbzZcH6O5PpdhjWtFhMBoL4A"), &mut w);
        write_value("DeveloperID", &String::from("002914"), &mut w);
        write_value("VersionNbr", &String::from("1983"), &mut w);

        end_element(&mut w); // Header
        start_element("Transaction", &mut w);

        t.write_xml(&mut w);

        end_element(&mut w); // Transaction
        end_element(&mut w); // Ver1.0
        end_element(&mut w); // PosRequest
        end_element(&mut w); // soap:Body
        end_element(&mut w); // soap:Envelope
    }

    b
}
