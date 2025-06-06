use reqwest::Result;
use url::Url;
use crate::state::app_state::{App_state, Response};


pub async fn handle_get(state:&mut App_state , response :&mut  Response) -> Result<()> {
    let input_url = state.url_input.clone();

    let parsed_url = Url::parse(&input_url).unwrap(); // Proper error handling
    let response_text = reqwest::get(parsed_url).await?.text().await?;

    response.body = response_text;
    Ok(())
}

