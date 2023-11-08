#![allow(dead_code)]
use std::cmp::Ordering;
pub fn orientation(p: &(f64, f64), q: &(f64, f64), r: &(f64, f64)) -> Ordering {
    //det[pq || qr]
    ((q.1 - p.1) * (r.0 - q.0) - (q.0 - p.0) * (r.1 - q.1)).total_cmp(&0.0)
}
pub fn graham_scan(mut points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    //Punkt auswählen
    let mut pos = 0;
    for i in 1..points.len() {
        if points[i].1 < points[pos].1
            || (points[i].1 == points[pos].1 && points[i].0 < points[pos].0)
        {
            pos = i;
        }
    }
    points.swap(0, pos);

    //restliche Punkte sortieren
    let ref_point = points[0];
    points[1..].sort_by(|a, b| orientation(a, b, &ref_point));

    //hülle aus referenz punkt und sortierter restliste bilden
    let mut hull: Vec<(f64, f64)> = Vec::new();
    hull.push(points[0]);
    hull.push(points[1]);
    for point in points.into_iter().skip(2) {
        while hull.len() > 1
            && orientation(&hull[hull.len() - 2], &hull[hull.len() - 1], &point) == Ordering::Less
        {
            hull.pop();
        }
        hull.push(point);
    }

    hull
}

fn kruskal_mst(vertices: u32, mut edges: Vec<(u32, u32, u32)>) -> Vec<(u32, u32, u32)> {

    fn find(parent: &mut Vec<u32>, i: u32) -> u32 {
        if parent[i as usize] != i {
            parent[i as usize] = find(parent, parent[i as usize]);
        }
        parent[i as usize]
    }

    fn union(parent: &mut Vec<u32>, rank: &mut [u32], x: u32, y: u32) {
        let x_root = find(parent, x);
        let y_root = find(parent, y);

        match rank[x_root as usize].cmp(&rank[y_root as usize]) {
            std::cmp::Ordering::Less => parent[x_root as usize] = y_root,
            std::cmp::Ordering::Equal => {
                parent[y_root as usize] = x_root;
                rank[x_root as usize] += 1;
            }
            std::cmp::Ordering::Greater => parent[y_root as usize] = x_root,
        }
    }

    let mut result: Vec<(u32, u32, u32)> = Vec::new();

    let mut i = 0;
    let mut e = 0;
    let mut parent: Vec<u32> = Vec::new();
    let mut rank: Vec<u32> = Vec::new();

    edges.sort_by(|a, b| a.2.cmp(&b.2));

    for v in 0..vertices {
        parent.push(v);
        rank.push(0);
    }

    while e < vertices - 1 && i < edges.len() {
        let next_edge = &edges[i];

        let x = find(&mut parent, next_edge.0);
        let y = find(&mut parent, next_edge.1);

        if x != y {
            result.push(*next_edge);
            union(&mut parent, &mut rank, x, y);
            e += 1;
        }

        i += 1;
    }

    result
}