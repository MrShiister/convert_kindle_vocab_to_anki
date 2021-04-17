use std::error::Error;
use super::{debug_print, Word};
use surf;
use serde_json::Value;
use futures::{
    executor::block_on,
    future::try_join_all,
};

pub fn get_definition(vocablist: Vec<String>, v: u8) -> Result<Vec<Word>, Box<dyn Error>> {

    let mut urilist: Vec<String> = Vec::new();
    let mut wordlist: Vec<Word> = Vec::new();

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

    for json in jsonlist.iter() {
        debug_print(format!("Received json:\n{}", json), 3, v);

        let word = Word {
                        //word: vocablist[i].to_string(),
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

    let mut futlist = Vec::new();

    for uri in urilist {
        let fut = surf::get(uri).recv_json();
        futlist.push(fut);
    }
    
    let reslist = try_join_all(futlist).await?;

    Ok(reslist)
}
