use std::error::Error;
use crate::approve::{RouterAddress, SpenderDetails};
use crate::client::OneInchClient;
use crate::consts::{BASIC_URL, SWAP_API_VERSION};

impl OneInchClient {

    /// Retrieves the router address for the specified network.
    /// Note: The function returns the result for the requested network, not the one set during `OneInchClient` initialization.
    pub async fn get_router_address(&self, details: SpenderDetails) -> Result<RouterAddress, Box<dyn Error>> {
        // Construct the URL for fetching router address.
        let url = format!("{}/swap/{}/{}/approve/spender", BASIC_URL, SWAP_API_VERSION, details.chain);

        // Send HTTP GET request with authorization header.
        let request_result = self.http_client
            .get(url)
            .header("Authorization", &self.token)
            .send()
            .await;

        // Handle request errors and check for successful response.
        let response = request_result
            .map_err(|e| Box::new(e) as Box<dyn Error>)?
            .error_for_status()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        // Parse JSON response into RouterAddress type.
        let address: RouterAddress = response.json().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        // Return the obtained router address.
        Ok(address)
    }

}