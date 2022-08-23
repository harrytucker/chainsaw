#[tracing::instrument]
pub async fn get_item(item_name: String) -> String {
    let api_key = "4f501ed944aa45d5a63c96ea726deb858bab98fcdb6a4828901becfc5ef1e959";
    let params = [("private_key", api_key)];

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("https://xivapi.com/search?string={item_name}"))
        .form(&params)
        .send()
        .await
        .unwrap();

    resp.text().await.unwrap()
}
