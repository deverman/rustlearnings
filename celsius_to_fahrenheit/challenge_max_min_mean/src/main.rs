fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;
    let mut total: f64 = 0.0;

    /* YOUR CODE GOES HERE */
    
    max = numbers[0];
    min = numbers[0];
    
    for num in numbers {
    
		if num > max {
			max = num;
	
		}
	
		if num < min {
	
			min = num;
	
		}
		
		total += num as f64;
	
    }
    
    mean = total/ numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}
