use std::collections::HashMap;

const METHODS: [&str; 9] = [
    "GET",
    "HEAD",
    "POST",
    "PUT",
    "DELETE",
    "CONNECT",
    "OPTIONS",
    "TRACE",
    "PATCH"
];

struct Param {
    number: i32,
    name: String
}

struct Node<V> {
    value: V,
    params: Vec<Param>,
    routes: HashMap<String, Node<V>>
}

struct RootNode<V> {
    static_routes: HashMap<String, V>,
    dynamic_routes: Node<V>
}

struct Route<V> {
    handler: V,
    params: HashMap<String, String>
}

pub struct Impetuous<V> {
    routes: HashMap<String, RootNode<V>>
}

impl<V> Impetuous<V> {

    pub fn new() -> Impetuous<V> {
        unimplemented!()
    }

    pub fn add(&self, method: &str, path: &str, handler: V) -> self {
        unimplemented!()
    }

    fn add_dynamic(&self, root: Node<V>, path: &str, handler: V) {
        unimplemented!()
    }

    pub fn find(&self, method: &str, path: &str) -> Option<&Route<V>> {
        unimplemented!()
    }

    fn find_dynamic(&self, node: Node<V>, path: &str) -> Option<&Route<V>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
