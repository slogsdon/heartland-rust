extern crate heartland;

use std::str;
use heartland::entities::credit::{CreditSale, CardData, ManualEntry};
use heartland::services::ServicesConfig;

#[test]
fn build_xml_credit_sale_success() {
    let mut c = ServicesConfig::new();
    c.secret_api_key = Some("skapi_cert_MT2PAQB-9VQA5Z1mOXQbzZcH6O5PpdhjWtFhMBoL4A");
    c.developer_id = Some("002914");
    c.version_number = Some("1983");

    let t = CreditSale {
        allow_duplicates: true,
        amount: "6.00",
        card_data: CardData {
            manual_entry: Some(ManualEntry {
                card_number: String::from("4111111111111111"),
                exp_month: Some(String::from("12")),
                exp_year: Some(String::from("2025")),
                cvv: Some(String::from("123")),
            }),
        },
    };

    let body = r#"<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns="http://Hps.Exchange.PosGateway" xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <soap:Body>
    <PosRequest>
      <Ver1.0>
        <Header>
          <SecretAPIKey>skapi_cert_MT2PAQB-9VQA5Z1mOXQbzZcH6O5PpdhjWtFhMBoL4A</SecretAPIKey>
          <DeveloperID>002914</DeveloperID>
          <VersionNbr>1983</VersionNbr>
        </Header>
        <Transaction>
          <CreditSale>
            <Block1>
              <AllowDup>Y</AllowDup>
              <Amt>6.00</Amt>
              <CardData>
                <ManualEntry>
                  <CardNbr>4111111111111111</CardNbr>
                  <ExpMonth>12</ExpMonth>
                  <ExpYear>2025</ExpYear>
                  <CVV2>123</CVV2>
                </ManualEntry>
              </CardData>
            </Block1>
          </CreditSale>
        </Transaction>
      </Ver1.0>
    </PosRequest>
  </soap:Body>
</soap:Envelope>"#;

    assert_eq!(body, str::from_utf8(&heartland::build_xml(c, t)).unwrap())
}
