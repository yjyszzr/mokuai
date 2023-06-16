use itertools::Itertools;

#[test]
fn test_iterools() {
    // let numbers = vec![5, 3, 1, 4, 2];
    // let sorted_numbers: Vec<_> = numbers.into_iter().sorted().collect();
    // println!("{:?}", sorted_numbers);

    use itertools::repeat_call;
    use itertools::Itertools;
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::from(vec![2, 5, 3, 7, 8]);
    // extract each element in sorted order
    for element in repeat_call(|| heap.pop()).while_some() {
        print!("{}", element);
    }

    itertools::assert_equal(
        repeat_call(|| 1).take(5),
        vec![1, 1, 1, 1, 1]
    );

}
