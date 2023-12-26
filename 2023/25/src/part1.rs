use std::io::stdin;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(PartialEq, Eq, Hash)]
struct Edge {
    a: String,
    b: String,
}

impl Edge {
    fn new(from: &str, to: &str) -> Edge {
        let (a, b) = if from < to {
            (from, to)
        } else {
            (to, from)
        };
        Edge{a: a.to_owned(), b: b.to_owned()}
    }
}

// may need 'a
struct Graph(HashMap<String, HashSet<String>>);

impl Graph {
    fn new() -> Graph {
        Graph(HashMap::new())
    }

    fn add(&mut self, e: Edge) {
        self.0.entry(e.a.clone()).and_modify(|t| {t.insert(e.b.clone());}).or_insert_with(|| {
            let mut h = HashSet::new();
            h.insert(e.b.clone());
            h
        });
        self.0.entry(e.b.clone()).and_modify(|t| {t.insert(e.a.clone());}).or_insert_with(|| {
            let mut h = HashSet::new();
            h.insert(e.a);
            h
        });
    }

    fn remove(&mut self, e: Edge) {
        if let Some(neighbors) = self.0.get_mut(&e.a) {
            neighbors.remove(&e.b);
        }
        if let Some(neighbors) = self.0.get_mut(&e.b) {
            neighbors.remove(&e.a);
        }
    }

    fn count_children(&self, start: &str) -> usize {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);
        
        while let Some(node) = queue.pop_front() {
            if seen.contains(node) {
                continue;
            }
            seen.insert(node.clone());
            for neighbor in self.0.get(node).unwrap().iter() {
                queue.push_back(neighbor);
            }
        }

        seen.len()
    }


    fn edges(&self) -> HashSet<Edge> {
        let mut edges = HashSet::new();
        for (from, tos) in self.0.iter() {
            for to in tos.iter() {
                edges.insert(Edge::new(from, to));
            }
        }
        edges
    }
}

fn render_graphviz(edges: HashSet<Edge>) {
    println!("graph g {{");
    for edge in edges {
        println!("{} -- {} [label=\"({},{})\"]", edge.a, edge.b, edge.a, edge.b);
    }
    println!("}}");
}

// count all paths from each node to specified neighbor (modified BFS). If == 3, that edge is one of the 3, otherwise not
// Once all three found and removed (verify there are strictly three), count out from nodes of last edge and mult for result

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let blacklist = vec![
        "rrp", "xbm", "hsv", "rbk",
        "lkh", "qrz", "lzv", "dcm", "mpm",
    ];
    let mut graph = Graph::new();
    for line in lines {
        let (from, tos) = line.split_once(": ").unwrap();
        if blacklist.contains(&from) {continue;}
        for to in tos.split(" ") {
            if blacklist.contains(&to) {continue;}
            graph.add(Edge::new(from, to));
        }
    }

    // graph.remove(Edge::new("fxr", "fzb"));
    // graph.remove(Edge::new("mpq", "vgk"));
    // graph.remove(Edge::new("thl", "nmv"));


    render_graphviz(graph.edges());

    // let one = graph.count_children("thl");
    // let another = graph.count_children("nmv");
    // 
    // println!("total nodes: {}", graph.0.len());
    // println!("one: {} ;; another: {}", one, another);
    // println!("{}", one * another);


}
