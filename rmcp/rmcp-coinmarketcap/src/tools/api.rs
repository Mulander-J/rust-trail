use rmcp::{
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, ServerHandler,
};
// use serde::Deserialize;

const BASE_URL: &str = "https://pro-api.coinmarketcap.com/v1";

/// Cryptocurrency Price Query MCP Service
///
/// This service uses the CoinMarketCap API to fetch real-time cryptocurrency price information.
/// Supports querying multiple cryptocurrencies by symbol or slug, and can convert to multiple fiat currencies.
///
/// # Examples
///
/// ```rust
/// use tools::api::CMCAPI;
///
/// let coin_price = CMCAPI::new("your-api-key");
/// ```
#[derive(Debug, Clone)]
pub struct CMCAPI {
    api_key: String,
}

#[tool(tool_box)]
impl CMCAPI {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    /// Gets the latest market quotes for cryptocurrencies
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters
    ///
    /// # Returns
    ///
    /// Returns formatted price information string
    #[tool(description = "Returns the latest market quote for 1 or more cryptocurrencies.")]
    async fn get_quotes(&self, #[tool(aggr)] params: QuoteRequest) -> String {
        if let Err(e) = params.validate() {
            return e;
        };

        let client = reqwest::Client::new();

        let response = client
            .get(format!("{}/cryptocurrency/quotes/latest", BASE_URL))
            .header("X-CMC_PRO_API_KEY", &self.api_key)
            .query(&params.get_querys())
            .send()
            .await;

        match response {
            Ok(response) => match response.json::<serde_json::Value>().await {
                Ok(data) => {
                    let mut result = String::new();
                    if let Some(data) = data["data"].as_object() {
                        for (coin_key, coin_data) in data {
                            result.push_str(&format!("{}:\n", coin_key));
                            if let Some(quotes) = coin_data["quote"].as_object() {
                                for (currency, quote_data) in quotes {
                                    if let Some(price) = quote_data["price"].as_f64() {
                                        result
                                            .push_str(&format!("  - {}: {:.2}\n", currency, price));
                                    }
                                }
                            }
                            result.push('\n');
                        }
                    }
                    if result.is_empty() {
                        "Unable to get prices for any of the specified cryptocurrencies".to_string()
                    } else {
                        result
                    }
                }
                Err(_) => "Failed to parse response data".to_string(),
            },
            Err(e) => format!("Request failed: {}", e),
        }
    }
}

#[tool(tool_box)]
impl ServerHandler for CMCAPI {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Cryptocurrency market query service".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct QuoteRequest {
    #[schemars(
        description = "Pass one or more comma-separated cryptocurrency symbols (e.g. \"BTC,ETH\"). If not using symbols, provide an empty string and use slug instead."
    )]
    pub symbol: String,

    #[schemars(
        description = "Pass one or more comma-separated cryptocurrency slugs (e.g. \"bitcoin,ethereum\"). If not using slugs, provide an empty string and use symbol instead."
    )]
    pub slug: String,

    #[schemars(
        description = "Calculate market quotes in specified currencies by passing comma-separated currency symbols (e.g. \"USD,CNY\"). Defaults to USD if empty string is provided."
    )]
    pub convert: String,
}

/// Custom deserializer for Option<String> fields
///
/// # Arguments
///
/// * `deserializer` - serde deserializer
///
/// # Returns
///
/// Returns processed Option<String>, empty strings will be converted to None
// fn deserialize_convert<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let s: Option<String> = Option::deserialize(deserializer)?;
//     Ok(s.and_then(|s| {
//         let trimmed = s.trim();
//         if trimmed.is_empty() {
//             None
//         } else {
//             Some(trimmed.to_string())
//         }
//     }))
// }

impl QuoteRequest {
    /// Validates the request parameters
    ///
    /// # Rules
    /// - Either symbol or slug must be provided (non-empty)
    /// - Cannot provide both symbol and slug (one must be empty)
    /// - Convert will default to USD if empty
    ///
    /// # Examples
    /// ```
    /// let valid_request = QuoteRequest {
    ///     symbol: "BTC".to_string(),
    ///     slug: "".to_string(),      // Empty when using symbol
    ///     convert: "USD".to_string(),
    /// };
    /// ```
    ///
    /// # Returns
    /// * `Ok(())` - Parameters are valid
    /// * `Err(String)` - Parameters are invalid, contains error message
    pub fn validate(&self) -> Result<(), String> {
        if self.symbol.trim().is_empty() && self.slug.trim().is_empty() {
            return Err("At least one of symbol or slug is required".to_string());
        }
        if !self.symbol.trim().is_empty() && !self.slug.trim().is_empty() {
            return Err("Only one of symbol or slug should be provided (use empty string for unused field)".to_string());
        }
        Ok(())
    }

    /// Gets the query parameters list
    ///
    /// # Notes
    /// - Automatically handles empty convert parameter by defaulting to USD
    /// - Only includes non-empty symbol or slug parameter
    ///
    /// # Returns
    /// Returns a list of query parameters as tuples of (key, value)
    pub fn get_querys(&self) -> Vec<(&str, String)> {
        let mut querys = vec![(
            "convert",
            if self.convert.trim().is_empty() { "USD".to_string() } else { self.convert.clone() }
        )];

        if !self.slug.trim().is_empty() {
            querys.push(("slug", self.slug.clone()));
        } else if !self.symbol.trim().is_empty() {
            querys.push(("symbol", self.symbol.clone()));
        }
        querys
    }
}
