use std::error::Error;
use super::debug_print;
use surf;
use serde_json::Value;
use futures::executor::block_on;

pub fn get_definition(vocab: &str, v: u8) -> Result<(), Box<dyn Error>> {

    debug_print(format!("Received String: {}", vocab), 2, v);

    let uri = format!("https://dictionaryapi.com/api/v3/references/collegiate/json/{}?key=04a5d981-0869-42c8-a87c-c8cbfdcfcb56", vocab);

    let res = block_on(get_json(uri));

    let res = match res {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            return Err("Invalid response".into())
        }
    };

    // clean the json
    // print the info
    println!("{}", res);

    Ok(())
}

pub async fn get_json(uri: String) -> Result<Value, Box<dyn Error>> {

    //#[derive(Deserialize, Serialize)]
    let res = match surf::get(uri).recv_json().await {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            return Err("get failed".into())
        }
    };

    Ok(res)
}
