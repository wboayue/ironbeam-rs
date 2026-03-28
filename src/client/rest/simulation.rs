use time::Date;
use time::macros::format_description;

use crate::client::Client;
use crate::client::http::HttpTransport;
use crate::error::Result;
use crate::types::{
    Response, SimulatedAccountAddCashRequest, SimulatedAccountCashReportResponse,
    SimulatedAccountExpireRequest, SimulatedAccountLiquidateRequest, SimulatedAccountResetRequest,
    SimulatedAccountSetRiskRequest, SimulatedTraderAddAccountRequest,
    SimulatedTraderAddAccountResponse, SimulatedTraderCreateRequest,
    SimulatedTraderCreateResponse,
};

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
    /// # use ironbeam_rs::types::SimulatedTraderAddAccountRequest;
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .demo().connect().await?;
    /// let account_id = client.simulated_account_add(&SimulatedTraderAddAccountRequest {
    ///     trader_id: "T123".into(),
    ///     password: "secret".into(),
    ///     template_id: "XAP50".into(),
    /// }).await?;
    /// println!("Account ID: {account_id}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn simulated_account_add(
        &self,
        request: &SimulatedTraderAddAccountRequest,
    ) -> Result<String> {
        let resp: SimulatedTraderAddAccountResponse =
            self.post("/simulatedAccountAdd", request).await?;
        Ok(resp.account_id)
    }

    /// Reset a simulated account to its initial state (demo only).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # use ironbeam_rs::types::SimulatedAccountResetRequest;
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .demo().connect().await?;
    /// client.simulated_account_reset(&SimulatedAccountResetRequest {
    ///     account_id: "ACC001".into(),
    ///     template_id: "XAP100".into(),
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn simulated_account_reset(
        &self,
        request: &SimulatedAccountResetRequest,
    ) -> Result<Response> {
        self.put("/simulatedAccountReset", request).await
    }

    /// Expire a simulated account (demo only).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # use ironbeam_rs::types::SimulatedAccountExpireRequest;
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .demo().connect().await?;
    /// client.simulated_account_expire(&SimulatedAccountExpireRequest {
    ///     account_id: "ACC001".into(),
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn simulated_account_expire(
        &self,
        request: &SimulatedAccountExpireRequest,
    ) -> Result<Response> {
        self.delete_with_body("/simulatedAccountExpire", request)
            .await
    }

    /// Add cash to a simulated account (demo only).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # use ironbeam_rs::types::SimulatedAccountAddCashRequest;
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .demo().connect().await?;
    /// client.simulated_account_add_cash(&SimulatedAccountAddCashRequest {
    ///     account_id: "ACC001".into(),
    ///     amount: 10_000.0,
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn simulated_account_add_cash(
        &self,
        request: &SimulatedAccountAddCashRequest,
    ) -> Result<Response> {
        self.post("/simulatedAccount/addCash", request).await
    }

    /// Get the cash report for a simulated account (demo only).
    ///
    /// Dates are formatted as YYYYMMDD integers in the query string.
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
        let start = start_date.format(fmt).map_err(|e| crate::error::Error::Other(e.to_string()))?;
        let end = end_date.format(fmt).map_err(|e| crate::error::Error::Other(e.to_string()))?;
        let path = format!(
            "/simulatedAccount/getCashReport/{account_id}?startDate={start}&endDate={end}"
        );
        self.get(&path).await
    }

    /// Liquidate simulated accounts (demo only).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # use ironbeam_rs::types::SimulatedAccountLiquidateRequest;
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .demo().connect().await?;
    /// client.simulated_account_liquidate(&SimulatedAccountLiquidateRequest {
    ///     accounts: Some(vec!["ACC001".into()]),
    ///     groups: None,
    ///     except_accounts: None,
    ///     force_manual_liquidation: None,
    ///     use_manual_liquidation_for_illiquid_markets: None,
    ///     send_account_email: None,
    ///     send_office_email: None,
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn simulated_account_liquidate(
        &self,
        request: &SimulatedAccountLiquidateRequest,
    ) -> Result<Response> {
        self.post("/simulatedAccount/liquidate", request).await
    }

    /// Set risk parameters for a simulated account (demo only).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # use ironbeam_rs::types::SimulatedAccountSetRiskRequest;
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .demo().connect().await?;
    /// client.simulated_account_set_risk(&SimulatedAccountSetRiskRequest {
    ///     account_id: "ACC001".into(),
    ///     liquidation_account_value: Some(25_000.0),
    ///     liquidation_loss_from_start_of_day: None,
    ///     liquidation_loss_from_high_of_day: None,
    ///     liquidation_loss_from_high_of_multiday: None,
    ///     liquidation_pct_loss_from_start_of_day: None,
    ///     liquidation_pct_loss_from_high_of_day: None,
    ///     liquidation_pct_loss_from_high_of_multiday: None,
    ///     liquidation_pct_margin_deficiency: None,
    ///     liquidation_max_value_override: None,
    ///     reduce_positions_only: None,
    ///     restore_trading: None,
    ///     margin_schedule_name: None,
    ///     template_id: None,
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn simulated_account_set_risk(
        &self,
        request: &SimulatedAccountSetRiskRequest,
    ) -> Result<Response> {
        self.post("/simulatedAccount/setRisk", request).await
    }
}

