use time::Date;
use time::macros::format_description;

use crate::client::Client;
use crate::client::http::HttpTransport;
use crate::error::Result;
use crate::types::{
    Response, SimulatedAccountAddCashRequest, SimulatedAccountCashReportResponse,
    SimulatedAccountExpireRequest, SimulatedAccountLiquidateRequest, SimulatedAccountResetRequest,
    SimulatedAccountSetRiskRequest, SimulatedTraderAddAccountRequest,
    SimulatedTraderAddAccountResponse, SimulatedTraderCreateRequest, SimulatedTraderCreateResponse,
};

// ---------------------------------------------------------------------------
// LiquidateBuilder — ergonomic liquidation request
// ---------------------------------------------------------------------------

/// Builder for simulated account liquidation requests.
///
/// # Example
///
/// ```
/// use ironbeam_rs::client::LiquidateBuilder;
///
/// let req = LiquidateBuilder::new()
///     .accounts(&["ACC001", "ACC002"])
///     .force_manual(true);
/// ```
#[derive(Debug, Clone, Default)]
pub struct LiquidateBuilder {
    accounts: Option<Vec<String>>,
    groups: Option<Vec<String>>,
    except_accounts: Option<Vec<String>>,
    force_manual_liquidation: Option<bool>,
    use_manual_liquidation_for_illiquid_markets: Option<bool>,
    send_account_email: Option<bool>,
    send_office_email: Option<bool>,
}

