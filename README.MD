#Day 39: 4-sum
<br>
In 2-sum, 3-sum or 4-sum problems we are searching for a tuples, triplets or quadruplets that sum to zero. Or any other value.
<br>
There are several strategies to approach 4-sum problem. The one I chose builds a hash map to search for pairs of tuples that satisfy (2sum, -2sum).
<br>
We can also set similarity = I{sum(tuple1) == -sum(tuple2)} to redefine the problem into concept of similarity. That brings us quickly into Locality-Sensitive Hashing and big data mining where the same algorithm can be successfully deployed and tweaked for memory/speed trade-off.
<br>
run:
```
fn main() {
    let data: Vec<i32> = vec![1, -8, 8, -1, -3, 4, -10, -8, -7, 7];
    let sum = sum_of_4(&data);
    for i in sum{
        println!("index:{:?} data:[{} {} {} {}]", i, data[i[0]], data[i[1]] , data[i[2]] ,data[i[3]]);
    }
}
```
result:
```
index:[0, 3, 8, 9] data:[1 -1 -7 7]
index:[0, 2, 3, 7] data:[1 8 -1 -8]
index:[1, 2, 8, 9] data:[-8 8 -7 7]
index:[2, 7, 8, 9] data:[8 -8 -7 7]
index:[1, 4, 5, 9] data:[-8 -3 4 7]
index:[0, 1, 2, 3] data:[1 -8 8 -1]
index:[4, 5, 7, 9] data:[-3 4 -8 7]
index:[3, 5, 6, 9] data:[-1 4 -10 7]
```