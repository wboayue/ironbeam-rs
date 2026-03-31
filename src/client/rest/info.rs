use crate::client::Client;
use crate::client::http::HttpTransport;
use crate::error::Result;
use crate::types::{
    ComplexGroupInfo, ComplexGroups, ComplexGroupsResponse, ComplexesResponse,
    ExchangeSourcesResponse, FutureInfo, SecurityDefinition, SecurityDefinitionsResponse,
    SecurityMarginAndValue, SecurityMarginAndValueResponse, SecurityStatus, SecurityStatusResponse,
    Spread, StrategyIdResponse, Symbol, SymbolFuturesResponse, SymbolInfo,
    SymbolOptionSpreadsResponse, SymbolOptionsResponse, SymbolOptionsResult,
    SymbolSearchOptionsResponse, SymbolsResponse, TraderInfo, TraderInfoResponse, UserInfo,
    UserInfoResponse,
};

// ---------------------------------------------------------------------------
// SymbolSearchParams — builder for /info/symbols query
// ---------------------------------------------------------------------------

/// Parameters for symbol search.
///
/// # Example
///
/// ```
/// use ironbeam_rs::client::SymbolSearchParams;
///
/// let params = SymbolSearchParams::new()
///     .text("GOLD")
///     .limit(10)
///     .prefer_active(true);
/// ```
#[derive(Debug, Default, Clone)]
pub struct SymbolSearchParams<'a> {
    text: Option<&'a str>,
    limit: Option<u32>,
    prefer_active: Option<bool>,
}

impl<'a> SymbolSearchParams<'a> {
    /// Create empty search params.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by text.
    #[must_use]
    pub fn text(mut self, text: &'a str) -> Self {
        self.text = Some(text);
        self
    }

    /// Limit number of results.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Prefer active contracts.
    #[must_use]
    pub fn prefer_active(mut self, prefer_active: bool) -> Self {
        self.prefer_active = Some(prefer_active);
        self
    }

    fn to_query_string(&self) -> String {
        let mut parts = Vec::new();
        if let Some(text) = self.text {
            parts.push(format!("text={}", urlencoding::encode(text)));
        }
        if let Some(limit) = self.limit {
            parts.push(format!("limit={limit}"));
        }
        if let Some(prefer_active) = self.prefer_active {
            parts.push(format!("preferActive={prefer_active}"));
        }
        if parts.is_empty() {
            String::new()
        } else {
            format!("?{}", parts.join("&"))
        }
    }
}

// ---------------------------------------------------------------------------
// Client methods
// ---------------------------------------------------------------------------

