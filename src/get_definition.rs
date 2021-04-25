use std::error::Error;
use super::{debug_print, Word};
use surf;
use serde_json::Value;
use futures::{
    executor::block_on,
    future::try_join_all,
};

static CHUNK_SIZE: usize = 100;

pub fn get_definition(wordlist: &mut Vec<Word>, v: u8) -> Result<(), Box<dyn Error>> {

    let mut jsonlist = Vec::new();
    let mut chunk_cnt = 0;

    // Sending too many requests seem to cause dropped connections. Send it batches of CHUNK_SIZE.
    for chunk in wordlist.chunks_mut(CHUNK_SIZE) {
        let mut urilist: Vec<String> = Vec::new();
        for (i, word) in chunk.iter_mut().enumerate() {
            let vocab = &word.word;
            debug_print(format!("Received {} Vocab: {}", chunk_cnt*CHUNK_SIZE+i+1, vocab), 1, v);
            let uri = format!("https://dictionaryapi.com/api/v3/references/collegiate/json/{}?key=04a5d981-0869-42c8-a87c-c8cbfdcfcb56", vocab);
            urilist.push(uri);
        }
        chunk_cnt += 1;

        let mut jsonlist_running = match block_on(get_json(urilist)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                return Err("Invalid response".into())
            }
        };

        jsonlist.append(&mut jsonlist_running);
    }


    for (i, mut json) in jsonlist.iter().enumerate() {
        let reslist;
        if json[0]["hwi"]["hw"].is_null() {
            // The definition of the word cannot be found. A list of close words is given in an array. Take the first word as the vocab.
            let vocab = &json[0];
            debug_print(format!("Definition for '{}' not found. Trying {} instead.", wordlist[i].word, vocab), 0, v);
            let uri = format!("https://dictionaryapi.com/api/v3/references/collegiate/json/{}?key=04a5d981-0869-42c8-a87c-c8cbfdcfcb56", vocab);
            let urilist = vec![uri];
            reslist = match block_on(get_json(urilist)) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("{}", e);
                    return Err("Invalid response".into())
                }
            };
            json = &reslist[0];
        }

        wordlist[i].headword = json[0]["hwi"]["hw"].to_string().replace("*", "").trim_matches('"').to_string();
        wordlist[i].pronunciation = match json[0]["hwi"]["prs"][0]["mw"].is_null() {
            true => "".to_string(),
            false => json[0]["hwi"]["prs"][0]["mw"].to_string().trim_matches('"').to_string(),
        };
        wordlist[i].definition = json[0]["shortdef"].to_string().replace("\",\"", " ; ").trim_matches('[').trim_matches(']').trim_matches('"').to_string();
        
        debug_print(format!("Received {} json: {}", i+1, wordlist[i].headword), 1, v);
        debug_print(format!("{}", wordlist[i]), 2, v);
        debug_print(format!("{}", json), 3, v);
        
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
