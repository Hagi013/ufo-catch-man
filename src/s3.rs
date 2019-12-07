use rusoto_core::{Region, ByteStream};
use rusoto_s3::{S3, S3Client, PutObjectRequest};
use bytes::Bytes;

pub fn put_object(buf: &[u8],  bucket: String, key: String) -> Result<(), Box<dyn std::error::Error>> {
    let s3 = S3Client::new(Region::ApNortheast1);
    let request_params = PutObjectRequest {
        bucket: bucket.clone(),
        key: key.clone(),
        body: Some(ByteStream::from(buf.to_vec())),
        ..Default::default()
    };
    s3.put_object(request_params)
        .sync()
        .or(Err("S3 Put Error".to_string()))?;
    Ok(())
}

pub fn put_bytes(body: Bytes,  bucket: String, key: String) -> Result<(), Box<dyn std::error::Error>> {
    let s3 = S3Client::new(Region::ApNortheast1);
    let request_params = PutObjectRequest {
        bucket: bucket.clone(),
        key: key.clone(),
        body: Some(ByteStream::from(body.to_vec())),
        ..Default::default()
    };
    s3.put_object(request_params)
        .sync()
        .or(Err("S3 Put Error".to_string()))?;
    Ok(())
}

