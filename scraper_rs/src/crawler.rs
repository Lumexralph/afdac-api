use std::ffi::OsStr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use chrono::{naive::NaiveDate, DateTime, Utc};
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
            println!("Response: {:?}", response);
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
#[serde(deny_unknown_fields)]
struct DrugProductData<'a> {
    data: Vec<DrugProduct<'a>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct DrugProduct<'a> {
    product_id: u64,
    ingredient_id: u64,
    manufacturer_id: u64,
    product_name: &'a str,
    form_id: u64,
    strength: &'a str,
    nafdac_reg_no: &'a str,
    product_category_id: u32,
    marketing_category_id: u32,
    applicant_id: u64,
    #[serde(default)]
    approval_date: NaiveDate,
    #[serde(default)]
    expiry_date: NaiveDate,
    product_description: String,
    pack_size: &'a str,
    biosimilar: &'a str,
    atc: &'a str,
    #[serde(default)]
    created_at: DateTime<Utc>,
    #[serde(default)]
    updated_at: DateTime<Utc>,
    ingredient: Ingredient<'a>,
    form: Form<'a>,
    applicant: Applicant<'a>,
    route: Route<'a>,
    status: &'a str, // should be enum
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Applicant<'a> {
    id: u64,
    name: &'a str,
    address: &'a str,
    #[serde(default)]
    created_at: DateTime<Utc>,
    #[serde(default)]
    updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Route<'a> {
    id: u64,
    name: &'a str,
    #[serde(default)]
    updated_at: DateTime<Utc>,
    #[serde(default)]
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Ingredient<'a> {
    id: u32,       // ingredient_id
    name: &'a str, // ingredient_name
    synonym: &'a str,
    #[serde(default)]
    updated_at: DateTime<Utc>,
    #[serde(default)]
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Form<'a> {
    id: u32,       // form_id
    name: &'a str, // form_name
    #[serde(default)]
    updated_at: DateTime<Utc>,
    #[serde(default)]
    created_at: DateTime<Utc>,
}
