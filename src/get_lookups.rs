use super::{debug_print, Word};
use rusqlite::{Connection, OpenFlags, Result};

pub fn get_lookups(dbpath: String, timestamp: u64, v: u8) -> Result<Vec<Word>> {

    let conn = Connection::open_with_flags(dbpath, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let mut stmt = conn.prepare("
        SELECT word_key, word, usage
        FROM LOOKUPS
        INNER JOIN WORDS
        ON LOOKUPS.word_key = WORDS.id
        WHERE LOOKUPS.timestamp >= ?
    ")?;
    let word_iter = stmt.query_map([timestamp], |row| {
        Ok(Word {
            word_key: row.get(0)?,
                word: row.get(1)?,
            headword: "".to_string(),
      pronunciations: "".to_string(),
    example_sentence: row.get(2)?,
          definition: "".to_string(),
        })
    })?;

    let mut wordlist: Vec<Word> = Vec::new();
    for word in word_iter {
        let word = word.unwrap();
        debug_print(format!("{}", word), 3, v);
        wordlist.push(word);
    }

    Ok(wordlist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_output() {
        let wordlist = get_lookups("vocab_test.db".to_string(), 1571009240989u64, 1).unwrap();
        for word in &wordlist {
            println!("{}", word);
        }
        assert_eq!(wordlist.len(), 7);
    }
}
