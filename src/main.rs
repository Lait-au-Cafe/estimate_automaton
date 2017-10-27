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
    state: char,            // $B%*!<%H%^%H%s$N>uBV(B('g'/'s')
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
    input: char,            // $B%*!<%H%^%H%s$X$NF~NO(B
    output: char,           // $B%*!<%H%^%H%s$N=PNO(B
    link: Stack<Link>
}
impl Input {
    fn push_link(&mut self, l: Link) {
        self.link.push(l);
    }

	fn scan_link(ptr: &mut i32, list: &mut Vec<Input>) -> Option<Link> {
		let mut res: Option<Link> = None;
		loop {
			if *ptr < 0 {
				panic!("Failed to Search link. ");
			}

			let mut link_list = &mut list[*ptr as usize].link;
			if !link_list.is_empty() {
				res = Some(link_list.pop().unwrap());
				break;
			}

			*ptr -= 1;
		}

		return res;
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

    // $BBP@o%G!<%?$N%j%9%H(B
    let mut input_list = Input::load_data("input.dat").unwrap();
    let mut list_ptr: usize = 0; // $B8=:_%j%9%H$N$I$3$rFI$s$G$$$k$+(B

    // $B%*!<%H%^%H%s$N?dDj7k2L(B
    let mut node_list: Vec<Node> = Vec::new();
    node_list.push(Node::new('w')); // $B$3$3$K=i<j$r@_Dj$9$k(B
    let mut node_ptr: usize = 0;

    while list_ptr < input_list.len() {

        // $B$9$G$K%Q%9$,B8:_$9$k$+$N%A%'%C%/(B
        match node_list[node_ptr].get_path(input_list[list_ptr].input) {
            Some(index) => {
                let dest = &node_list[index];
                if dest.state == input_list[list_ptr].output {
                    // $B$9$G$KHt$S@h$,B8:_$7(B, $B%G!<%?$HL7=b$7$J$$>l9g(B
                    node_ptr = index;
                } else {
                    // $BHt$S@h$,B8:_$9$k$,%G!<%?$HL7=b$9$k>l9g(B
					// $BF~NO%j%9%H$r$5$+$N$\$C$F%j%s%/$r8+$D$1$F2r=|(B
					let mut lptr = list_ptr;
					let link: Link = 
						Input::scan_link(&mut (lptr as i32), &mut input_list).unwrap();
					
					// $B?7$7$$Ht$S@h$rC5$9(B
					let mut ptr = link.dest as i32;
					Node::scan(input_list[list_ptr].output, &mut ptr, &node_list);
					
                }
            }, 
            Node => {
                // $BHt$S@h$,B8:_$7$J$$>l9g(B
                let mut ptr = node_ptr as i32;
                Node::scan(input_list[list_ptr].output, &mut ptr, &node_list);
				
                if ptr < 0 {
                    // $B?7$7$/%N!<%I$r:n@.(B
                    node_list.push(Node::new(input_list[list_ptr].output));
                    node_list[node_ptr].set_path(input_list[list_ptr].input, node_ptr+1);
                    node_ptr += 1;
                } else {
                    // $B%j%s%/$rD%$C$F0\F0(B
                    input_list[list_ptr].push_link(Link::new(ptr as usize));
					node_list[node_ptr].set_path(input_list[list_ptr].input, ptr as usize);
					node_ptr = ptr as usize;
                }
            },
        }
        list_ptr += 1;
    }
}
