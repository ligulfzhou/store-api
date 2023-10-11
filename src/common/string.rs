use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn common_prefix(strs: Vec<String>) -> Option<String> {
    if strs.is_empty() {
        return None;
    }

    let mut res = "".to_string();
    let mut i = 0;
    loop {
        let mut c = None;
        for str in &strs {
            if i == str.len() {
                return Some(res);
            }

            match c {
                None => {
                    c = Some(str.as_bytes()[i]);
                }
                Some(letter) if letter != str.as_bytes()[i] => return Some(res),
                _ => continue,
            }
        }
        if let Some(letter) = c {
            res.push(char::from(letter));
        }

        i += 1;
    }
}

fn lcp(list: &[&[u8]]) -> Option<Vec<u8>> {
    if list.is_empty() {
        return None;
    }
    let mut ret = Vec::new();
    let mut i = 0;
    loop {
        let mut c = None;
        for word in list {
            if i == word.len() {
                return Some(ret);
            }
            match c {
                None => {
                    c = Some(word[i]);
                }
                Some(letter) if letter != word[i] => return Some(ret),
                _ => continue,
            }
        }
        if let Some(letter) = c {
            ret.push(letter);
        }
        i += 1;
    }
}

pub fn remove_whitespace_string(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

pub fn remove_whitespace_str(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn random_string(n: usize) -> String {
    let s: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(|x| x as char)
        .collect();
    s
}

pub fn is_empty_string_vec(str_vec: Vec<&str>) -> bool {
    for x in str_vec {
        if !x.is_empty() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::common::string::common_prefix;

    #[test]
    fn test_lcp() -> anyhow::Result<()> {
        let cp = common_prefix(vec![
            "helloworld".to_string(),
            "hellobob".to_string(),
            "hellxyz".to_string(),
        ]);
        tracing::info!("cp: {:?}", cp);
        Ok(())
    }
}
