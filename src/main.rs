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
    fn get_path(&self, c: char) -> Option<usize> {
        return self.path.get(&c).and_then(|i| Some(*i));
    }
    fn set_path(&mut self, c: char, dest: usize) {
        if self.path.insert(c, dest).is_some() {
            println!("PathError: the path already exists. ");
        }
    }
	
	fn scan(
		stat: char, 
		ptr: &mut i32, 
		list: &Vec<Node>
	) {

		while *ptr >= 0 {
			if list[*ptr as usize].state == stat {
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
impl Input {
    fn push_link(&mut self, l: Link) {
        self.link.push(l);
    }

    fn load_data(filename: &str) -> Result<Vec<Input>, String> {
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
}

struct Link {
    dest: usize
}
impl Link {
    fn new(i: usize) -> Link {
        return Link{dest: i};
    }
}


fn main() {
    println!("Hello");

    // 対戦データのリスト
    let mut input_list = Input::load_data("input.dat").unwrap();
    let mut list_ptr: usize = 0; // 現在リストのどこを読んでいるか

    // オートマトンの推定結果
    let mut node_list: Vec<Node> = Vec::new();
    node_list.push(Node::new('w')); // ここに初手を設定する
    let mut node_ptr: usize = 0;

    while list_ptr < input_list.len() {
        let data = &mut input_list[list_ptr];
        //let mut cur_node = &node_list[node_ptr];

        // すでにパスが存在するかのチェック
        match node_list[node_ptr].get_path(data.input) {
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
                let mut ptr = node_ptr as i32;
                Node::scan(data.output, &mut ptr, &node_list);
				
                if ptr < 0 {
                    // 新しくノードを作成
                    node_list.push(Node::new(data.output));
                    node_list[node_ptr].set_path(data.input, node_ptr+1);
                    node_ptr += 1;
                } else {
                    // リンクを張って移動
                    data.push_link(Link::new(ptr as usize));
                }
            },
        }
        list_ptr += 1;
    }
}
