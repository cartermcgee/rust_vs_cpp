// CODE ADAPTED FROM https://www.geeksforgeeks.org/dijkstras-shortest-path-algorithm-greedy-algo-7/

const V: usize = 9;

fn find_min_distance(dist: [i32; V], spt_set: [bool; V]) -> i32 {
    let mut min = std::i32::MAX;
    let mut min_index = 0;
    for i in 0..V{
        if !spt_set[i] && dist[i] <= min{
            min = dist[i];
            min_index = i as i32;
        }
    }
    return min_index;
}

fn print_solution(dist: [i32; V]){
    println!("Vertex \t\t Distance From Source");
    for i in 0..V{
        println!("{} \t\t {}", i, dist[i]);
    }
}

fn djikstra(graph: [[i32; V]; V], src: usize){
    let mut dist: [i32; V] = [std::i32::MAX; V];
    let mut spt_set: [bool; V] = [false; V];
    dist[src] = 0;
    for _ in 0..V-1{
        let u = find_min_distance(dist, spt_set) as usize;
        spt_set[u] = true;
        for v in 0..V{
            if !spt_set[v] && graph[u][v] != 0 && dist[u] != std::i32::MAX
                && dist[u] + graph[u][v] < dist[v]{
                dist[v] = dist[u] + graph[u][v];
            }
        }
    }
    print_solution(dist);
}

fn main() {
    let graph: [[i32; V]; V] =  [ [ 0, 4, 0, 0, 0, 0, 0, 8, 0 ],
                                   [ 4, 0, 8, 0, 0, 0, 0, 11, 0 ],
                                   [ 0, 8, 0, 7, 0, 4, 0, 0, 2 ],
                                   [ 0, 0, 7, 0, 9, 14, 0, 0, 0 ],
                                   [ 0, 0, 0, 9, 0, 10, 0, 0, 0 ],
                                   [ 0, 0, 4, 14, 10, 0, 2, 0, 0 ],
                                   [ 0, 0, 0, 0, 0, 2, 0, 1, 6 ],
                                   [ 8, 11, 0, 0, 0, 0, 1, 0, 7 ],
                                   [ 0, 0, 2, 0, 0, 0, 6, 7, 0 ] ];
    djikstra(graph, 0);
}
