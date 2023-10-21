use std::time::Duration;

use fantoccini::{ClientBuilder, Locator, elements::Element};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to webdriver instance that is listening on port 4444
    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await?;

    // Go to NAFDAC's drug product website.
    client.goto("https://greenbook.nafdac.gov.ng").await?;

    // Select number of product displayed entries.
    let select_element = client
        .find(Locator::Css(
            "div.dataTables_wrapper div.dataTables_length select",
        ))
        .await?;
    
    // Display 100 items.
    select_element
        .clone()
        .select_by_label("100")
        .await?;

    // Get display text after selection
    if let Some(initial_text) = select_element.prop("value").await? {
        println!("The selected option {}", initial_text);
    }

    let  table_body: Element = client
        .wait()
        .at_most(Duration::from_secs(5))
        .every(Duration::from_millis(100))
        .for_element(Locator::Css(
            "table.dataTable tbody tr",
        ))
        .await?;

    let content = table_body.html(false).await?;
    println!("{}", content);

    let tbody_rows = client
        .find_all(Locator::Css(
            "table.dataTable tbody tr"
        ))
        .await?;

    for row in tbody_rows.as_slice() {
        let value = row.html(false).await?;
        println!("Result: {}", value)
    }

    println!("{}", tbody_rows.len());

    client.close().await?;

    Ok(())
}

