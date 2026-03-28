use crate::client::Client;
use crate::client::http::HttpTransport;
use crate::error::Result;
use crate::types::{
    AccountBalanceResponse, AccountFillsResponse, AccountPositions, AccountRiskResponse,
    AccountsPositionsResponse, AllAccountsResponse, Balance, BalanceType, OrderFill,
    PositionsResponse, RiskInfo,
};

impl<H: HttpTransport> Client<H> {
    /// Get all accounts for the authenticated trader.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let accounts = client.all_accounts().await?;
    /// println!("Accounts: {accounts:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn all_accounts(&self) -> Result<Vec<String>> {
        let resp: AllAccountsResponse = self.get("/account/getAllAccounts").await?;
        Ok(resp.accounts)
    }

    /// Get account balance.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # use ironbeam_rs::types::BalanceType;
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let balances = client.balance("ACC001", BalanceType::CurrentOpen).await?;
    /// for b in &balances {
    ///     println!("{}: cash={:?} equity={:?}", b.account_id, b.cash_balance, b.total_equity);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn balance(
        &self,
        account_id: &str,
        balance_type: BalanceType,
    ) -> Result<Vec<Balance>> {
        let account_id = urlencoding::encode(account_id);
        let path = format!(
            "/account/{account_id}/balance?balanceType={}",
            balance_type.as_str()
        );
        let resp: AccountBalanceResponse = self.get(&path).await?;
        Ok(resp.balances)
    }

    /// Get open positions for an account.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let resp = client.positions("ACC001").await?;
    /// for p in &resp.positions {
    ///     println!("{:?}: {:?} @ {:?}", p.exch_sym, p.quantity, p.price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn positions(&self, account_id: &str) -> Result<PositionsResponse> {
        let account_id = urlencoding::encode(account_id);
        self.get(&format!("/account/{account_id}/positions")).await
    }

    /// Get risk information for an account.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let risks = client.risk("ACC001").await?;
    /// for r in &risks {
    ///     println!("{}: net_liq={:?}", r.account_id, r.current_net_liquidation_value);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn risk(&self, account_id: &str) -> Result<Vec<RiskInfo>> {
        let account_id = urlencoding::encode(account_id);
        let resp: AccountRiskResponse =
            self.get(&format!("/account/{account_id}/risk")).await?;
        Ok(resp.risks)
    }

    /// Get order fills for an account.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let fills = client.fills("ACC001").await?;
    /// for f in &fills {
    ///     println!("{}: {:?} @ {:?}", f.exch_sym, f.fill_quantity, f.fill_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fills(&self, account_id: &str) -> Result<Vec<OrderFill>> {
        let account_id = urlencoding::encode(account_id);
        let resp: AccountFillsResponse =
            self.get(&format!("/account/{account_id}/fills")).await?;
        Ok(resp.fills)
    }

    /// Get balances for all accounts.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # use ironbeam_rs::types::BalanceType;
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let balances = client.all_balances(BalanceType::CurrentOpen).await?;
    /// for b in &balances {
    ///     println!("{}: cash={:?}", b.account_id, b.cash_balance);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn all_balances(&self, balance_type: BalanceType) -> Result<Vec<Balance>> {
        let path = format!(
            "/account/getAllBalances?balanceType={}",
            balance_type.as_str()
        );
        let resp: AccountBalanceResponse = self.get(&path).await?;
        Ok(resp.balances)
    }

    /// Get positions for all accounts.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let positions = client.all_positions().await?;
    /// for ap in &positions {
    ///     println!("{}: {} positions", ap.account_id, ap.positions.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn all_positions(&self) -> Result<Vec<AccountPositions>> {
        let resp: AccountsPositionsResponse =
            self.get("/account/getAllPositions").await?;
        Ok(resp.positions)
    }

    /// Get risk information for all accounts.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let risks = client.all_risk().await?;
    /// for r in &risks {
    ///     println!("{}: net_liq={:?}", r.account_id, r.current_net_liquidation_value);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn all_risk(&self) -> Result<Vec<RiskInfo>> {
        let resp: AccountRiskResponse = self.get("/account/getAllRiskInfo").await?;
        Ok(resp.risks)
    }

    /// Get order fills for all accounts.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let fills = client.all_fills().await?;
    /// for f in &fills {
    ///     println!("{}: {:?} @ {:?}", f.exch_sym, f.fill_quantity, f.fill_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn all_fills(&self) -> Result<Vec<OrderFill>> {
        let resp: AccountFillsResponse = self.get("/account/getAllFills").await?;
        Ok(resp.fills)
    }
}

