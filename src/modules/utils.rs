use std::str::FromStr;

pub fn check_pari() -> Result<String, String> {
    let f = "x";

    let n = i32::from_str(f);

    // match
    let m = match n {
        Ok(n) => n,
        Err(_) => Err("cannot parse")?,
    };

    if m % 2 == 0 {
        Ok("pari".to_string())
    } else {
        Err("non pari".to_string())
    }

    // map_or_else
    /*n.map_or_else(|_| Err("cannot parse")?,|m| {
        if m % 2 == 0 {
            Ok(String::from("pari"))
        } else {
            Err(String::from("non pari"))
        }
    })*/

    /*
    // unwrap
    let m = n.unwrap_or(1);
    if m % 2 == 0 {
        Ok(String::from("pari"))
    } else {
        Err(String::from("non pari"))
    }*/
}