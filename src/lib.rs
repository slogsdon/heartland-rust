extern crate hyper;
extern crate xml;
pub mod abstractions;
pub mod entities;
pub mod services;
mod util;

use hyper::client::Client;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use std::io::Read;
use std::str;
use xml::writer::{EmitterConfig, XmlEvent};

use abstractions::traits::Transaction;
use services::ServicesConfig;
use util::xml::{start_element, end_element, maybe_write_value, write_value};

pub fn connect<T: Transaction>(s: ServicesConfig, t: T) {
    let url = "https://cert.api2.heartlandportico.com\
               /Hps.Exchange.PosGateway\
               /PosGatewayService.asmx?wsdl";
    let body = build_xml(s, t);

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

pub fn build_xml<T: Transaction>(s: ServicesConfig, t: T) -> Vec<u8> {
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

        if let Some(secret_api_key) = s.secret_api_key {
            write_value("SecretAPIKey", &secret_api_key.to_owned(), &mut w);
        } else {
            maybe_write_value("UserName", &s.username, &mut w);
            maybe_write_value("Password", &s.password, &mut w);
            maybe_write_value("SiteId", &s.site_id, &mut w);
            maybe_write_value("DeviceId", &s.device_id, &mut w);
            maybe_write_value("LicenseId", &s.license_id, &mut w);
        }
        maybe_write_value("DeveloperID", &s.developer_id, &mut w);
        maybe_write_value("VersionNbr", &s.version_number, &mut w);

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