impl LiquidateBuilder {
    /// Create an empty liquidation request.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Accounts to liquidate.
    #[must_use]
    pub fn accounts(mut self, accounts: &[&str]) -> Self {
        self.accounts = Some(accounts.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Groups to liquidate.
    #[must_use]
    pub fn groups(mut self, groups: &[&str]) -> Self {
        self.groups = Some(groups.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Accounts to exclude from liquidation.
    #[must_use]
    pub fn except_accounts(mut self, accounts: &[&str]) -> Self {
        self.except_accounts = Some(accounts.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Force manual liquidation.
    #[must_use]
    pub fn force_manual(mut self, force: bool) -> Self {
        self.force_manual_liquidation = Some(force);
        self
    }

    /// Use manual liquidation for illiquid markets.
    #[must_use]
    pub fn manual_for_illiquid(mut self, manual: bool) -> Self {
        self.use_manual_liquidation_for_illiquid_markets = Some(manual);
        self
    }

    /// Send email to account holder.
    #[must_use]
    pub fn send_account_email(mut self, send: bool) -> Self {
        self.send_account_email = Some(send);
        self
    }

    /// Send email to office.
    #[must_use]
    pub fn send_office_email(mut self, send: bool) -> Self {
        self.send_office_email = Some(send);
        self
    }

    fn to_request(&self) -> SimulatedAccountLiquidateRequest {
        SimulatedAccountLiquidateRequest {
            accounts: self.accounts.clone(),
            groups: self.groups.clone(),
            except_accounts: self.except_accounts.clone(),
            force_manual_liquidation: self.force_manual_liquidation,
            use_manual_liquidation_for_illiquid_markets: self
                .use_manual_liquidation_for_illiquid_markets,
            send_account_email: self.send_account_email,
            send_office_email: self.send_office_email,
        }
    }
}

// ---------------------------------------------------------------------------
// RiskBuilder — ergonomic risk parameter setup
// ---------------------------------------------------------------------------

/// Builder for simulated account risk parameters.
///
/// # Example
///
/// ```
/// use ironbeam_rs::client::RiskBuilder;
///
/// let risk = RiskBuilder::new("ACC001")
///     .liquidation_account_value(25_000.0)
///     .reduce_positions_only(true);
/// ```
#[derive(Debug, Clone)]
pub struct RiskBuilder {
    account_id: String,
    liquidation_account_value: Option<f64>,
    liquidation_loss_from_start_of_day: Option<f64>,
    liquidation_loss_from_high_of_day: Option<f64>,
    liquidation_loss_from_high_of_multiday: Option<f64>,
    liquidation_pct_loss_from_start_of_day: Option<f64>,
    liquidation_pct_loss_from_high_of_day: Option<f64>,
    liquidation_pct_loss_from_high_of_multiday: Option<f64>,
    liquidation_pct_margin_deficiency: Option<f64>,
    liquidation_max_value_override: Option<f64>,
    reduce_positions_only: Option<bool>,
    restore_trading: Option<bool>,
    margin_schedule_name: Option<String>,
    template_id: Option<String>,
}

impl RiskBuilder {
    /// Create a risk builder for the given account.
    #[must_use]
    pub fn new(account_id: impl Into<String>) -> Self {
        Self {
            account_id: account_id.into(),
            liquidation_account_value: None,
            liquidation_loss_from_start_of_day: None,
            liquidation_loss_from_high_of_day: None,
            liquidation_loss_from_high_of_multiday: None,
            liquidation_pct_loss_from_start_of_day: None,
            liquidation_pct_loss_from_high_of_day: None,
            liquidation_pct_loss_from_high_of_multiday: None,
            liquidation_pct_margin_deficiency: None,
            liquidation_max_value_override: None,
            reduce_positions_only: None,
            restore_trading: None,
            margin_schedule_name: None,
            template_id: None,
        }
    }

    /// Liquidation account value threshold.
    #[must_use]
    pub fn liquidation_account_value(mut self, value: f64) -> Self {
        self.liquidation_account_value = Some(value);
        self
    }

    /// Liquidation loss from start of day.
    #[must_use]
    pub fn liquidation_loss_from_start_of_day(mut self, value: f64) -> Self {
        self.liquidation_loss_from_start_of_day = Some(value);
        self
    }

    /// Liquidation loss from high of day.
    #[must_use]
    pub fn liquidation_loss_from_high_of_day(mut self, value: f64) -> Self {
        self.liquidation_loss_from_high_of_day = Some(value);
        self
    }

    /// Liquidation loss from high of multiday.
    #[must_use]
    pub fn liquidation_loss_from_high_of_multiday(mut self, value: f64) -> Self {
        self.liquidation_loss_from_high_of_multiday = Some(value);
        self
    }

    /// Liquidation percentage loss from start of day.
    #[must_use]
    pub fn liquidation_pct_loss_from_start_of_day(mut self, value: f64) -> Self {
        self.liquidation_pct_loss_from_start_of_day = Some(value);
        self
    }

    /// Liquidation percentage loss from high of day.
    #[must_use]
    pub fn liquidation_pct_loss_from_high_of_day(mut self, value: f64) -> Self {
        self.liquidation_pct_loss_from_high_of_day = Some(value);
        self
    }

    /// Liquidation percentage loss from high of multiday.
    #[must_use]
    pub fn liquidation_pct_loss_from_high_of_multiday(mut self, value: f64) -> Self {
        self.liquidation_pct_loss_from_high_of_multiday = Some(value);
        self
    }

    /// Liquidation percentage margin deficiency.
    #[must_use]
    pub fn liquidation_pct_margin_deficiency(mut self, value: f64) -> Self {
        self.liquidation_pct_margin_deficiency = Some(value);
        self
    }

    /// Override maximum account value limit.
    #[must_use]
    pub fn liquidation_max_value_override(mut self, value: f64) -> Self {
        self.liquidation_max_value_override = Some(value);
        self
    }

    /// Only reduce positions, don't flatten.
    #[must_use]
    pub fn reduce_positions_only(mut self, reduce: bool) -> Self {
        self.reduce_positions_only = Some(reduce);
        self
    }

    /// Restore trading after liquidation.
    #[must_use]
    pub fn restore_trading(mut self, restore: bool) -> Self {
        self.restore_trading = Some(restore);
        self
    }

    /// Margin schedule name.
    #[must_use]
    pub fn margin_schedule_name(mut self, name: impl Into<String>) -> Self {
        self.margin_schedule_name = Some(name.into());
        self
    }

    /// Template ID.
    #[must_use]
    pub fn template_id(mut self, id: impl Into<String>) -> Self {
        self.template_id = Some(id.into());
        self
    }

    fn to_request(&self) -> SimulatedAccountSetRiskRequest {
        SimulatedAccountSetRiskRequest {
            account_id: self.account_id.clone(),
            liquidation_account_value: self.liquidation_account_value,
            liquidation_loss_from_start_of_day: self.liquidation_loss_from_start_of_day,
            liquidation_loss_from_high_of_day: self.liquidation_loss_from_high_of_day,
            liquidation_loss_from_high_of_multiday: self.liquidation_loss_from_high_of_multiday,
            liquidation_pct_loss_from_start_of_day: self.liquidation_pct_loss_from_start_of_day,
            liquidation_pct_loss_from_high_of_day: self.liquidation_pct_loss_from_high_of_day,
            liquidation_pct_loss_from_high_of_multiday: self
                .liquidation_pct_loss_from_high_of_multiday,
            liquidation_pct_margin_deficiency: self.liquidation_pct_margin_deficiency,
            liquidation_max_value_override: self.liquidation_max_value_override,
            reduce_positions_only: self.reduce_positions_only,
            restore_trading: self.restore_trading,
            margin_schedule_name: self.margin_schedule_name.clone(),
            template_id: self.template_id.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Client methods
// ---------------------------------------------------------------------------

impl<H: HttpTransport> Client<H> {
    /// Create a simulated trader (demo only, enterprise feature).
    ///
    /// Returns the new trader ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # use ironbeam_rs::types::SimulatedTraderCreateRequest;
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .demo().connect().await?;
    /// let trader_id = client.simulated_trader_create(&SimulatedTraderCreateRequest {
    ///     first_name: "John".into(),
    ///     last_name: "Doe".into(),
    ///     address1: "123 Main St".into(),
    ///     address2: None,
    ///     city: "Chicago".into(),
    ///     state: "IL".into(),
    ///     country: "US".into(),
    ///     zip_code: "60601".into(),
    ///     phone: "555-0100".into(),
    ///     email: "john@example.com".into(),
    ///     password: "secret".into(),
    ///     template_id: "XAP100".into(),
    /// }).await?;
    /// println!("Trader ID: {trader_id}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn simulated_trader_create(
        &self,
        request: &SimulatedTraderCreateRequest,
    ) -> Result<String> {
        let resp: SimulatedTraderCreateResponse =
            self.post("/simulatedTraderCreate", request).await?;
        Ok(resp.trader_id)
    }

    /// Add an account to an existing simulated trader (demo only).
    ///
    /// Returns the new account ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .demo().connect().await?;
    /// let account_id = client.simulated_account_add("T123", "secret", "XAP50").await?;
    /// println!("Account ID: {account_id}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn simulated_account_add(
        &self,
        trader_id: &str,
        password: &str,
        template_id: &str,
    ) -> Result<String> {
        let req = SimulatedTraderAddAccountRequest {
            trader_id: trader_id.to_string(),
            password: password.to_string(),
            template_id: template_id.to_string(),
        };
        let resp: SimulatedTraderAddAccountResponse =
            self.post("/simulatedAccountAdd", &req).await?;
        Ok(resp.account_id)
    }

    /// Reset a simulated account to its initial state (demo only).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .demo().connect().await?;
    /// client.simulated_account_reset("ACC001", "XAP100").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn simulated_account_reset(
        &self,
        account_id: &str,
        template_id: &str,
    ) -> Result<Response> {
        let req = SimulatedAccountResetRequest {
            account_id: account_id.to_string(),
            template_id: template_id.to_string(),
        };
        self.put("/simulatedAccountReset", &req).await
    }

    /// Expire a simulated account (demo only).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .demo().connect().await?;
    /// client.simulated_account_expire("ACC001").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn simulated_account_expire(&self, account_id: &str) -> Result<Response> {
        let req = SimulatedAccountExpireRequest {
            account_id: account_id.to_string(),
        };
        self.delete_with_body("/simulatedAccountExpire", &req).await
    }

    /// Add cash to a simulated account (demo only).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .demo().connect().await?;
    /// client.simulated_account_add_cash("ACC001", 10_000.0).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn simulated_account_add_cash(
        &self,
        account_id: &str,
        amount: f32,
    ) -> Result<Response> {
        let req = SimulatedAccountAddCashRequest {
            account_id: account_id.to_string(),
            amount,
        };
        self.post("/simulatedAccount/addCash", &req).await
    }

    /// Get the cash report for a simulated account (demo only).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .demo().connect().await?;
    /// let start = time::Date::from_calendar_date(2025, time::Month::January, 1)?;
    /// let end = time::Date::from_calendar_date(2025, time::Month::December, 31)?;
    /// let report = client.simulated_account_cash_report("ACC001", start, end).await?;
    /// for entry in &report.cash_report {
    ///     println!("{entry:?}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn simulated_account_cash_report(
        &self,
        account_id: &str,
        start_date: Date,
        end_date: Date,
    ) -> Result<SimulatedAccountCashReportResponse> {
        let fmt = format_description!("[year][month][day]");
        let start = start_date
            .format(fmt)
            .map_err(|e| crate::error::Error::Other(e.to_string()))?;
        let end = end_date
            .format(fmt)
            .map_err(|e| crate::error::Error::Other(e.to_string()))?;
        let account_id = urlencoding::encode(account_id);
        let path =
            format!("/simulatedAccount/getCashReport/{account_id}?startDate={start}&endDate={end}");
        self.get(&path).await
    }

    /// Liquidate simulated accounts (demo only).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials, LiquidateBuilder};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .demo().connect().await?;
    /// let req = LiquidateBuilder::new()
    ///     .accounts(&["ACC001"])
    ///     .force_manual(true);
    /// client.simulated_account_liquidate(&req).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn simulated_account_liquidate(
        &self,
        builder: &LiquidateBuilder,
    ) -> Result<Response> {
        let req = builder.to_request();
        self.post("/simulatedAccount/liquidate", &req).await
    }

    /// Set risk parameters for a simulated account (demo only).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials, RiskBuilder};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .demo().connect().await?;
    /// let risk = RiskBuilder::new("ACC001")
    ///     .liquidation_account_value(25_000.0);
    /// client.simulated_account_set_risk(&risk).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn simulated_account_set_risk(&self, builder: &RiskBuilder) -> Result<Response> {
        let req = builder.to_request();
        self.post("/simulatedAccount/setRisk", &req).await
    }
}

#[cfg(test)]
mod tests {
    use hyper::header::AUTHORIZATION;
    use hyper::{Method, StatusCode};
    use time::Month;

