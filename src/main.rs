extern crate regex;

use std::collections::HashMap;
//use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;


type Stack<T> = Vec<T>;
//struct Stack<T> {
//    content: VecDeque<T>
//}
//impl<T> Stack<T> {
//    fn new() -> Stack<T> {
//        return Stack{content: VecDeque::new()};
//    }
//    fn push(&mut self, value: T) {
//        self.content.push_back(value);
//        return ;
//    }
//
//    fn pop(&mut self) -> Option<T> {
//        return self.content.pop_back();
//    }
//}

struct Node {
//    num: usize,
    state: char,            // オートマトンの状態('g'/'s')
    path: HashMap<char, usize>
}
impl Node {
    fn new(s: char) -> Node {
        let mut hm = HashMap::new();
        return Node{state: s, path: hm};
    }
    fn getPath(&self, c: char) -> Option<usize> {
        return self.path.get(&c).and_then(|i| Some(*i));
    }
	
	fn searchDestNode(
		stat: char, 
		ptr: &mut usize, 
		list: &mut Vec<Node>
	) {

		while *ptr >= 0 {
			if list[*ptr].state == stat {
				//dest = Some(list[*ptr]);
				break;
			}
			*ptr -= 1;
		}
		return;
	}
}

struct Input {
    input: char,            // オートマトンへの入力
    output: char,           // オートマトンの出力
    link: Stack<Link>
}
fn LoadData(filename: &str) -> Result<Vec<Input>, String> {
    // open file
    let path = Path::new(&filename);
    let display = path.display();
    let file = File::open(&path)
        .map_err(|e|format!("Cannot open {}", display).to_owned() 
            + " : " + e.description())?;

    // read file
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)
        .map_err(|e|"Cannot read file : ".to_owned() + e.description())?;
    
    // convert
    let mut raw = String::from_utf8(buf)
        .map_err(|e|e.description().to_owned())?;

    let mut list: Vec<Input> = Vec::new();
    for s in raw.split('\n') {
        let mut cs = s.chars().collect::<Vec<char>>();
        Regex::new("^[gw],[gw]$")
            .map_err(|e| e.description().to_owned())?
            .find(s)
            .and_then(|mat| {
                list.push(Input{
                    input: *cs.last().unwrap(),
                    output: *cs.first().unwrap(),
                    link: Stack::new()
                });
                Some(0)
            });
    }

    Ok(list)
}

struct Link {
    node_num: usize
}


fn main() {
    println!("Hello");

    // 対戦データのリスト
    let input_list = LoadData("input.dat").unwrap();
    let mut list_ptr: usize = 0; // 現在リストのどこを読んでいるか

    // オートマトンの推定結果
    let mut node_list: Vec<Node> = Vec::new();
    node_list.push(Node::new('w')); // ここに初手を設定する
    let mut node_ptr: usize = 0;

    while list_ptr < input_list.len() {
        let data = &input_list[list_ptr];
        let mut cur_node = &node_list[node_ptr];

        // すでにパスが存在するかのチェック
        match cur_node.getPath(data.input) {
            Some(index) => {
                let dest = &node_list[index];
                if dest.state == data.output {
                    // すでに飛び先が存在し, データと矛盾しない場合
                    node_ptr = index;
                } else {
                    // 飛び先が存在するがデータと矛盾する場合

                }
            }, 
            Node => {
                // 飛び先が存在しない場合
				
            },
        }
        list_ptr += 1;
    }
}
