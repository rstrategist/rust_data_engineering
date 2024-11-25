//Information about the AWS S3 service
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Error};

// Create a new AWS S3 client
pub async fn client() -> Result<Client, Error> {
    let region_provider = RegionProviderChain::first_try(None)
        .or_default_provider()
        .or_else("eu-north-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    Ok(client)
}

/* return a list of all buckets in an AWS S3 account
*/

pub async fn list_buckets(client: &Client) -> Result<Vec<String>, Error> {
    let mut bucket_names: Vec<String> = Vec::new();
    let resp = client.list_buckets().send().await?;
    
    // buckets() returns a slice reference directly, no need for Option handling
    for bucket in resp.buckets() {
        if let Some(name) = bucket.name() {
            bucket_names.push(name.to_string());
        }
    }
    
    Ok(bucket_names)
}


// Get the size of an AWS S3 bucket by summing all the objects in the bucket
// return the size in bytes
pub async fn bucket_size(client: &Client, bucket_name: &str) -> Result<i64, Error> {
    let mut size: i64 = 0;
    
    let objects = client
        .list_objects_v2()
        .bucket(bucket_name)
        .send()
        .await?;

    // Directly iterate over the contents
    for object in objects.contents() {
        if let Some(size_value) = object.size() {
            size += size_value;
        }
    }

    Ok(size)
}


/* Use list_buckets to get a list of all buckets in an AWS S3 account
return a vector of all bucket sizes.
If there is an error continue to the next bucket only print if verbose is true
Return the vector
*/
pub async fn list_bucket_sizes(client: &Client, verbose: Option<bool>) -> Result<Vec<i64>, Error> {
    let verbose = verbose.unwrap_or(false);
    let buckets = list_buckets(client).await.unwrap();
    let mut bucket_sizes: Vec<i64> = Vec::new();
    for bucket in buckets {
        match bucket_size(client, &bucket).await {
            Ok(size) => bucket_sizes.push(size),
            Err(e) => {
                if verbose {
                    println!("Error: {}", e);
                }
            }
        }
    }
    Ok(bucket_sizes)
}