    use crate::client::http::mock::{MockHttp, MockResponse};
    use crate::client::test_support::test_client_with_auth;
    use crate::error::Error;
    use crate::types::*;

    use super::{LiquidateBuilder, RiskBuilder};

    fn dummy_trader_create() -> SimulatedTraderCreateRequest {
        SimulatedTraderCreateRequest {
            first_name: "J".into(),
            last_name: "D".into(),
            address1: "x".into(),
            address2: None,
            city: "x".into(),
            state: "x".into(),
            country: "x".into(),
            zip_code: "x".into(),
            phone: "x".into(),
            email: "x".into(),
            password: "x".into(),
            template_id: "XAP100".into(),
        }
    }

    // --- simulated_trader_create ---

    #[tokio::test]
    async fn simulated_trader_create_returns_id() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"TraderId":"T001"}"#)]);
        let client = test_client_with_auth(mock);

        let req = dummy_trader_create();

        let trader_id = client.simulated_trader_create(&req).await.unwrap();
        assert_eq!(trader_id, "T001");

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::POST);
        assert!(reqs[0].uri.to_string().ends_with("/simulatedTraderCreate"));
        assert_eq!(
            reqs[0].headers.get(AUTHORIZATION).unwrap(),
            "Bearer tok_test"
        );

        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["FirstName"], "J");
        assert_eq!(body["TemplateId"], "XAP100");
    }

    #[tokio::test]
    async fn simulated_trader_create_api_error() {
        let mock = MockHttp::new(vec![MockResponse::error(
            StatusCode::BAD_REQUEST,
            r#"{"error1":"Invalid template"}"#,
        )]);
        let client = test_client_with_auth(mock);

        let err = client
            .simulated_trader_create(&dummy_trader_create())
            .await
            .unwrap_err();
        match err {
            Error::Api { status, message } => {
                assert_eq!(status, 400);
                assert_eq!(message, "Invalid template");
            }
            other => panic!("expected Api error, got {other:?}"),
        }
    }

    // --- simulated_account_add ---

    #[tokio::test]
    async fn simulated_account_add_returns_id() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"AccountId":"ACC001"}"#)]);
        let client = test_client_with_auth(mock);

        let account_id = client
            .simulated_account_add("T001", "secret", "XAP50")
            .await
            .unwrap();
        assert_eq!(account_id, "ACC001");

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::POST);
        assert!(reqs[0].uri.to_string().ends_with("/simulatedAccountAdd"));

        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["TraderId"], "T001");
        assert_eq!(body["Password"], "secret");
        assert_eq!(body["TemplateId"], "XAP50");
    }

    // --- simulated_account_reset ---

    #[tokio::test]
    async fn simulated_account_reset_uses_put() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        let client = test_client_with_auth(mock);

        let resp = client
            .simulated_account_reset("ACC001", "XAP100")
            .await
            .unwrap();
        assert_eq!(resp.status, ResponseStatus::Ok);

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::PUT);
        assert!(reqs[0].uri.to_string().ends_with("/simulatedAccountReset"));

        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["AccountId"], "ACC001");
        assert_eq!(body["TemplateId"], "XAP100");
    }

    // --- simulated_account_expire ---

    #[tokio::test]
    async fn simulated_account_expire_uses_delete_with_body() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        let client = test_client_with_auth(mock);

        let resp = client.simulated_account_expire("ACC001").await.unwrap();
        assert_eq!(resp.status, ResponseStatus::Ok);

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::DELETE);
        assert!(reqs[0].uri.to_string().ends_with("/simulatedAccountExpire"));

        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["AccountId"], "ACC001");
    }

    // --- simulated_account_add_cash ---

    #[tokio::test]
    async fn simulated_account_add_cash_sends_amount() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        let client = test_client_with_auth(mock);

        client
            .simulated_account_add_cash("ACC001", 10_000.0)
            .await
            .unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::POST);
        assert!(
            reqs[0]
                .uri
                .to_string()
                .ends_with("/simulatedAccount/addCash")
        );

        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["AccountId"], "ACC001");
        assert_eq!(body["Amount"], 10_000.0);
    }

    // --- simulated_account_cash_report ---

    #[tokio::test]
    async fn simulated_account_cash_report_formats_dates() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"AccountId":"ACC001","CashReport":[{"amount":5000.0}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let start = time::Date::from_calendar_date(2025, Month::January, 1).unwrap();
        let end = time::Date::from_calendar_date(2025, Month::December, 31).unwrap();

        let resp = client
            .simulated_account_cash_report("ACC001", start, end)
            .await
            .unwrap();

        assert_eq!(resp.account_id.as_deref(), Some("ACC001"));
        assert_eq!(resp.cash_report.len(), 1);

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        let uri = reqs[0].uri.to_string();
        assert!(uri.contains("/simulatedAccount/getCashReport/ACC001"));
        assert!(uri.contains("startDate=20250101"));
        assert!(uri.contains("endDate=20251231"));
    }

    // --- simulated_account_liquidate ---

    #[tokio::test]
    async fn simulated_account_liquidate_sends_accounts() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        let client = test_client_with_auth(mock);

        let req = LiquidateBuilder::new()
            .accounts(&["ACC001", "ACC002"])
            .force_manual(true);

        client.simulated_account_liquidate(&req).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::POST);
        assert!(
            reqs[0]
                .uri
                .to_string()
                .ends_with("/simulatedAccount/liquidate")
        );

        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["Accounts"], serde_json::json!(["ACC001", "ACC002"]));
        assert_eq!(body["ForceManualLiquidation"], true);
        assert!(body.get("Groups").is_none());
    }

    // --- simulated_account_set_risk ---

    #[tokio::test]
    async fn simulated_account_set_risk_sends_params() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        let client = test_client_with_auth(mock);

        let risk = RiskBuilder::new("ACC001").liquidation_account_value(25_000.0);

        client.simulated_account_set_risk(&risk).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::POST);
        assert!(
            reqs[0]
                .uri
                .to_string()
                .ends_with("/simulatedAccount/setRisk")
        );

        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["AccountId"], "ACC001");
        assert_eq!(body["LiquidationAccountValue"], 25_000.0);
        assert!(body.get("ReducePositionsOnly").is_none());
    }

    // --- auth header forwarding ---

    #[tokio::test]
    async fn simulation_endpoints_forward_auth_header() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"TraderId":"T1"}"#)]);
        let client = test_client_with_auth(mock);

        client
            .simulated_trader_create(&dummy_trader_create())
            .await
            .unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(
            reqs[0].headers.get(AUTHORIZATION).unwrap(),
            "Bearer tok_test"
        );
    }

    // --- malformed JSON ---

    #[tokio::test]
    async fn simulation_malformed_json_returns_json_error() {
        let mock = MockHttp::new(vec![MockResponse::ok(b"not json".to_vec())]);
        let client = test_client_with_auth(mock);

        let err = client
            .simulated_account_add_cash("ACC001", 100.0)
            .await
            .unwrap_err();
        assert!(matches!(err, Error::Json(_)));
    }
}
