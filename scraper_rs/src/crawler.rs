use anyhow::Result;

use headless_chrome::browser::tab::ResponseHandler;
use headless_chrome::protocol::cdp::Network::GetResponseBodyReturnObject;
use headless_chrome::{Browser, LaunchOptions};

fn handle_reponse() -> ResponseHandler {
    Box::new(|event_params, fetch_body| {
        println!("Got event: {:?}", event_params);
        let response = fetch_body().unwrap_or(GetResponseBodyReturnObject{
            body: "empty".to_string(),
            base_64_encoded: false,
        });
        println!("Response: {:?}", response);
    })
}

pub fn query(input: &str) -> Result<()> {
    let mut launch_option = LaunchOptions::default_builder()
        .build()
        .expect("Could not find chrome-executable");
    launch_option.headless = false;
    launch_option.enable_logging = true;
    launch_option.window_size = Some((1085, 800));

    let browser = Browser::new(launch_option)?;
    let tab = browser.new_tab()?;
    tab.register_response_handling("products_response", handle_reponse())?;
    tab.navigate_to("https://greenbook.nafdac.gov.ng")?
        .wait_for_element("div.dataTables_wrapper div.dataTables_length select")?
        .click()?;
    tab.type_str(input)?.press_key("Enter")?;
    tab.wait_until_navigated()?;

    Ok(())
}
