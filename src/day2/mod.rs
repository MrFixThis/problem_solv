mod btree;
mod editor;
mod huffman;
mod n_queen;
mod no_prefix;

pub fn start() {
    // N-Queens
    println!("{:#?}", n_queen::solve_queen::<4>());

    // Pre order traversal
    let root = btree::Node {
        item: 1,
        left: None,
        right: Some(Box::new(btree::Node {
            item: 2,
            left: None,
            right: Some(Box::new(btree::Node {
                item: 5,
                left: Some(Box::new(btree::Node {
                    item: 3,
                    left: None,
                    right: Some(Box::new(btree::Node {
                        item: 4,
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(btree::Node {
                    item: 6,
                    left: None,
                    right: None,
                })),
            })),
        })),
    };
    btree::pre_order(root);

    // No prefix
    let words = vec![
        "aab".to_owned(),
        "aac".to_owned(),
        "aacghgh".to_owned(),
        "aabghgh".to_owned(),
    ];
    no_prefix::no_prefix(&words);

    let dec = "1001011";
    let node = huffman::Node {
        _freq: 5,
        char: None,
        left: Some(Box::new(huffman::Node {
            _freq: 2,
            char: None,
            left: Some(Box::new(huffman::Node {
                _freq: 1,
                char: Some('B'),
                left: None,
                right: None,
            })),
            right: Some(Box::new(huffman::Node {
                _freq: 1,
                char: Some('C'),
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(huffman::Node {
            _freq: 3,
            char: Some('A'),
            left: None,
            right: None,
        })),
    };
    println!("{}", huffman::huff_dec(node, dec));

    // Text editor
    // editor::txt_ed().expect("damn! something went wrong");
}
