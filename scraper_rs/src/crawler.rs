use std::time::Duration;
use std::thread::sleep;

use anyhow::Result;
use headless_chrome::browser::tab::ResponseHandler;
use headless_chrome::protocol::cdp::Network::{GetResponseBodyReturnObject, ResourceType::Xhr};
use headless_chrome::{Browser, LaunchOptions};

const NAFDAC_DRUG_PRODUCT_URL: &str = "https://greenbook.nafdac.gov.ng";

fn handle_reponse() -> ResponseHandler {
    Box::new(|event_params, fetch_body| {
        if event_params.Type == Xhr && event_params.response.url.contains(format!("{}/?draw", NAFDAC_DRUG_PRODUCT_URL).as_str()) {
            let response = fetch_body().unwrap_or(GetResponseBodyReturnObject {
                body: "empty".to_string(),
                base_64_encoded: false,
            });
            println!("Response: {:?}", response);
        }
    })
}

pub fn query(input: &str) -> Result<()> {
    let mut launch_option = LaunchOptions::default_builder()
        .build()
        .expect("Could not find chrome-executable");
    // TODO: Remove this before production
    launch_option.headless = false;

    let browser = Browser::new(launch_option)?;
    let tab = browser.new_tab()?;
    tab.navigate_to(NAFDAC_DRUG_PRODUCT_URL)?
        .wait_for_element("div.dataTables_wrapper div.dataTables_length select")?
        .click()?;
    // Select the option with value input e.g "100" in the dropdown field entries.
    tab.register_response_handling("products_response", handle_reponse())?;
    tab.type_str(input)?.press_key("Enter")?;
    tab.wait_until_navigated()?;
    
    // NOTE: you can only fetch the body after it's been downloaded, which might be some time
    // after the initial 'response' (with status code, headers, etc.) has come back. hence this
    // sleep:
    // See: https://github.com/rust-headless-chrome/rust-headless-chrome/blob/f8012ec4ed867ccc0efa6b262723363afd775011/tests/simple.rs#L671
    sleep(Duration::from_secs(5));

    Ok(())
}
