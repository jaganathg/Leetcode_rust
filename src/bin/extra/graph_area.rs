
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn area_under_graph(points: &[Point]) -> f64 {
    let mut total_area = 0.0;
    let n = points.len();

    for i in 0..n-1 {
        let x1 = points[i].x;
        let y1 = points[i].y;
        let x2 = points[i+1].x;
        let y2 = points[i+1].y;

        let area = 0.5 * (x2 - x1) * (y1 + y2);
        total_area += area;
    }
    total_area
}

fn main() {
    let points = vec![
        Point { x: 1.0, y: 1.0 },
        Point { x: 3.0, y: 4.0 },
        Point { x: 5.0, y: 2.0 },
        Point { x: 7.0, y: 5.0 },
    ];
    let area = area_under_graph(&points);
    println!("Area under the graph: {}", area);
    

}