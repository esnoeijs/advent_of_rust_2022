use std::io;

use crate::file::read_lines;

#[derive(Debug)]
enum DirResult {
    SetPath(Path),
    AddPath(Path),
    UpPath(),
    DirSize(DirSize),
}

type Path = String;
type DirSize = u32;

trait Handler {
    fn handle(&self, value: &Vec<String>) -> Option<DirResult>;
}

struct DirHandler;

impl Handler for DirHandler {
    fn handle(&self, value: &Vec<String>) -> Option<DirResult> {
        match value.get(0).map(|s| s.as_str()) {
            Some("$ cd /") => Some(DirResult::SetPath("/".to_string())),
            Some("$ cd ..") => Some(DirResult::UpPath()),
            Some(cmd) if cmd.starts_with("$ cd ") => {
                Some(DirResult::AddPath(cmd["$ cd ".len()..].to_string()))
            }
            _ => None,
        }
    }
}

struct LsHandler;

impl Handler for LsHandler {
    fn handle(&self, value: &Vec<String>) -> Option<DirResult> {
        let default_cmd = &"".to_string();
        let cmd = value.get(0).unwrap_or(default_cmd);

        if cmd == "$ ls" {
            let dir_size = value.iter().fold(0, |size, line| {
                size + line
                    .split_whitespace()
                    .next()
                    .and_then(|num| num.parse().ok())
                    .unwrap_or(0)
            });
            return Some(DirResult::DirSize(dir_size));
        }
        None
    }
}

struct Chain {
    handlers: Vec<Box<dyn Handler>>,
}
impl Chain {
    fn new() -> Self {
        Self { handlers: vec![] }
    }

    fn add_handler(&mut self, handler: Box<dyn Handler>) {
        self.handlers.push(handler);
    }

    fn handle(&self, value: &Vec<String>) -> Option<DirResult> {
        for handler in &self.handlers {
            if let Some(result) = handler.handle(value) {
                return Some(result);
            };
        }
        None
    }
}

pub fn solution(filename: &String) {
    let mut handlers = Chain::new();
    handlers.add_handler(Box::new(DirHandler));
    handlers.add_handler(Box::new(LsHandler));

    let mut dir_sizes: Vec<(String, u32)> = vec![];
    let mut cur_path: String = "/".to_string();
    for request in read_request(read_lines(filename)).unwrap() {
        if let Some(res) = handlers.handle(&request) {
            match res {
                DirResult::SetPath(ref path) => cur_path = path.to_owned(),
                DirResult::UpPath() => {
                    let second_to_last_slash =
                        cur_path[0..cur_path.len() - 1].rfind("/").unwrap_or(0);
                    cur_path = cur_path[0..second_to_last_slash].to_string() + "/"
                }
                DirResult::AddPath(ref directory) => cur_path = cur_path + directory + "/",
                DirResult::DirSize(ref size) => {
                    dir_sizes.push((cur_path.to_owned(), size.to_owned()))
                }
            };
        }
    }

    dir_sizes.sort_by(|x, y| x.0.len().partial_cmp(&y.0.len()).unwrap());

    let copy = dir_sizes.clone();
    for dir in dir_sizes.iter_mut() {
        let new_size = copy
            .iter()
            .filter(|x| x.0.starts_with(&dir.0))
            .fold(0, |x, y| x + y.1);
        dir.1 = new_size;
    }

    let size: u32 = dir_sizes
        .iter()
        .map(|x| x.1)
        .filter(|x| x <= &100_000)
        .sum();

    println!("{:?}", size);
    let total_used_size = dir_sizes.get(0).unwrap().1;

    // disk size 70M
    // used size x
    // free space 30M
    //
    // 70M - used + deleted > 30M
    let chosen_dir = dir_sizes
        .iter()
        .map(|dir| dir.1)
        .filter(|size| 70_000_000 - total_used_size + size > 30_000_000)
        .min_by(|x, y| x.partial_cmp(&y).unwrap())
        .unwrap();
    println!("{:?}", chosen_dir);
}

fn read_request(
    lines: Result<std::io::Lines<std::io::BufReader<std::fs::File>>, std::io::Error>,
) -> Result<Vec<Vec<String>>, io::Error> {
    let mut request: Vec<String> = vec![];
    let mut requests: Vec<Vec<String>> = vec![];
    for line in lines.unwrap().map(|x| x.unwrap()) {
        if request.len() != 0 && &line[0..1] == "$" {
            requests.push(request.clone());
            request.clear();
            request.push("".to_string() + line.as_str());
        } else {
            request.push("".to_string() + line.as_str());
        }
    }
    requests.push(request.clone());

    return Ok(requests);
}
