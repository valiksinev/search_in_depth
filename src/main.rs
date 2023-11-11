use std::io;
use std::collections::{BTreeMap, BTreeSet};

fn dfs(vertices: &Vec<u32>, set: &mut BTreeSet<u32>, map: &BTreeMap<u32, Vec<u32>> ) {
    for v in vertices {
        if set.insert(*v) {
            dfs(map.get(v).unwrap(), set, map);
        }
    }
}

fn main() {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();
    let params: Vec<&str> = s.split(' ').collect();

    let _n: u32 = params.get(0).unwrap().trim().parse().unwrap();
    let m: u32 = params.get(1).unwrap().trim().parse().unwrap();

    let mut map : BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for _i in 0..m {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let edges : Vec<&str> = s.split(' ').collect();
        let u: u32 = edges.get(0).unwrap().trim().parse().unwrap();
        let v: u32 = edges.get(1).unwrap().trim().parse().unwrap();
        let mut push = |a, b| {
            let vertices = map.entry(a).or_insert(vec![]);
            vertices.push(b);
        };
        push(u, v);
        push(v, u);
    }

    let mut set = BTreeSet::new();

    if let Some (vertices) = map.get(&1) {
        set.insert(1_u32);
        dfs(vertices, &mut set, &map);
    }

    println!("{}", set.len());
    let s = format!("{:?}", set).replace("{", "").replace("}", "").replace(",", "");
    println!("{}", s);
}


