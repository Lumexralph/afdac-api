use std::time::Duration;

use fantoccini::{ClientBuilder, Locator, elements::Element};
use scraper::{Html, Selector};

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

    let tbody_rows = client
        .find_all(Locator::Css(
            "table.dataTable tbody tr"
        ))
        .await?;

    let td_selector = Selector::parse("td").unwrap();
    for row in tbody_rows.as_slice() {
        let html = row.html(false).await?;
        let raw_string = format!(r#"<table>{}</table>"#, html);
        let fragment = Html::parse_fragment(&raw_string);

        println!("{}", "===========================");
        for element in fragment.select(&td_selector) {
            println!("{}", element.text().collect::<Vec<_>>().join(","));
        }
        // let value = row.html(false).await?;
        // println!("Result: {}", value)
    }

    println!("{}", tbody_rows.len());

    client.close().await?;

    // let html = r#"<table><tr role="row" class="odd"><td class="sorting_1 dtr-control">Afrab Oral Rehydration Salt</td><td><a href="/ingredient/products/191">Oral Rehydration Salts</a></td><td><a href="/products/details/2971">A11-100055</a></td><td>Granules</td><td>Oral</td><td>20.5 g</td><td><a href="/applicant/products/82">Afrab-Chem Limited</a></td><td>2021-03-29</td><td>Active</td></tr></table>"#;

    // let fragment = Html::parse_fragment(html);
    // let selector = Selector::parse("td").unwrap();

    // for element in fragment.select(&selector) {
    //     println!("{}", element.value().name());
    // }

    Ok(())
}
