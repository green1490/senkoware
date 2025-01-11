use std::collections::VecDeque;

struct Node {
    child:Vec<Node>,
    name:String,
    is_file:bool
}
/* 
              N (root)
            / | \
(file.txt) N  N  N (etc, dev)
                 |
                 N (test.conf)
*/
#[test]
fn directory_discovery() -> Result<(), std::io::Error> {
    let root = Node {name: "Root".to_string(), is_file: false, child: vec![
        Node{name: "file.txt".to_string(), is_file: true, child: vec![]},
        Node{name: "etc".to_string(), is_file: false, child: vec![]},
        Node{name: "dev".to_string(), is_file: false, child: vec![
            Node{name: "test.conf".to_string(), is_file: true, child: vec![]},
        ]}
    ]};
    let mut stack:VecDeque<Node> = VecDeque::from(root.child);
    while stack.len() != 0 {
        let current_node = stack.pop_front().unwrap();
        stack.extend(current_node.child);
        println!("{}", current_node.name);
    }
    Ok(())
}

#[test]
#[allow(unused_variables)]
fn home_directory(){
    let home = home::home_dir().unwrap();
}