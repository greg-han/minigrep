

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

//query's lifetime is elided, and contents lifetime needs to be explicitly
//mentioned, because otherwise Rust wouldn't know which variable's lifetime the return value needs
//to be dependent on
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //query is shadowed now and is no longer a string slice
    let query = query.to_lowercase();
    //but it's a slice now!
    //can also use .contains(&query), but purposefully verbose for mental note
    let query = &query[..];
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    
    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query,contents)
        );
    }
}
