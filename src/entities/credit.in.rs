pub mod credit {
    #![allow(unused_mut)]
    extern crate xml;

    use std::io::{Write};
    use xml::{EventWriter};

    use super::super::abstractions::traits::Transaction;
    use super::super::util::xml::{start_element, end_element, maybe_write_value, write_value};

    #[derive(Debug, Default, Deserialize)]
    pub struct ManualEntry {
        pub card_number: String,
        pub exp_month: Option<String>,
        pub exp_year: Option<String>,
        pub cvv: Option<String>,
    }

    impl Transaction for ManualEntry {
        fn write_xml<W: Write>(&self, w: &mut EventWriter<W>) {
            start_element("ManualEntry", w);

            write_value("CardNbr", &self.card_number, w);
            maybe_write_value("ExpMonth", &self.exp_month, w);
            maybe_write_value("ExpYear", &self.exp_year, w);
            maybe_write_value("CVV2", &self.cvv, w);

            end_element(w);
        }
    }

    #[derive(Debug, Default, Deserialize)]
    pub struct CardData {
        pub manual_entry: Option<ManualEntry>,
    }

    impl Transaction for CardData {
        fn write_xml<W: Write>(&self, w: &mut EventWriter<W>) {
            start_element("CardData", w);

            if let Some(ref me) = self.manual_entry {
                me.write_xml(w);
            }

            end_element(w);
        }
    }

    #[derive(Debug, Default, Deserialize)]
    pub struct CreditSale {
        pub allow_duplicates: bool,
        pub amount: String,
        pub card_data: CardData,
    }

    impl Transaction for CreditSale {
        fn write_xml<W: Write>(&self, w: &mut EventWriter<W>) {
            start_element("CreditSale", w);
            start_element("Block1", w);

            let allow_dup = String::from(if self.allow_duplicates {
                "Y"
            } else {
                "N"
            });
            write_value("AllowDup", &allow_dup, w);
            write_value("Amt", &self.amount, w);
            self.card_data.write_xml(w);

            end_element(w); // Block1
            end_element(w); // CreditSale
        }
    }
}
