

fn can_place_flower(flowerbed: Vec<i32>, mut n: i32) -> bool {
    let mut flowerbed = flowerbed;
    let len = flowerbed.len();
    
    for i in 0..len {
        if flowerbed[i] == 0 && (i == 0 || flowerbed[i - 1] == 0) && (i == len - 1 || flowerbed[i + 1] == 0) {
            flowerbed[i] = 1; // Plant a flower here
            n -= 1; // Decrement the number of flowers we need to plant
            
            if n == 0 {
                return true; // We've planted all the required flowers
            }
        }
    }
    
    n <= 0
    
}

fn main() {
    let flowerbed = vec![0, 0, 1, 0, 0];
    let flowers = 1;
    let result = can_place_flower(flowerbed, flowers);
    println!("{:?}", result);

    let flowerbed = vec![1, 0, 0, 0, 1];
    let flowers = 2;
    let result = can_place_flower(flowerbed, flowers);
    println!("{:?}", result);

}