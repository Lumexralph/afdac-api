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
    println!("Raw HTML: {:?}", document.html());
    let table_head_selector = Selector::parse("table.data-table>thead>tr>th").unwrap();

    let product_table_headers = document.select(&table_head_selector).map(|x| x.inner_html());
    product_table_headers
                .enumerate()
                .for_each(|(i, name)| println!("{}. {}", i, name));

    let data_table_body_selector = Selector::parse("table.data-table tbody tr").unwrap();
    // let product_table_body = document.select(&data_table_body_selector);
    let tr_selector = Selector::parse("tr").unwrap();

    //let table_body = document.select(&data_table_body_selector).next().unwrap();
    for element in document.select(&data_table_body_selector) {
        println!("{:?}", element.inner_html());
        for row in element.select(&tr_selector) {
            println!("{:?}", row.html());
        }
        //let story_txt = element.text().collect::<Vec<_>>();
        // println!("{:?}", element.);
        //println!("{}", element.value().name());
    }

    // if let Some(p) = product_table_body.peekable().peek_mut() {
    //     if p.has_children() {
    //         p.children().for_each(|el| println!("{:?}", el.value()))
    //     }
    // }

    // for val in product_table_body {
    //     let row_selector = Selector::parse("tr").unwrap();
    //     let rows: Vec<_> = val.select(&row_selector).collect();

    //     println!("{}",rows.len());
    // }

}
