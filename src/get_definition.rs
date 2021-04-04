use std::error::Error;
use super::{debug_print, Word};
use surf;
use serde_json::Value;
use futures::{
    executor::block_on,
    stream::FuturesOrdered,
};

pub fn get_definition(vocablist: Vec<String>, v: u8) -> Result<Vec<Word>, Box<dyn Error>> {

    let urilist: Vec<String> = Vec::new();
    let wordlist: Vec<Word> = Vec::new();

    for vocab in vocablist {
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

        let word = Word {
                        word: vocablist[i].to_string(),
                    headword: json[0]["hwi"]["hw"].to_string(),
              pronunciations: json[0]["hwi"]["prs"][0]["mw"].to_string(),
            example_sentence: "".to_string(),
                  definition: json[0]["shortdef"].to_string(),
        };
        
        debug_print(format!("{}", word), 1, v);
        wordlist.push(word);
    }

    Ok(wordlist)
        
}

pub async fn get_json(urilist: Vec<String>) -> Result<Vec<Value>, Box<dyn Error>> {

    let reslist: Vec<Value> = Vec::new();
    let futlist = FuturesOrdered::new();

    for uri in urilist {
        let fut = surf::get(uri).recv_json();
        futlist.push(fut);
    }
    
    futlist.collect::<Vec<_>>(); // TODO: How to poll/collect?

//    let res = match surf::get(uri).recv_json().await {
//        Ok(s) => s,
//        Err(e) => {
//            eprintln!("{}", e);
//            // return Err("get failed".into())
//        }
//    };

    Ok(reslist)
}
