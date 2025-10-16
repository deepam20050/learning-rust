use proconio::input;

fn dfs(u: usize, g: &Vec<Vec<usize>>, visited: &mut Vec<usize>) {
    visited[u] = 1;
    for to in &g[u] {
        if visited[*to] == 0 {
            dfs(*to, g, visited);
        }
    }
}

fn main() {
    input! {
        n: usize
    }
    let mut g = vec![Vec::<usize>::new(); n + 1];
    let mut visited = vec![0 as usize; n + 1];
    for i in 1..=n {
        input! {
            a: usize,
            b: usize,
        }
        if a == 0 && b == 0 {
            visited[i] = 1;
        } else {
            if a != 0 {
                g[a].push(i);
            }
            if b != 0 {
                g[b].push(i);
            }
        }
    }
    for i in 1..=n {
        if visited[i] == 1 {
            dfs(i, &g, &mut visited);
        }
    }
    let skills_learnt = visited.iter().filter(|&&x| x == 1).count();
    println!("{}", skills_learnt);
}
