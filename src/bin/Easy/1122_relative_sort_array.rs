use std::collections::HashMap; 

fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut dp: HashMap<i32, i32> = HashMap::new();
    let mut end: Vec<i32> = Vec::new();

    // no need of HashMap for arr2.
    let arr2_set: HashMap<_, _> = arr2.iter().enumerate()
        .map(|(i, &v)| (v, i)).collect();

    for &i in arr1.iter() {
        if arr2_set.contains_key(&i) {
            *dp.entry(i).or_insert(0) += 1;
        } else {
            end.push(i);
        }
    }

    for &i in arr2.iter() {
        if let Some(&count) = dp.get(&i) {
            for _ in 0..count {
                res.push(i);
            }
        }
        
    }

    end.sort();    
    res.append(&mut end);



    res
}


fn main() {
    let arr1 = vec![2,3,1,3,2,4,6,7,9,2,19];
    let arr2 = vec![2,1,4,3,9,6];
    let res = relative_sort_array(arr1, arr2);
    println!("{:?}", res);

    let arr1 = vec![28,6,22,8,44,17];
    let arr2 = vec![22,28,8,6];

    let res = relative_sort_array(arr1, arr2);
    println!("{:?}", res);
}
