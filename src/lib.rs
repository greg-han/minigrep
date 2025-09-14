

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

//query's lifetime is elided, and contents lifetime needs to be explicitly
//mentioned, because otherwise Rust wouldn't know which variable's lifetime the return value needs
//to be dependent on
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //query is shadowed now
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        //signature of contains() takes a string slice, hence, reference
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }

    results
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
