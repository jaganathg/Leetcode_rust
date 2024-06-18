use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;
use std::collections::VecDeque;



fn hand_of_stright_brutal(hand: &mut Vec<i32>, group_size: i32) -> bool {
    if hand.len() % group_size as usize != 0 {
        return false;
    }

    hand.sort();

    while !hand.is_empty() {
        let first_card = hand[0];
        for i in 0..group_size {
            let card_to_find = first_card + i;
            if let Some(pos) = hand.iter().position(|&x| x == card_to_find) {
                hand.remove(pos);
            } else {
                return false;
            }
        }
    }
    true
}
// NOTE: Not working
fn hand_of_stright_dynamic(hand:  Vec<i32>, group_size: i32) -> bool {
    if hand.len() % group_size as usize != 0 {
        return false;
    }

    let mut count: HashMap<i32, i32> = HashMap::new();
    for &card in hand.iter() {
        *count.entry(card).or_insert(0) += 1;
    }

    let mut minH = BinaryHeap::new();
    for &card in count.keys() {
        minH.push(Reverse(card));
    }

    while let Some(Reverse(first)) = minH.peek() {
        let first = *first;

        for i in first..first + group_size {
            if let Some(card_count) = count.get_mut(&i) {
                if *card_count == 0 {
                    return false;
                }
                *card_count -= 1;
                if *card_count == 0 {
                    if i != first {
                        return false;
                    }
                    minH.pop();
                }
            } else {
                return false;
            }
        }
    }
    true
}

// !Learn this code
fn leetcode_solutions( hand: Vec<i32>, group_size: i32) -> bool {
    let mut h = HashMap::new();

        // here ref of hand is passed in for loop, still hand is valid
        for &n in &hand {
            *h.entry(n).or_insert(0) += 1;
        }

        for n in hand {
            let mut m = n;

            while h.contains_key(&(m - 1)) {
                m -= 1;
            }

            while m <= n {
                while h.contains_key(&m) {
                    for k in m..m+group_size {
                        if !h.contains_key(&k) {
                            return false;
                        }

                        if h[&k] == 1 {
                            h.remove(&k);
                        } else {
                            h.insert(k, h[&k] - 1);
                        }
                    }
                }

                m += 1;
            }
        }

        true
}


fn main(){
    let hand1 = vec![1, 2, 3, 6, 2, 3, 4, 7, 8];
    let group_size1 = 3;
    println!("{}", leetcode_solutions(hand1, group_size1)); // true

    let mut hand1 = vec![1, 2, 3, 6, 2, 3, 4, 7, 8];
    let group_size1 = 3;
    println!("{}", hand_of_stright_brutal(&mut hand1, group_size1)); // true

    let hand2 = vec![1, 2, 3, 4, 5];
    let group_size2 = 4;
    println!("{}", hand_of_stright_dynamic(hand2, group_size2)); // false
}
