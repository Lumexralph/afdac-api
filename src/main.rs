use reqwest;
use scraper::{Html, Selector};

fn main() {
    let response = reqwest::blocking::get(
        "https://greenbook.nafdac.gov.ng",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = Html::parse_document(&response);
    let table_head_selector = Selector::parse("table.data-table>thead>tr>th").unwrap();

    let product_table_headers = document.select(&table_head_selector).map(|x| x.inner_html());
    product_table_headers
                .enumerate()
                .for_each(|(i, name)| println!("{}. {}", i, name));

    let product_table_body_selector = Selector::parse("table.data-table>tbody").unwrap();
    let product_table_body: Vec<_> = document.select(&product_table_body_selector).collect();

    for val in product_table_body {
        let row_selector = Selector::parse("tr").unwrap();
        let rows: Vec<_> = val.select(&row_selector).collect();

        println!("{}",rows.len());
    }

}
