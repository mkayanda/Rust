fn main() {
    let parking_lot = [[1,3,5,7,9],
                       [2,4,6,8,10],
                       [11,13,15,17,19],
                       [12,14,16,18,20],
                       [21,23,25,27,29],
                       [22,24,26,28,30]];
    let number = parking_lot[0][0];
    println!("number is {}", number);

    let garage = [1,2,3,4,5,6,7,8,9,10];
    let _index: usize = garage.len();
    println!("last number is {}", garage[9]);

}