impl<H: HttpTransport> Client<H> {
    /// Get trader info (accounts list, live/demo status).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let info = client.trader_info(None).await?;
    /// println!("Accounts: {:?}", info.accounts);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn trader_info(&self, trader_id: Option<&str>) -> Result<TraderInfo> {
        let path = match trader_id {
            Some(id) => format!("/info/trader?traderId={}", urlencoding::encode(id)),
            None => "/info/trader".to_string(),
        };
        let resp: TraderInfoResponse = self.get(&path).await?;
        Ok(resp.info)
    }

    /// Get user general info (contact info, account metadata).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let info = client.user_info(None).await?;
    /// println!("Title: {:?}", info.account_title);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn user_info(&self, trader_id: Option<&str>) -> Result<UserInfo> {
        let path = match trader_id {
            Some(id) => format!("/info/user?traderId={}", urlencoding::encode(id)),
            None => "/info/user".to_string(),
        };
        let resp: UserInfoResponse = self.get(&path).await?;
        Ok(resp.info)
    }

    /// Get security definitions for given symbols (max 10).
    ///
    /// Symbols use `EXCHANGE:SYMBOL` format, e.g. `"XCME:ES.U16"`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let defs = client.security_definitions(&["XCME:ES.U16"]).await?;
    /// for d in &defs {
    ///     println!("{}: {:?}", d.exch_sym, d.product_description);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn security_definitions(&self, symbols: &[&str]) -> Result<Vec<SecurityDefinition>> {
        let resp: SecurityDefinitionsResponse = self
            .symbol_query("/info/security/definitions", symbols)
            .await?;
        Ok(resp.security_definitions)
    }

    /// Get margin and value info for symbols (max 10).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let margins = client.security_margin(&["XCME:ES.U16"]).await?;
    /// for m in &margins {
    ///     println!("{}: init_long={:?}", m.exch_sym, m.initial_margin_long);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn security_margin(&self, symbols: &[&str]) -> Result<Vec<SecurityMarginAndValue>> {
        let resp: SecurityMarginAndValueResponse =
            self.symbol_query("/info/security/margin", symbols).await?;
        Ok(resp.security_margin_and_values)
    }

    /// Get trading status for symbols (max 10).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let statuses = client.security_status(&["XCME:ES.U16"]).await?;
    /// for s in &statuses {
    ///     println!("{}: {:?}", s.exch_sym, s.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn security_status(&self, symbols: &[&str]) -> Result<Vec<SecurityStatus>> {
        let resp: SecurityStatusResponse =
            self.symbol_query("/info/security/status", symbols).await?;
        Ok(resp.security_statuses)
    }

    /// Search for symbols.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials, SymbolSearchParams};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let params = SymbolSearchParams::new().text("GOLD").limit(5);
    /// let symbols = client.symbols(&params).await?;
    /// for s in &symbols {
    ///     println!("{}: {:?}", s.symbol, s.description);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn symbols(&self, params: &SymbolSearchParams<'_>) -> Result<Vec<SymbolInfo>> {
        let qs = params.to_query_string();
        let resp: SymbolsResponse = self.get(&format!("/info/symbols{qs}")).await?;
        Ok(resp.symbols)
    }

    /// Get list of available exchanges.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let exchanges = client.exchange_sources().await?;
    /// println!("Exchanges: {exchanges:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn exchange_sources(&self) -> Result<Vec<String>> {
        let resp: ExchangeSourcesResponse = self.get("/info/exchangeSources").await?;
        Ok(resp.exchanges)
    }

    /// Get market complexes for an exchange.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let complexes = client.complexes("XCME").await?;
    /// for c in &complexes {
    ///     println!("{:?}: {} groups", c.name, c.groups.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn complexes(&self, exchange: &str) -> Result<Vec<ComplexGroups>> {
        let exchange = urlencoding::encode(exchange);
        let resp: ComplexesResponse = self.get(&format!("/info/complexes/{exchange}")).await?;
        Ok(resp.market_complexes)
    }

    /// Search for futures symbols by exchange and market group.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let futures = client.futures_symbols("XCME", "ES").await?;
    /// for f in &futures {
    ///     println!("{}: {:?}", f.symbol, f.description);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn futures_symbols(
        &self,
        exchange: &str,
        market_group: &str,
    ) -> Result<Vec<FutureInfo>> {
        let exchange = urlencoding::encode(exchange);
        let market_group = urlencoding::encode(market_group);
        let resp: SymbolFuturesResponse = self
            .get(&format!(
                "/info/symbol/search/futures/{exchange}/{market_group}"
            ))
            .await?;
        Ok(resp.symbols)
    }

    /// Get symbol groups by market complex.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let groups = client.symbol_groups("Currency").await?;
    /// for g in &groups {
    ///     println!("{:?}: {:?}", g.group, g.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn symbol_groups(&self, complex: &str) -> Result<Vec<ComplexGroupInfo>> {
        let complex = urlencoding::encode(complex);
        let resp: ComplexGroupsResponse = self
            .get(&format!("/info/symbol/search/groups/{complex}"))
            .await?;
        Ok(resp.symbol_groups)
    }

    /// Get option groups for a symbol.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let result = client.option_groups("XCME:ES.U16").await?;
    /// println!("Groups: {:?}", result.groups);
    /// for og in &result.option_groups {
    ///     println!("{:?}: {:?}", og.group, og.description);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn option_groups(&self, symbol: &str) -> Result<SymbolOptionsResult> {
        let symbol = urlencoding::encode(symbol);
        let resp: SymbolOptionsResponse = self
            .get(&format!("/info/symbol/search/options/{symbol}"))
            .await?;
        Ok(SymbolOptionsResult {
            groups: resp.groups,
            option_groups: resp.option_groups,
        })
    }

    /// Search for specific options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let options = client.search_options("XCME:ES.U16", "ES", "call", true).await?;
    /// println!("Found {} options", options.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_options(
        &self,
        symbol: &str,
        group: &str,
        option_type: &str,
        near: bool,
    ) -> Result<Vec<Symbol>> {
        let symbol = urlencoding::encode(symbol);
        let group = urlencoding::encode(group);
        let option_type = urlencoding::encode(option_type);
        let resp: SymbolSearchOptionsResponse = self
            .get(&format!(
                "/info/symbol/search/options/ext/{symbol}/{group}/{option_type}/{near}"
            ))
            .await?;
        Ok(resp.symbol_options)
    }

    /// Get available option spreads for a symbol.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let spreads = client.option_spreads("XCME:ES.U16").await?;
    /// println!("Spreads: {spreads:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn option_spreads(&self, symbol: &str) -> Result<Vec<Spread>> {
        let symbol = urlencoding::encode(symbol);
        let resp: SymbolOptionSpreadsResponse = self
            .get(&format!("/info/symbol/search/options/spreads/{symbol}"))
            .await?;
        Ok(resp.symbol_spreads)
    }

    /// Get a new strategy ID for order grouping.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let strategy = client.strategy_id().await?;
    /// println!("ID: {}, range: {}..{}", strategy.id, strategy.minimum, strategy.maximum);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn strategy_id(&self) -> Result<StrategyIdResponse> {
        self.get("/info/strategyId").await
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

    use super::SymbolSearchParams;

    // --- trader_info ---

    #[tokio::test]
    async fn trader_info_returns_info() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"accounts":["ACC1"],"isLive":true,"traderId":"T1"}"#,
        )]);
        let client = test_client_with_auth(mock);

        let info = client.trader_info(None).await.unwrap();

        assert_eq!(info.accounts, vec!["ACC1"]);
        assert!(info.is_live);
        assert_eq!(info.trader_id, "T1");
    }

    #[tokio::test]
    async fn trader_info_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"accounts":[],"isLive":false,"traderId":"T1"}"#,
        )]);
        let client = test_client_with_auth(mock);

        client.trader_info(None).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        assert!(reqs[0].uri.to_string().ends_with("/info/trader"));
    }

    #[tokio::test]
    async fn trader_info_with_trader_id() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"accounts":["ACC1"],"isLive":true,"traderId":"T1"}"#,
        )]);
        let client = test_client_with_auth(mock);

        client.trader_info(Some("T1")).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        let uri = reqs[0].uri.to_string();
        assert!(uri.contains("/info/trader?traderId=T1"));
    }

    // --- user_info ---

    #[tokio::test]
    async fn user_info_returns_info() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"accountTitle":"Test","emailAddress1":"a@b.com","accounts":["ACC1"]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let info = client.user_info(None).await.unwrap();

        assert_eq!(info.account_title.as_deref(), Some("Test"));
        assert_eq!(info.email_address_1.as_deref(), Some("a@b.com"));
    }

    #[tokio::test]
    async fn user_info_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"accounts":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.user_info(Some("T1")).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        let uri = reqs[0].uri.to_string();
        assert!(uri.contains("/info/user?traderId=T1"));
    }

    // --- security_definitions ---

    #[tokio::test]
    async fn security_definitions_returns_defs() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"securityDefinitions":[{"exchSym":"XCME:ES.U16","productDescription":"E-mini S&P"}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let defs = client.security_definitions(&["XCME:ES.U16"]).await.unwrap();

        assert_eq!(defs.len(), 1);
        assert_eq!(defs[0].exch_sym, "XCME:ES.U16");
        assert_eq!(defs[0].product_description.as_deref(), Some("E-mini S&P"));
    }

    #[tokio::test]
    async fn security_definitions_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"securityDefinitions":[]}"#)]);
        let client = test_client_with_auth(mock);

        client
            .security_definitions(&["XCME:ES.U16", "XCME:NQ.U16"])
            .await
            .unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        let uri = reqs[0].uri.to_string();
        assert!(uri.contains("/info/security/definitions?symbols="));
        assert!(uri.contains("XCME%3AES.U16"));
    }

    // --- security_margin ---

    #[tokio::test]
    async fn security_margin_returns_margins() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"securityMarginAndValues":[{"exchSym":"XCME:ES.U16","initialMarginLong":12000.0}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let margins = client.security_margin(&["XCME:ES.U16"]).await.unwrap();

        assert_eq!(margins.len(), 1);
        assert_eq!(margins[0].initial_margin_long, Some(12000.0));
    }

    #[tokio::test]
    async fn security_margin_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"securityMarginAndValues":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.security_margin(&["XCME:ES.U16"]).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        let uri = reqs[0].uri.to_string();
        assert!(uri.contains("/info/security/margin?symbols="));
    }

    // --- security_status ---

    #[tokio::test]
    async fn security_status_returns_statuses() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"securityStatuses":[{"exchSym":"XCME:ES.U16","statusValue":17}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let statuses = client.security_status(&["XCME:ES.U16"]).await.unwrap();

        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].status_value, Some(17));
    }

    #[tokio::test]
    async fn security_status_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"securityStatuses":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.security_status(&["XCME:ES.U16"]).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        let uri = reqs[0].uri.to_string();
        assert!(uri.contains("/info/security/status?symbols="));
    }

    // --- symbols ---

    #[tokio::test]
    async fn symbols_returns_results() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"symbols":[{"symbol":"XCME:ES.U16","description":"E-mini S&P","symbolType":"Future"}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let params = SymbolSearchParams::new().text("ES").limit(5);
        let symbols = client.symbols(&params).await.unwrap();

        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].symbol, "XCME:ES.U16");
    }

    #[tokio::test]
    async fn symbols_sends_correct_uri_with_params() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"symbols":[]}"#)]);
        let client = test_client_with_auth(mock);

        let params = SymbolSearchParams::new()
            .text("ES")
            .limit(10)
            .prefer_active(true);
        client.symbols(&params).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        let uri = reqs[0].uri.to_string();
        assert!(uri.contains("/info/symbols?"));
        assert!(uri.contains("text=ES"));
        assert!(uri.contains("limit=10"));
        assert!(uri.contains("preferActive=true"));
    }

    #[tokio::test]
    async fn symbols_sends_no_query_when_empty() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"symbols":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.symbols(&SymbolSearchParams::new()).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert!(reqs[0].uri.to_string().ends_with("/info/symbols"));
    }

    #[tokio::test]
    async fn symbols_sends_partial_query_params() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"symbols":[]}"#)]);
        let client = test_client_with_auth(mock);

        client
            .symbols(&SymbolSearchParams::new().limit(10))
            .await
            .unwrap();

        let reqs = client.request.http.recorded_requests();
        let uri = reqs[0].uri.to_string();
        assert!(uri.contains("limit=10"));
        assert!(!uri.contains("text="));
        assert!(!uri.contains("preferActive="));
    }

    // --- security validation ---

    #[tokio::test]
    async fn security_query_rejects_empty_symbols() {
        let mock = MockHttp::new(vec![]);
        let client = test_client_with_auth(mock);

        let err = client.security_definitions(&[]).await.unwrap_err();
        assert!(matches!(err, Error::Other(msg) if msg.contains("empty")));
    }

    #[tokio::test]
    async fn security_query_rejects_over_ten_symbols() {
        let mock = MockHttp::new(vec![]);
        let client = test_client_with_auth(mock);

        let syms: Vec<&str> = (0..11).map(|_| "XCME:ES.U16").collect();
        let err = client.security_definitions(&syms).await.unwrap_err();
        assert!(matches!(err, Error::Other(msg) if msg.contains("10")));
    }

    // --- exchange_sources ---

    #[tokio::test]
    async fn exchange_sources_returns_list() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"exchanges":["XCME","XCBT"]}"#)]);
        let client = test_client_with_auth(mock);

        let exchanges = client.exchange_sources().await.unwrap();

        assert_eq!(exchanges, vec!["XCME", "XCBT"]);
    }

    #[tokio::test]
    async fn exchange_sources_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"exchanges":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.exchange_sources().await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        assert!(reqs[0].uri.to_string().ends_with("/info/exchangeSources"));
    }

    // --- complexes ---

    #[tokio::test]
    async fn complexes_returns_groups() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"marketComplexes":[{"name":"Equity","groups":[{"group":"ES","name":"E-mini S&P"}]}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let complexes = client.complexes("XCME").await.unwrap();

        assert_eq!(complexes.len(), 1);
        assert_eq!(complexes[0].name.as_deref(), Some("Equity"));
        assert_eq!(complexes[0].groups.len(), 1);
    }

    #[tokio::test]
    async fn complexes_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"marketComplexes":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.complexes("XCME").await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert!(reqs[0].uri.to_string().ends_with("/info/complexes/XCME"));
    }

    // --- futures_symbols ---

    #[tokio::test]
    async fn futures_symbols_returns_futures() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"symbols":[{"symbol":"XCME:ES.U16","maturityMonth":"Sep","maturityYear":2016,"description":"E-mini S&P"}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let futures = client.futures_symbols("XCME", "ES").await.unwrap();

        assert_eq!(futures.len(), 1);
        assert_eq!(futures[0].symbol, "XCME:ES.U16");
        assert_eq!(futures[0].maturity_year, Some(2016));
    }

    #[tokio::test]
    async fn futures_symbols_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"symbols":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.futures_symbols("XCME", "ES").await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert!(
            reqs[0]
                .uri
                .to_string()
                .ends_with("/info/symbol/search/futures/XCME/ES")
        );
    }

    // --- symbol_groups ---

    #[tokio::test]
    async fn symbol_groups_returns_groups() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"symbolGroups":[{"group":"6E","name":"Euro FX"}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let groups = client.symbol_groups("Currency").await.unwrap();

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].group.as_deref(), Some("6E"));
    }

    #[tokio::test]
    async fn symbol_groups_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"symbolGroups":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.symbol_groups("Currency").await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert!(
            reqs[0]
                .uri
                .to_string()
                .ends_with("/info/symbol/search/groups/Currency")
        );
    }

    // --- option_groups ---

    #[tokio::test]
    async fn option_groups_returns_result() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"groups":["G1","G2"],"optionGroups":[{"group":"G1","description":"Group 1"}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let result = client.option_groups("XCME:ES.U16").await.unwrap();

        assert_eq!(result.groups, vec!["G1", "G2"]);
        assert_eq!(result.option_groups.len(), 1);
        assert_eq!(
            result.option_groups[0].description.as_deref(),
            Some("Group 1")
        );
    }

    #[tokio::test]
    async fn option_groups_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"groups":[],"optionGroups":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.option_groups("XCME:ES.U16").await.unwrap();

        let reqs = client.request.http.recorded_requests();
        let uri = reqs[0].uri.to_string();
        assert!(uri.contains("/info/symbol/search/options/XCME%3AES.U16"));
    }

    // --- search_options ---

    #[tokio::test]
    async fn search_options_returns_symbols() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"symbolOptions":["XCME:ES.U16.C4500","XCME:ES.U16.C4600"]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let options = client
            .search_options("XCME:ES.U16", "ES", "call", true)
            .await
            .unwrap();

        assert_eq!(options.len(), 2);
    }

    #[tokio::test]
    async fn search_options_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"symbolOptions":[]}"#)]);
        let client = test_client_with_auth(mock);

        client
            .search_options("XCME:ES.U16", "ES", "call", true)
            .await
            .unwrap();

        let reqs = client.request.http.recorded_requests();
        let uri = reqs[0].uri.to_string();
        assert!(uri.contains("/info/symbol/search/options/ext/"));
        assert!(uri.contains("/call/true"));
    }

    // --- option_spreads ---

    #[tokio::test]
    async fn option_spreads_returns_spreads() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"symbolSpreads":["+1:XCME:ES.U16.C4500:-1:XCME:ES.U16.C4600"]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let spreads = client.option_spreads("XCME:ES.U16").await.unwrap();

        assert_eq!(spreads.len(), 1);
    }

    #[tokio::test]
    async fn option_spreads_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"symbolSpreads":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.option_spreads("XCME:ES.U16").await.unwrap();

        let reqs = client.request.http.recorded_requests();
        let uri = reqs[0].uri.to_string();
        assert!(uri.contains("/info/symbol/search/options/spreads/"));
    }

    // --- strategy_id ---

    #[tokio::test]
    async fn strategy_id_returns_response() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"Id":12345,"Minimum":10000,"Maximum":20000}"#,
        )]);
        let client = test_client_with_auth(mock);

        let resp = client.strategy_id().await.unwrap();

        assert_eq!(resp.id, 12345);
        assert_eq!(resp.minimum, 10000);
        assert_eq!(resp.maximum, 20000);
    }

    #[tokio::test]
    async fn strategy_id_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"Id":1,"Minimum":0,"Maximum":100}"#,
        )]);
        let client = test_client_with_auth(mock);

        client.strategy_id().await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        assert!(reqs[0].uri.to_string().ends_with("/info/strategyId"));
    }

    // --- cross-cutting ---

    #[tokio::test]
    async fn info_sends_auth_header() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"exchanges":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.exchange_sources().await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(
            reqs[0].headers.get(AUTHORIZATION).unwrap(),
            "Bearer tok_test"
        );
    }

    #[tokio::test]
    async fn info_maps_api_error() {
        let mock = MockHttp::new(vec![MockResponse::error(
            StatusCode::NOT_FOUND,
            r#"{"error1":"Not Found"}"#,
        )]);
        let client = test_client_with_auth(mock);

        let err = client.exchange_sources().await.unwrap_err();

        match err {
            Error::Api { status, message } => {
                assert_eq!(status, 404);
                assert_eq!(message, "Not Found");
            }
            other => panic!("expected Api error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn info_maps_malformed_json() {
        let mock = MockHttp::new(vec![MockResponse::ok(b"not json".to_vec())]);
        let client = test_client_with_auth(mock);

        let err = client.exchange_sources().await.unwrap_err();
        assert!(matches!(err, Error::Json(_)));
    }
}
