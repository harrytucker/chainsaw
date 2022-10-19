use axum::{
    extract::Path,
    headers::HeaderMap,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[tracing::instrument]
pub async fn get_item(item_name: Path<String>, headers: HeaderMap) -> Result<String, Response> {
    let api_key = "4f501ed944aa45d5a63c96ea726deb858bab98fcdb6a4828901becfc5ef1e959";
    let params = [
        ("private_key", api_key),
        ("string", &item_name),
        ("indexes", "item"),
    ];

    let client = reqwest::Client::new();
    let resp_text = match client
        .get("https://xivapi.com/search")
        .headers(extract_trace_headers(&headers))
        .query(&params)
        .send()
        .await
    {
        Ok(resp) => resp.text(),
        Err(err) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())
        }
    };

    match resp_text.await {
        Ok(text) => Ok(text),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()),
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct MarketBoardSaleHistory {
    #[serde(rename = "itemID")]
    pub item_id: i64,
    pub last_upload_time: i64, // timestamp
    pub entries: Vec<MarketBoardEntry>,
    pub dc_name: String,
    // ignoring stack size histograms
    pub regular_sale_velocity: f32,
    pub nq_sale_velocity: f32,
    pub hq_sale_velocity: f32,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct MarketBoardEntry {
    pub hq: bool,
    pub price_per_unit: i64,
    pub quantity: i64,
    pub buyer_name: String,
    pub on_mannequin: bool,
    pub timestamp: i64, // timestamp
    pub world_name: String,
    #[serde(rename = "worldID")]
    pub world_id: i64,
}

#[cfg(test)]
mod test {
    use chainsaw::Result;
    use pretty_assertions::assert_eq;

    use crate::xiv::{MarketBoardEntry, MarketBoardSaleHistory};

    #[test]
    fn test_deserialize() -> Result<()> {
        let input = r#"
            {
              "itemID": 37742,
              "lastUploadTime": 1661300472533,
              "entries": [
                {
                  "hq": true,
                  "pricePerUnit": 1485500,
                  "quantity": 1,
                  "buyerName": "Badmoon Rising",
                  "onMannequin": false,
                  "timestamp": 1661298306,
                  "worldName": "Spriggan",
                  "worldID": 85
                },
                {
                  "hq": true,
                  "pricePerUnit": 1485500,
                  "quantity": 1,
                  "buyerName": "Pelinal Whitestrake",
                  "onMannequin": false,
                  "timestamp": 1661293236,
                  "worldName": "Spriggan",
                  "worldID": 85
                }
              ],
              "dcName": "Chaos",
              "stackSizeHistogram": {
                "1": 2
              },
              "stackSizeHistogramNQ": {},
              "stackSizeHistogramHQ": {
                "1": 2
              },
              "regularSaleVelocity": 0.2857143,
              "nqSaleVelocity": 0,
              "hqSaleVelocity": 0.2857143
            }
        "#;

        let expected_entry_one = MarketBoardEntry {
            hq: true,
            price_per_unit: 1485500,
            quantity: 1,
            buyer_name: "Badmoon Rising".to_string(),
            on_mannequin: false,
            timestamp: 1661298306,
            world_name: "Spriggan".to_string(),
            world_id: 85,
        };

        let expected_entry_two = MarketBoardEntry {
            hq: true,
            price_per_unit: 1485500,
            quantity: 1,
            buyer_name: "Pelinal Whitestrake".to_string(),
            on_mannequin: false,
            timestamp: 1661293236,
            world_name: "Spriggan".to_string(),
            world_id: 85,
        };

        let expected_market_board_history = MarketBoardSaleHistory {
            item_id: 37742,
            last_upload_time: 1661300472533,
            entries: vec![expected_entry_one, expected_entry_two],
            dc_name: "Chaos".to_string(),
            regular_sale_velocity: 0.2857143,
            nq_sale_velocity: 0.0,
            hq_sale_velocity: 0.2857143,
        };

        let parsed: MarketBoardSaleHistory = serde_json::from_str(input)?;
        assert_eq!(expected_market_board_history, parsed);

        Ok(())
    }
}

fn extract_trace_headers(input_headers: &HeaderMap) -> HeaderMap {
    const TRACE_HEADERS: [&str; 5] = [
        "x-request-id",
        "x-b3-traceid",
        "x-b3-spanid",
        "x-b3-parentspanid",
        "x-b3-sampled",
    ];

    let mut headers = HeaderMap::new();
    TRACE_HEADERS.into_iter().for_each(|key| {
        if let Some(value) = input_headers.get(key) {
            headers.insert(key, value.into());
        }
    });

    headers
}
