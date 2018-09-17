use std::collections::HashSet;
use std::iter::FromIterator;

fn sum_of_2(data: &Vec<i32>) -> Vec<(i32, i32, usize, usize)> {
    let data_len = data.len();
    let mut sum_of_2: Vec<(i32, i32, usize, usize)> = Vec::<(i32, i32, usize, usize)>::new();
    for i in 0..data_len - 1 {
        for j in i + 1..data_len {
            sum_of_2.push((data[i] as i32, data[j] as i32, i, j));
        }
    }
    sum_of_2
}

fn sum_of_4(data: &Vec<i32>) -> HashSet<Vec<usize>> {
    let sum_of_2 = sum_of_2(&data);
    let mut sum_of_4 = HashSet::<Vec<usize>>::new();
    for i in 0..sum_of_2.len() - 1 {
        for j in i + 1..sum_of_2.len() {
            let sum = sum_of_2[i].0 + sum_of_2[i].1 + sum_of_2[j].0 + sum_of_2[j].1;
            if sum == 0 {
                let mut sum_vec = Vec::<usize>::new();
                sum_vec.push(sum_of_2[i].2);
                sum_vec.push(sum_of_2[i].3);
                sum_vec.push(sum_of_2[j].2);
                sum_vec.push(sum_of_2[j].3);
                sum_vec.sort();
                let sum_set: HashSet<usize> = HashSet::from_iter(sum_vec.iter().cloned());
                if sum_set.len() == 4 {
                    sum_of_4.insert(sum_vec);
                }
            }
        }
    }
    sum_of_4
}

fn main() {
    let data: Vec<i32> = vec![1, -8, 8, -1, -3, 4, -10, -8, -7, 7];
    let sum = sum_of_4(&data);
    for i in sum{
        println!("index:{:?} data:[{} {} {} {}]", i, data[i[0]], data[i[1]] , data[i[2]] ,data[i[3]]);
    }
}