#[cfg(test)]
mod tests {
    use hyper::Method;
    use hyper::StatusCode;
    use hyper::header::AUTHORIZATION;

    use crate::client::http::mock::{MockHttp, MockResponse};
    use crate::client::test_support::test_client_with_auth;
    use crate::error::Error;
    use crate::types::BalanceType;

    // --- all_accounts ---

    #[tokio::test]
    async fn all_accounts_returns_list() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"accounts":["ACC1","ACC2"]}"#)]);
        let client = test_client_with_auth(mock);

        let accounts = client.all_accounts().await.unwrap();

        assert_eq!(accounts, vec!["ACC1", "ACC2"]);
        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        assert!(reqs[0].uri.to_string().ends_with("/account/getAllAccounts"));
    }

    #[tokio::test]
    async fn all_accounts_sends_auth_header() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"accounts":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.all_accounts().await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(
            reqs[0].headers.get(AUTHORIZATION).unwrap(),
            "Bearer tok_test"
        );
    }

    #[tokio::test]
    async fn get_maps_api_error() {
        let mock = MockHttp::new(vec![MockResponse::error(
            StatusCode::NOT_FOUND,
            r#"{"error1":"Not Found"}"#,
        )]);
        let client = test_client_with_auth(mock);

        let err = client.all_accounts().await.unwrap_err();

        match err {
            Error::Api { status, message } => {
                assert_eq!(status, 404);
                assert_eq!(message, "Not Found");
            }
            other => panic!("expected Api error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn get_maps_malformed_json() {
        let mock = MockHttp::new(vec![MockResponse::ok(b"not json".to_vec())]);
        let client = test_client_with_auth(mock);

        let err = client.all_accounts().await.unwrap_err();
        assert!(matches!(err, Error::Json(_)));
    }

    // --- balance ---

    #[tokio::test]
    async fn balance_returns_balances() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"balances":[{"accountId":"ACC1","currencyCode":"USD","cashBalance":50000.0}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let balances = client
            .balance("ACC1", BalanceType::CurrentOpen)
            .await
            .unwrap();

        assert_eq!(balances.len(), 1);
        assert_eq!(balances[0].account_id, "ACC1");
        assert_eq!(balances[0].cash_balance, Some(50000.0));
    }

    #[tokio::test]
    async fn balance_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"balances":[]}"#)]);
        let client = test_client_with_auth(mock);

        client
            .balance("ACC1", BalanceType::StartOfDay)
            .await
            .unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        let uri = reqs[0].uri.to_string();
        assert!(uri.contains("/account/ACC1/balance"));
        assert!(uri.contains("balanceType=START_OF_DAY"));
    }

    #[tokio::test]
    async fn balance_sends_auth_header() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"balances":[]}"#)]);
        let client = test_client_with_auth(mock);

        client
            .balance("ACC1", BalanceType::CurrentOpen)
            .await
            .unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(
            reqs[0].headers.get(AUTHORIZATION).unwrap(),
            "Bearer tok_test"
        );
    }

    // --- positions ---

    #[tokio::test]
    async fn positions_returns_response() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"accountId":"ACC1","positions":[{"accountId":"ACC1","exchSym":"XCME:ES.U16","quantity":2.0,"price":4500.0,"side":"LONG"}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let resp = client.positions("ACC1").await.unwrap();

        assert_eq!(resp.account_id.as_deref(), Some("ACC1"));
        assert_eq!(resp.positions.len(), 1);
        assert_eq!(resp.positions[0].exch_sym.as_deref(), Some("XCME:ES.U16"));
    }

    #[tokio::test]
    async fn positions_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"accountId":"ACC1","positions":[]}"#,
        )]);
        let client = test_client_with_auth(mock);

        client.positions("ACC1").await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        assert!(reqs[0]
            .uri
            .to_string()
            .ends_with("/account/ACC1/positions"));
    }

    // --- risk ---

    #[tokio::test]
    async fn risk_returns_risks() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"risks":[{"accountId":"ACC1","regCode":"COMBINED","currencyCode":"USD","liquidationValue":100000.0,"startNetLiquidationValue":95000.0,"currentNetLiquidationValue":98000.0,"maxNetLiquidationValue":100000.0,"maxNetLiquidationValueMultiDay":100000.0,"liquidationEvents":[]}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let risks = client.risk("ACC1").await.unwrap();

        assert_eq!(risks.len(), 1);
        assert_eq!(risks[0].account_id, "ACC1");
        assert_eq!(risks[0].current_net_liquidation_value, Some(98000.0));
    }

    #[tokio::test]
    async fn risk_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"risks":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.risk("ACC1").await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        assert!(reqs[0].uri.to_string().ends_with("/account/ACC1/risk"));
    }

    // --- fills ---

    #[tokio::test]
    async fn fills_returns_fills() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"fills":[{"orderId":"ORD1","strategyId":1,"accountId":"ACC1","exchSym":"XCME:ES.U16","status":"FILLED","side":"BUY","quantity":1.0,"price":4500.0,"fillQuantity":1.0,"fillTotalQuantity":1.0,"fillPrice":4500.0,"avgFillPrice":4500.0,"fillDate":"2025-01-15T10:30:00Z","timeOrderEvent":1705312200000,"orderUpdateId":"UPD1"}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let fills = client.fills("ACC1").await.unwrap();

        assert_eq!(fills.len(), 1);
        assert_eq!(fills[0].order_id, "ORD1");
        assert_eq!(fills[0].fill_price, Some(4500.0));
    }

    #[tokio::test]
    async fn fills_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"fills":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.fills("ACC1").await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        assert!(reqs[0].uri.to_string().ends_with("/account/ACC1/fills"));
    }

    // --- all_balances ---

    #[tokio::test]
    async fn all_balances_returns_balances() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"balances":[{"accountId":"ACC1","currencyCode":"USD","cashBalance":50000.0}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let balances = client.all_balances(BalanceType::CurrentOpen).await.unwrap();

        assert_eq!(balances.len(), 1);
        assert_eq!(balances[0].account_id, "ACC1");
    }

    #[tokio::test]
    async fn all_balances_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"balances":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.all_balances(BalanceType::StartOfDay).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        let uri = reqs[0].uri.to_string();
        assert!(uri.contains("/account/getAllBalances"));
        assert!(uri.contains("balanceType=START_OF_DAY"));
    }

    // --- all_positions ---

    #[tokio::test]
    async fn all_positions_returns_positions() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"positions":[{"accountId":"ACC1","positions":[{"accountId":"ACC1","exchSym":"XCME:ES.U16","quantity":2.0,"price":4500.0,"side":"LONG"}]}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let positions = client.all_positions().await.unwrap();

        assert_eq!(positions.len(), 1);
        assert_eq!(positions[0].account_id, "ACC1");
        assert_eq!(positions[0].positions.len(), 1);
    }

    #[tokio::test]
    async fn all_positions_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"positions":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.all_positions().await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        assert!(reqs[0]
            .uri
            .to_string()
            .ends_with("/account/getAllPositions"));
    }

    // --- all_risk ---

    #[tokio::test]
    async fn all_risk_returns_risks() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"risks":[{"accountId":"ACC1","regCode":"COMBINED","currencyCode":"USD","liquidationValue":100000.0,"startNetLiquidationValue":95000.0,"currentNetLiquidationValue":98000.0,"maxNetLiquidationValue":100000.0,"maxNetLiquidationValueMultiDay":100000.0,"liquidationEvents":[]}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let risks = client.all_risk().await.unwrap();

        assert_eq!(risks.len(), 1);
        assert_eq!(risks[0].account_id, "ACC1");
    }

    #[tokio::test]
    async fn all_risk_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"risks":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.all_risk().await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        assert!(reqs[0]
            .uri
            .to_string()
            .ends_with("/account/getAllRiskInfo"));
    }

    // --- all_fills ---

    #[tokio::test]
    async fn all_fills_returns_fills() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"fills":[{"orderId":"ORD1","strategyId":1,"accountId":"ACC1","exchSym":"XCME:ES.U16","status":"FILLED","side":"BUY","quantity":1.0,"price":4500.0,"fillQuantity":1.0,"fillTotalQuantity":1.0,"fillPrice":4500.0,"avgFillPrice":4500.0,"fillDate":"2025-01-15T10:30:00Z","timeOrderEvent":1705312200000,"orderUpdateId":"UPD1"}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let fills = client.all_fills().await.unwrap();

        assert_eq!(fills.len(), 1);
        assert_eq!(fills[0].order_id, "ORD1");
    }

    #[tokio::test]
    async fn all_fills_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"fills":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.all_fills().await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        assert!(reqs[0]
            .uri
            .to_string()
            .ends_with("/account/getAllFills"));
    }
}
