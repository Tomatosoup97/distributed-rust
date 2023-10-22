use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub var: String,
    children: HashMap<String, Rc<RefCell<Node>>>,
    edges: HashMap<String, f64>,
}

impl Node {
    pub fn new(var: String) -> Self {
        Node {
            var,
            children: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, other_node: &Rc<RefCell<Node>>, edge_value: f64) {
        let other_node_var = other_node.borrow().var.clone();
        self.edges.insert(other_node_var.clone(), edge_value);
        self.children.insert(other_node_var, other_node.clone());
    }
}

fn cons_graph(equations: Vec<Vec<String>>, values: Vec<f64>) -> HashMap<String, Rc<RefCell<Node>>> {
    let mut graph = HashMap::new();

    for (equation, value) in equations.iter().zip(values.iter()) {
        let (a, b) = (equation[0].clone(), equation[1].clone());
        // a / b == value

        let a_node = graph
            .entry(a.clone())
            .or_insert(Rc::new(RefCell::new(Node::new(a))))
            .clone();

        let b_node = graph
            .entry(b.clone())
            .or_insert(Rc::new(RefCell::new(Node::new(b))))
            .clone();

        a_node.borrow_mut().add_edge(&b_node, *value);
        b_node.borrow_mut().add_edge(&a_node, 1. / *value);
    }

    graph
}

fn find_path(graph: &HashMap<String, Rc<RefCell<Node>>>, a: &str, b: &str) -> f64 {
    if a == b {
        return 1.;
    }

    let mut visited: HashSet<String> = HashSet::new();
    let mut stack: Vec<(Rc<RefCell<Node>>, f64)> = vec![(graph[a].clone(), 1.)];

    while let Some((node, val)) = stack.pop() {
        if node.borrow().var == b {
            return val;
        }

        if visited.contains(&node.borrow().var) {
            continue;
        }
        visited.insert(node.borrow().var.clone());

        for child in node.borrow().children.values() {
            let edge = node.borrow().edges[&child.borrow().var];
            stack.push((child.clone(), val * edge));
        }
    }
    -1.
}

fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    let graph = cons_graph(equations, values);
    let mut results: Vec<f64> = vec![];

    for query in queries.iter() {
        let (a, b) = (query[0].clone(), query[1].clone());

        match (graph.get(&a), graph.get(&b)) {
            (Some(_), Some(_)) => {
                results.push(find_path(&graph, &a, &b));
            }
            _ => {
                results.push(-1.);
            }
        }
    }

    results
}
