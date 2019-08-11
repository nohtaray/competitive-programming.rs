/// Bellman Ford
/// 関係ない負閉路を無視する版。
#[allow(dead_code)]
fn bellman_ford(graph: &[Vec<(usize, i64)>], from_v: usize, to_v: usize) -> Option<i64> {
    /// graph 上で from_v から到達できるかどうかのベクタ
    fn reachable(graph: &[Vec<(usize, i64)>], from_v: usize) -> Vec<bool> {
        let mut ret: Vec<bool> = vec![false; graph.len()];
        ret[from_v] = true;

        // dfs
        let mut stack: Vec<usize> = vec![];
        stack.push(from_v);
        while let Some(v) = stack.pop() {
            for &(u, _) in &graph[v] {
                if !ret[u] {
                    ret[u] = true;
                    stack.push(u);
                }
            }
        }
        ret
    }

    // from_v から到達でき、かつ to_v へ到達できる頂点のみ考える
    // これら以外には負閉路があっても関係ない
    let mut vertices: Vec<usize> = vec![];
    let mut rev: Vec<Vec<(usize, i64)>> = vec![vec![]; graph.len()];
    for (v, edges) in graph.iter().enumerate() {
        for &(u, d) in edges {
            rev[u].push((v, d));
        }
    }
    let r1 = reachable(&graph, from_v);
    let r2 = reachable(&rev, to_v);
    for v in 0..graph.len() {
        if r1[v] && r2[v] {
            vertices.push(v);
        }
    }

    // bellman ford
    let inf = i64::pow(10, 18);
    let mut dist = vec![inf; graph.len()];
    dist[from_v] = 0;
    let mut updated = true;
    for _ in 0..graph.len() {
        updated = false;
        for v in &vertices {
            for &(u, d) in &graph[*v] {
                if dist[*v] + d < dist[u] {
                    dist[u] = dist[*v] + d;
                    updated = true;
                }
            }
        }
        if !updated {
            break;
        }
    }

    if !updated && dist[to_v] < inf {
        Some(dist[to_v])
    } else {
        None
    }
}
