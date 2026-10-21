use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_insensitive: bool, // true 大小写不敏感 false 大小写敏感
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, String> {
        if args.len() != 3 {
            return Err("use method: ./minigrep search_word filename".to_string());
        }

        // 默认不设置环境变量都是大小写敏感的搜索，如果设置了就是大小写不敏感
        let case_insensitive = env::var("CASE_INSENSITIVE").is_ok();

        // vec 底层是数组，是连续的内存，是不能move的
        // 必须使用clone复制出新的字符串，这里复习一下为什么数组/索引/vec的部分元素不能move，但是tup/struct可以
        Ok(Config {
            query: args[1].clone(),
            file_name: args[2].clone(),
            case_insensitive: case_insensitive,
        })
    }
}

// Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，不过无需指定具体将会返回的值的类型
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 读取文件内容
    let contents = fs::read_to_string(config.file_name)?;
    let result = if config.case_insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        // 默认都是大小写敏感的
        search(&config.query, &contents)
    };
    println!("SEARCH RESULT:");
    for i in result {
        println!("{}", i);
    }
    Ok(())
}

// 返回包含query的若干行文本
pub fn search<'a, 'b>(query: &'a str, contents: &'b str) -> Vec<&'b str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

// 大小写不敏感的搜索
pub fn search_case_insensitive<'a, 'b>(query: &'a str, contents: &'b str) -> Vec<&'b str> {
    let query = query.to_lowercase();
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    // 大小写敏感搜索
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    // 大小写不敏感搜索
    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape."],
            search_case_insensitive(query, contents)
        );
    }
}
