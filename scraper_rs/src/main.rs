use anyhow::Result;

mod crawler;

fn main() -> Result<()> {
    let input = "100";
    crawler::query(input)
}
