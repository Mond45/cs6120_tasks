use std::collections::HashSet;

fn postorder(u: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
    visited[u] = true;
    for v in graph[u].iter() {
        if !visited[*v] {
            postorder(*v, graph, visited, order);
        }
    }
    order.push(u);
}

pub fn find_dominators(preds: &Vec<Vec<usize>>, succs: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut dom: Vec<HashSet<usize>> = vec![(0..preds.len()).collect(); preds.len()];
    dom[0] = [0].into();

    /*
    dom = {every block -> all blocks}
    dom[entry] = {entry}
    while dom is still changing:
        for vertex in CFG except entry:
            dom[vertex] = {vertex} ∪ ⋂(dom[p] for p in vertex.preds}
    */

    let mut rev_postorder = vec![];
    let mut visited = vec![false; preds.len()];
    postorder(0, &succs, &mut visited, &mut rev_postorder);
    rev_postorder.pop();
    rev_postorder.reverse();

    let mut changing = true;
    while changing {
        changing = false;

        for v in rev_postorder.iter() {
            let mut new_dom: HashSet<_> = preds
                .get(*v)
                .expect("v should be in preds")
                .iter()
                .map(|pred| dom.get(*pred).expect("pred should be in dom"))
                .fold((0..preds.len()).collect(), |accum, val| {
                    accum.intersection(val).cloned().collect()
                });
            new_dom.insert(*v);

            changing |= dom.get(*v).unwrap() != &new_dom;
            dom[*v] = new_dom;
        }
    }

    dom.into_iter().map(|d| d.into_iter().collect()).collect()
}

pub fn rev_graph(graph: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut output = vec![HashSet::new(); graph.len()];

    for (from, tos) in graph.iter().enumerate() {
        for &to in tos {
            output[to].insert(from);
        }
    }

    output
        .into_iter()
        .map(|v| v.into_iter().collect())
        .collect()
}

pub fn form_dom_tree(dominators: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let dominates: Vec<HashSet<_>> = rev_graph(&dominators)
        .into_iter()
        .map(|v| v.into_iter().collect::<HashSet<_>>())
        .collect();

    let n = dominators.len();

    let mut dom_tree: Vec<Vec<usize>> = vec![Vec::new(); n];
    for a in 0..n {
        for b in 0..n {
            if a != b
                && dominates[a].contains(&b)
                && (0..n).into_iter().all(|c| {
                    if a == c || b == c {
                        true
                    } else {
                        !(dominates[a].contains(&c) && dominates[c].contains(&b))
                    }
                })
            {
                dom_tree[a].push(b);
            }
        }
        dom_tree[a].sort();
    }

    dom_tree
}

pub fn dom_frontier(dominators: &Vec<Vec<usize>>, preds: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = dominators.len();
    let mut frontier: Vec<Vec<usize>> = vec![Vec::new(); n];

    let dominates: Vec<HashSet<_>> = rev_graph(&dominators)
        .into_iter()
        .map(|v| v.into_iter().collect::<HashSet<_>>())
        .collect();

    for a in 0..n {
        for b in 0..n {
            if !(a != b && dominates[a].contains(&b))
                && preds[b].iter().any(|p| dominates[a].contains(p))
            {
                frontier[a].push(b);
            }
        }
    }

    frontier
}
