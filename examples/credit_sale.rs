extern crate heartland;
use heartland::entities::credit::{CreditSale, CardData, ManualEntry};
use heartland::services::ServicesConfig;

fn main() {
    let c = ServicesConfig {
        secret_api_key: Some("skapi_cert_MTyMAQBiHVEAewvIzXVFcmUd2UcyBge_eCpaASUp0A"),
        .. ServicesConfig::default()
    };
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
    heartland::connect(c, t);
}
