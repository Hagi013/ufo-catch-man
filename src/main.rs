extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate rusoto_core;
extern crate rusoto_s3;

pub mod disclosure;
pub mod http;
pub mod s3;

use std::env;
use http::{get, get_bytes};
use chrono::{Utc, Duration, DateTime};
use chrono::prelude::*;
use disclosure::{Disclosure, DisclosureResult};
use s3::put_bytes;
use bytes::{Bytes, Buf};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let target_year: i32 = env::args()
        .nth(1)
        .ok_or("You should input year in first args.".to_string())?
        .parse()
        .or(Err("You should input year(type number) in first args.".to_string()))?;
    let url_list = get_urls_disclosure(target_year);
    println!("{:?}", url_list);

    for url in url_list {
        exec_ufo_catch(&url, &target_year).await?;
    }
    Ok(())
}

fn get_urls_disclosure(target_year: i32) -> Vec<String> {
    let base_url = "https://disclosure.edinet-fsa.go.jp/api/v1/documents.json?type=2&date=";
    println!("{}", base_url);
    let start_date = Utc.ymd(target_year, 1, 1).and_hms(0, 0, 0);
    let days = (Utc.ymd(target_year + 1, 1, 1).and_hms(0, 0, 0) - start_date).num_days();
    let mut url_list: Vec<String> = Vec::new();
    for (_idx, num) in (0..days).enumerate() {
        url_list.push(format!("{}{}", base_url, (start_date + Duration::days(num)).format("%Y-%m-%d").to_string()));
    }
    url_list
}

async fn exec_ufo_catch(url: &str, target_year: &i32) -> Result<(), Box<dyn std::error::Error>> {
    let ufo_doc_list = fetch_ufo_doc_list(url).await?;
    for doc in ufo_doc_list.iter().as_ref() {
        let url: String = format!("{}{}{}", "https://disclosure.edinet-fsa.go.jp/api/v1/documents/", doc.clone().doc_id, "?type=1");
        let ufo_file: Bytes = fetch_ufo_file(&url).await?;
        let sec_code = doc.clone().sec_code.unwrap();
        let filer_name = doc.clone().filer_name.unwrap();
        let doc_description = doc.clone().doc_description.unwrap();
        let filer_name = doc.clone().filer_name.unwrap();
        let doc_description = doc.clone().doc_description.unwrap();
        let period_start = doc.clone().period_start.unwrap();
        let period_end = doc.clone().period_end.unwrap();
        let splited_period_start = period_start.split('-').collect::<Vec<&str>>();
        let splited_period_end = period_end.split('-').collect::<Vec<&str>>();

        let start_date = Utc.ymd(splited_period_start[0].parse().unwrap(), splited_period_start[1].parse().unwrap(), splited_period_start[2].parse().unwrap()).and_hms(0, 0, 0);
        let days = (Utc.ymd(splited_period_end[0].parse().unwrap(), splited_period_end[1].parse().unwrap(), splited_period_end[2].parse().unwrap()).and_hms(0, 0, 0) - start_date).num_days();
        if days < 364 { continue; }
        let path = format!("{}{}{}{}{}{}{}{}{}", "ufo/", target_year, "/", sec_code, "/", filer_name, "/", doc_description, ".zip");
        put_ufo_file(ufo_file, path);
    }
    Ok(())
}

async fn fetch_ufo_doc_list(url: &str) -> Result<Vec<DisclosureResult>, Box<dyn std::error::Error>> {
    let disclosure: Disclosure = get::<Disclosure>(url).await?;
    let doc_list = disclosure.get_ufo_doc_list();
//    println!("{:?}", doc_list);
    Ok(doc_list)
}

async fn fetch_ufo_file(url: &str) -> Result<Bytes, Box<dyn std::error::Error>> {
//    let bytes = get_bytes("https://disclosure.edinet-fsa.go.jp/api/v1/documents/S100FJBP?type=1").await?;
    let bytes = get_bytes(url).await?;
    Ok(bytes)
}

fn put_ufo_file(body: Bytes, path: String) -> Result<(), Box<dyn std::error::Error>> {
//    put_bytes(bytes, "dev-hands-crawled-additional-data-store".to_string(), "ufo/text/xbrl.zip".to_string());
    put_bytes(body, "dev-hands-crawled-additional-data-store".to_string(), path.to_string())
}

