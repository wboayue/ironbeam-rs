use crate::client::Client;
use crate::client::http::HttpTransport;
use crate::error::Result;
use crate::types::AllAccountsResponse;

impl<H: HttpTransport> Client<H> {
    /// Get all accounts for the authenticated trader.
    pub async fn all_accounts(&self) -> Result<Vec<String>> {
        let resp: AllAccountsResponse = self.get("/account/getAllAccounts").await?;
        Ok(resp.accounts)
    }
}
