use std::collections::HashMap;

static METHODS: [&str; 9] = [
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
    value: Option<V>,
    params: Option<Vec<Param>>,
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
        let mut impetuous: Impetuous<V> = Impetuous {
            routes: HashMap::new()
        };
        for i in 0..METHODS.len() {
            impetuous.routes.insert(String::from(METHODS[i]), RootNode {
                static_routes: HashMap::new(),
                dynamic_routes: Node {
                    value: None,
                    params: None,
                    routes: HashMap::new()
                }
            });
        }
        impetuous
    }

    pub fn add(&self) -> Self {
        unimplemented!()
    }

    fn add_dynamic(&self) {
        unimplemented!()
    }

    pub fn find(&self) {
        unimplemented!()
    }

    fn find_dynamic(&self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Impetuous;

    #[test]
    fn new_impetuous() {
        let impetuous: Impetuous<i32> = Impetuous::new();
        assert_eq!(impetuous.routes.is_empty(), false);
    }
}
