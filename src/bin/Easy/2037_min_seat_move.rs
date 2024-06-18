

fn min_move(seat: Vec<i32>, stud: Vec<i32>) -> i32 {
    let mut seat = seat;
    let mut stud = stud;
    seat.sort();
    stud.sort();

    let total_moves = seat.iter().zip(stud.iter()).map(|(a , b)| (a - b).abs()).sum();
    total_moves
}

fn main() {
    let seat = vec![3, 1, 5];
    let stud = vec![2, 7, 4];
    let result = min_move(seat, stud);
    println!("{:?}", result);

    let seat = vec![4, 1, 5, 9];
    let stud = vec![1, 3, 2, 6];
    let result = min_move(seat, stud);
    println!("{:?}", result);

    let seat = vec![1, 2, 3, 4, 5];
    let stud = vec![1, 2, 3, 4, 5];
    let result = min_move(seat, stud);
    println!("{:?}", result);
}