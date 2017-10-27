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
    state: char,            // $B%*!<%H%^%H%s$N>uBV(B('g'/'s')
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
    input: char,            // $B%*!<%H%^%H%s$X$NF~NO(B
    output: char,           // $B%*!<%H%^%H%s$N=PNO(B
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

    // $BBP@o%G!<%?$N%j%9%H(B
    let input_list = LoadData("input.dat").unwrap();
    let mut list_ptr: usize = 0; // $B8=:_%j%9%H$N$I$3$rFI$s$G$$$k$+(B

    // $B%*!<%H%^%H%s$N?dDj7k2L(B
    let mut node_list: Vec<Node> = Vec::new();
    node_list.push(Node::new('w')); // $B$3$3$K=i<j$r@_Dj$9$k(B
    let mut node_ptr: usize = 0;

    while list_ptr < input_list.len() {
        let data = &input_list[list_ptr];
        let mut cur_node = &node_list[node_ptr];

        // $B$9$G$K%Q%9$,B8:_$9$k$+$N%A%'%C%/(B
        match cur_node.getPath(data.input) {
            Some(index) => {
                let dest = &node_list[index];
                if dest.state == data.output {
                    // $B$9$G$KHt$S@h$,B8:_$7(B, $B%G!<%?$HL7=b$7$J$$>l9g(B
                    node_ptr = index;
                } else {
                    // $BHt$S@h$,B8:_$9$k$,%G!<%?$HL7=b$9$k>l9g(B

                }
            }, 
            Node => {
                // $BHt$S@h$,B8:_$7$J$$>l9g(B
				
            },
        }
        list_ptr += 1;
    }
}
