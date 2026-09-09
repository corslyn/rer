use reqwest::header::{ACCEPT, HeaderMap, HeaderValue};

/// recupere les infos des RER depuis la gare, avec l' "identifiant du référentiel des arrêts​"
pub fn request(gare: &str, api_key: &str) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://prim.iledefrance-mobilites.fr/marketplace/stop-monitoring?MonitoringRef=STIF:StopPoint:Q:{}:",
        gare
    );
    let headers = construct_headers(api_key);
    let response = reqwest::blocking::Client::new()
        .get(&url)
        .headers(headers)
        .send()?
        .text()?;
    Ok(response)
}

/// ajoute les headers dont la cle api
fn construct_headers(api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(
        "apiKey",
        HeaderValue::from_bytes(api_key.as_bytes())
            .expect("invalid API key header value"),
    );
    headers
}