#[cfg(test)]
mod tests {
    use hyper::{Method, StatusCode};
    use hyper::header::AUTHORIZATION;
    use time::Month;

    use crate::client::http::mock::{MockHttp, MockResponse};
    use crate::client::test_support::test_client_with_auth;
    use crate::error::Error;
    use crate::types::*;

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
        assert_eq!(reqs[0].headers.get(AUTHORIZATION).unwrap(), "Bearer tok_test");

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

        let err = client.simulated_trader_create(&dummy_trader_create()).await.unwrap_err();
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

        let req = SimulatedTraderAddAccountRequest {
            trader_id: "T001".into(),
            password: "secret".into(),
            template_id: "XAP50".into(),
        };

        let account_id = client.simulated_account_add(&req).await.unwrap();
        assert_eq!(account_id, "ACC001");

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::POST);
        assert!(reqs[0].uri.to_string().ends_with("/simulatedAccountAdd"));
    }

    // --- simulated_account_reset ---

    #[tokio::test]
    async fn simulated_account_reset_uses_put() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        let client = test_client_with_auth(mock);

        let req = SimulatedAccountResetRequest {
            account_id: "ACC001".into(),
            template_id: "XAP100".into(),
        };

        let resp = client.simulated_account_reset(&req).await.unwrap();
        assert_eq!(resp.status, ResponseStatus::Ok);

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::PUT);
        assert!(reqs[0].uri.to_string().ends_with("/simulatedAccountReset"));

        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["AccountId"], "ACC001");
    }

    // --- simulated_account_expire ---

    #[tokio::test]
    async fn simulated_account_expire_uses_delete_with_body() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        let client = test_client_with_auth(mock);

        let req = SimulatedAccountExpireRequest {
            account_id: "ACC001".into(),
        };

        let resp = client.simulated_account_expire(&req).await.unwrap();
        assert_eq!(resp.status, ResponseStatus::Ok);

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::DELETE);
        assert!(reqs[0]
            .uri
            .to_string()
            .ends_with("/simulatedAccountExpire"));

        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["AccountId"], "ACC001");
    }

    // --- simulated_account_add_cash ---

    #[tokio::test]
    async fn simulated_account_add_cash_sends_amount() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        let client = test_client_with_auth(mock);

        let req = SimulatedAccountAddCashRequest {
            account_id: "ACC001".into(),
            amount: 10_000.0,
        };

        client.simulated_account_add_cash(&req).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::POST);
        assert!(reqs[0]
            .uri
            .to_string()
            .ends_with("/simulatedAccount/addCash"));

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

        let req = SimulatedAccountLiquidateRequest {
            accounts: Some(vec!["ACC001".into(), "ACC002".into()]),
            groups: None,
            except_accounts: None,
            force_manual_liquidation: Some(true),
            use_manual_liquidation_for_illiquid_markets: None,
            send_account_email: None,
            send_office_email: None,
        };

        client.simulated_account_liquidate(&req).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::POST);
        assert!(reqs[0]
            .uri
            .to_string()
            .ends_with("/simulatedAccount/liquidate"));

        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["Accounts"], serde_json::json!(["ACC001", "ACC002"]));
        assert_eq!(body["ForceManualLiquidation"], true);
    }

    // --- simulated_account_set_risk ---

    #[tokio::test]
    async fn simulated_account_set_risk_sends_params() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        let client = test_client_with_auth(mock);

        let req = SimulatedAccountSetRiskRequest {
            account_id: "ACC001".into(),
            liquidation_account_value: Some(25_000.0),
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
        };

        client.simulated_account_set_risk(&req).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::POST);
        assert!(reqs[0]
            .uri
            .to_string()
            .ends_with("/simulatedAccount/setRisk"));

        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["AccountId"], "ACC001");
        assert_eq!(body["LiquidationAccountValue"], 25_000.0);
        // Optional None fields should be omitted
        assert!(body.get("ReducePositionsOnly").is_none());
    }

    // --- auth header forwarding ---

    #[tokio::test]
    async fn simulation_endpoints_forward_auth_header() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"TraderId":"T1"}"#)]);
        let client = test_client_with_auth(mock);

        client.simulated_trader_create(&dummy_trader_create()).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].headers.get(AUTHORIZATION).unwrap(), "Bearer tok_test");
    }

    // --- malformed JSON ---

    #[tokio::test]
    async fn simulation_malformed_json_returns_json_error() {
        let mock = MockHttp::new(vec![MockResponse::ok(b"not json".to_vec())]);
        let client = test_client_with_auth(mock);

        let req = SimulatedAccountAddCashRequest {
            account_id: "ACC001".into(),
            amount: 100.0,
        };

        let err = client.simulated_account_add_cash(&req).await.unwrap_err();
        assert!(matches!(err, Error::Json(_)));
    }
}
