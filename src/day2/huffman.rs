use std::collections::VecDeque;

pub struct Node {
    pub _freq: usize,
    pub char: Option<char>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl AsRef<Node> for Node {
    fn as_ref(&self) -> &Node {
        self
    }
}

pub fn huff_dec(node: Node, str: &str) -> String {
    let mut text = String::new();
    let mut codes: VecDeque<_> = str.chars().collect();

    while !codes.is_empty() {
        traverse(&mut text, &mut codes, Some(&node));
    }

    text
}

fn traverse<N: AsRef<Node>>(txt: &mut String, codes: &mut VecDeque<char>, node: Option<&N>) {
    let Some(node) = node else { return };
    let node = N::as_ref(node);

    if let Some(char) = node.char {
        return txt.push(char);
    }

    if let Some('0') = codes.pop_front() {
        traverse(txt, codes, node.left.as_ref())
    } else {
        traverse(txt, codes, node.right.as_ref())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_traversal() {
        let dec = "1001011";
        let node = super::Node {
            _freq: 5,
            char: None,
            left: Some(Box::new(super::Node {
                _freq: 2,
                char: None,
                left: Some(Box::new(super::Node {
                    _freq: 1,
                    char: Some('B'),
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(super::Node {
                    _freq: 1,
                    char: Some('C'),
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(super::Node {
                _freq: 3,
                char: Some('A'),
                left: None,
                right: None,
            })),
        };
        println!("{}", super::huff_dec(node, dec));
    }
}
