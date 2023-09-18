/*
Query all S3 buckets for a particular string and return back matches
*/

use std::collections::HashMap;
use aws_sdk_s3::Client;


/*Get all S3 buckets in the account */

/*Query all S3 buckets for a particular string and return back matches */
pub async fn get_query_buckets(query: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let client = Client::new(&aws_config::load_from_env().await);
    let buckets = client.list_buckets().send().await?;
    let mut bucket_map = HashMap::new();
    //match a string to a bucket
    for bucket in buckets.buckets.unwrap() {
        let name = bucket.name.unwrap();
        let response = client.get_object().bucket(&name).key(query).send().await;
        match response {
            Ok(_) => {
                /*lets do a partial match of the string
                if the partial string is found in the name of the bucket i.e. "foo" in "foobar"
                then we will add the bucket name and query to the bucket_map
                */
                if name.contains(query) {
                    bucket_map.insert(name, query.to_string());
                }

            }
            Err(_) => {
                println!("No match found for bucket: {} with Query: {}", name, query);
            }
        }
    }
    Ok(bucket_map)
}
//Add tests here

