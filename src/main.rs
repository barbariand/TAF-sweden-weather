use std::ops::Add;
use reqwest;
use tokio;
use scraper;
use std::fs;
#[tokio::main]
async fn main()->Result<(),reqwest::Error> {
    let body = reqwest::get("https://aro.lfv.se/Links/Link/ViewLink?TorLinkId=314&type=MET").await.unwrap().text().await.unwrap();
    //let body=String::from_utf8(fs::read("./text").unwrap()).unwrap();
    let test=scraper::Html::parse_document(&body);
    let headerselect=scraper::selector::Selector::parse(".item-header").unwrap();
    let textselect=scraper::selector::Selector::parse(".item-text").unwrap();
    let headers=test.select(&headerselect);
    let texts=test.select(&textselect);
    for (header,text) in headers.zip(texts){
        let mut headertext: String = String::new();
            for t in header.text() {
                headertext = headertext.add(t);
            }
            let mut ft: String = String::new();
            for t in text.text() {
                ft = ft.add(t);
            }
        println!("header: {}, text:{}",headertext,ft)
    }
    Ok(())
}
