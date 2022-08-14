use std::collections::HashMap;


fn main(){
    handshake();
}
#[tokio::main]


async fn handshake() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://127.0.0.1:5000/")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(resp)
}