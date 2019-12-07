use reqwest::Response;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use bytes::Bytes;

pub async fn get<T>(url: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
        T: Debug,
{
    let res: Response = reqwest::get(url).await?;
    let body: T = res.json::<T>().await?;
//    println!("{:?}", body);
    Ok(body)
}

pub async fn get_text(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let res: Response = reqwest::get(url).await?;
    let body: String = res.text().await?;
//    println!("{:?}", body);
    Ok(body)
}

pub async fn get_bytes(url: &str) -> Result<Bytes, Box<dyn std::error::Error>> {
    let bytes = reqwest::get(url).await?.bytes().await?;
    Ok(bytes)
}

pub async fn get_response(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let res: Response = reqwest::get(url).await?;
    Ok(res)
}
