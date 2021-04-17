use std::error::Error;
use super::{debug_print, Word};
use surf;
use serde_json::Value;
use futures::{
    executor::block_on,
    future::try_join_all,
};

pub fn get_definition(wordlist: &mut Vec<Word>, v: u8) -> Result<(), Box<dyn Error>> {

    let mut urilist: Vec<String> = Vec::new();

    for word in wordlist.into_iter() {
        let vocab = &word.word;
        debug_print(format!("Received Vocab: {}", vocab), 2, v);
        let uri = format!("https://dictionaryapi.com/api/v3/references/collegiate/json/{}?key=04a5d981-0869-42c8-a87c-c8cbfdcfcb56", vocab);
        urilist.push(uri);
    }

    let jsonlist = match block_on(get_json(urilist)) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            return Err("Invalid response".into())
        }
    };

    for (i, json) in jsonlist.iter().enumerate() {
        debug_print(format!("Received json:\n{}", json), 3, v);

        wordlist[i].headword = json[0]["hwi"]["hw"].to_string();
        wordlist[i].pronunciations = json[0]["hwi"]["prs"][0]["mw"].to_string();
        wordlist[i].definition = json[0]["shortdef"].to_string();
        
        debug_print(format!("{}", wordlist[i]), 1, v);
    }

    Ok(())
        
}

pub async fn get_json(urilist: Vec<String>) -> Result<Vec<Value>, Box<dyn Error>> {

    let mut futlist = Vec::new();

    for uri in urilist {
        let fut = surf::get(uri).recv_json();
        futlist.push(fut);
    }
    
    let reslist = try_join_all(futlist).await?;

    Ok(reslist)
}
