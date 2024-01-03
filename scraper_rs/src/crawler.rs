use std::ffi::OsStr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use chrono::naive::NaiveDate;
use headless_chrome::browser::tab::ResponseHandler;
use headless_chrome::protocol::cdp::Network::{GetResponseBodyReturnObject, ResourceType::Xhr};
use headless_chrome::{Browser, LaunchOptions};
use serde::{Deserialize, Serialize};

const NAFDAC_DRUG_PRODUCT_URL: &str = "https://greenbook.nafdac.gov.ng";

fn handle_reponse(page_count: AtomicUsize) -> ResponseHandler {
    Box::new(move |event_params, fetch_body| {
        if event_params.Type == Xhr
            && event_params
                .response
                .url
                .contains(format!("{}/?draw", NAFDAC_DRUG_PRODUCT_URL).as_str())
        {
            let response = fetch_body().unwrap_or(GetResponseBodyReturnObject {
                body: "empty".to_string(),
                base_64_encoded: false,
            });
            println!("response: {:?}", response.body);
            let _: DrugProductData = serde_json::from_str(response.body.as_str()).unwrap();
            // println!("product_data: {:?}", v);
            let count = page_count.fetch_add(1, Relaxed);
            println!("Page Count: {}", count);
        }
    })
}

pub fn query(input: &str) -> Result<()> {
    let mut launch_options = LaunchOptions::default_builder()
        .build()
        .expect("Could not find chrome-executable");
    launch_options.args = vec![
        // Add the following flags to prevent popups and notifications.
        // See: http://peter.sh/experiments/chromium-command-line-switches
        &OsStr::new("--disable-popup-blocking"),
        &OsStr::new("--disable-notifications"),
    ];

    let browser = Browser::new(launch_options)?;
    let tab = browser.new_tab()?;
    tab.navigate_to(NAFDAC_DRUG_PRODUCT_URL)?
        .wait_for_element("div.dataTables_wrapper div.dataTables_length select")?
        .click()?;

    let page_count = AtomicUsize::new(1);
    tab.register_response_handling("products_response", handle_reponse(page_count))?;
    // Select the option with value input e.g "100" in the dropdown field entries.
    tab.type_str(input)?
        .press_key("Enter")?
        .wait_until_navigated()?;

    // This is a JavsScript snippet that will be executed in the browser.
    // This is done because the alert function will block the browser.
    // This is a problem in a headless browser.
    tab.evaluate(
        r#"
        window.alert = console.log;
        alert("overwritten alerts function");
    "#,
        false,
    )?;

    // NOTE: you can only fetch the body after it's been downloaded, which might be some time
    // after the initial 'response' (with status code, headers, etc.) has come back. hence this
    // sleep:
    // See: https://github.com/rust-headless-chrome/rust-headless-chrome/blob/f8012ec4ed867ccc0efa6b262723363afd775011/tests/simple.rs#L671
    sleep(Duration::from_secs(1));

    let mut is_next_button_active = true;
    while is_next_button_active {
        let element = tab.wait_for_element("li#DataTables_Table_0_next")?;
        let attrs = element.get_attributes()?.unwrap();
        is_next_button_active = !attrs.iter().any(|attr| attr.contains("disabled"));
        element.click()?;

        // NOTE: you can only fetch the body after it's been downloaded.
        // See: https://github.com/rust-headless-chrome/rust-headless-chrome/blob/f8012ec4ed867ccc0efa6b262723363afd775011/tests/simple.rs#L671
        sleep(Duration::from_secs(5));
    }

    println!("No more pages to crawl");
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DrugProductData<'a> {
    #[serde(bound(deserialize = "'de: 'a"))]
    data: Vec<DrugProduct<'a>>,
}

mod string_format {
    use serde::{self, Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(
        s: &str,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(s.trim().to_string())
    }
}

mod date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &NaiveDate,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct DrugProduct<'a> {
    product_id: u64,
    ingredient_id: u64,
    manufacturer_id: u64,
    #[serde(with = "string_format")]
    product_name: String,
    form_id: &'a str,
    #[serde(with = "string_format")]
    strength: String,
    #[serde(rename(deserialize = "NAFDAC"))]
    #[serde(with = "string_format")]
    nafdac_reg_no: String,
    product_category_id: u32,
    marketing_category_id: u32,
    applicant_id: u64,
    #[serde(with = "date_format")]
    approval_date: NaiveDate,
    #[serde(with = "date_format")]
    expiry_date: NaiveDate,
    #[serde(with = "string_format")]
    product_description: String,
    #[serde(with = "string_format")]
    pack_size: String,
    #[serde(with = "string_format")]
    atc: String,
    #[serde(with = "string_format")]
    smpc: String,
    ingredient: Ingredient,
    form: Form,
    applicant: Applicant,
    // TODO: figure out how to handle when this field is null
    route: Route,
    status: &'a str, // should be enum
}

#[derive(Serialize, Deserialize, Debug)]
struct Applicant {
    id: u64,
    #[serde(with = "string_format")]
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Route {
    id: u64,
    #[serde(with = "string_format")]
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Ingredient {
    #[serde(rename(deserialize = "ingredient_id"))]
    id: u32,
    #[serde(rename(deserialize = "ingredient_name"))]
    #[serde(with = "string_format")]
    name: String,
    #[serde(with = "string_format")]
    synonym: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Form {
    id: u32,       // form_id
    #[serde(with = "string_format")]
    name: String, // form_name
}
