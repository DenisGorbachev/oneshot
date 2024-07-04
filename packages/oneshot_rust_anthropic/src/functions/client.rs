use clust::{ApiKey, Client, ClientBuilder, Version};

pub fn client(api_key: String) -> Client {
    ClientBuilder::new(ApiKey::new(api_key))
        .version(Version::V2023_06_01)
        .build()
}
