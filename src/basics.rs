/**
    Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/
pub fn gauss(n: i32) -> i32 {
    let mut sum = 0;
    if n < 0{
        sum = -1;
    }
    else {
        sum = (n * (n + 1)) / 2;
    }
    return sum;
}

/**
    Returns the number of elements in the list that 
    are in the range [s,e]
**/
pub fn in_range(ls: &[i32], s: i32, e: i32) -> i32 {
    let mut num_of_elem = 0;
    for i in ls{
      if i >= &s && i <= &e {
        num_of_elem += 1;
      }
    }
    return num_of_elem;
}

/**
    Returns true if target is a subset of set, false otherwise

    Ex: [1,3,2] is a subset of [1,2,3,4,5]
**/
pub fn subset<T: PartialEq>(set: &[T], target: &[T]) -> bool {
    let mut is_subset = true;
    for i in target{
      if !set.contains(i) {
        is_subset = false;
      }
    }
    return is_subset;
}

/**
    Returns the mean of elements in ls. If the list is empty, return None
    It might be helpful to use the fold method of the Iterator trait
**/
pub fn mean(ls: &[f64]) -> Option<f64> {
    let mut mean = 0.0;
    let mut sum = 0.0;
    if ls.len() == 0{
        return None;
    }
    else{
        for i in ls{
            sum += i;
        }
        mean = sum / ls.len() as f64;
    }
    return Some(mean);
}

/**
    Converts a binary number to decimal, where each bit is stored in order in the array
    
    Ex: to_decimal of [1,0,1,0] returns 10
**/
pub fn to_decimal(ls: &[i32]) -> i32 {
    ls.iter().fold(0, |acc, &a| acc*2 + a)
}

/**
    Decomposes an integer into its prime factors and returns them in a vector
    You can assume factorize will never be passed anything less than 2

    Ex: factorize of 36 should return [2,2,3,3] since 36 = 2 * 2 * 3 * 3
**/
pub fn factorize(n: u32) -> Vec<u32> {
    let mut vec = Vec::new();
    let mut num = n;
    
    while (num % 2 == 0){
      vec.push(2);
      num = num / 2;
    }

    let mut i = 3;
    while i <= (num as f64).sqrt() as u32{
      while num % i == 0{
        vec.push(i);
        num = num / i;
      }
      i = i + 2;
    }

    if (num > 2) {
      vec.push(num);
    }

    return vec;
}

/** 
    Takes all of the elements of the given slice and creates a new vector.
    The new vector takes all the elements of the original and rotates them, 
    so the first becomes the last, the second becomes first, and so on.
    
    EX: rotate [1,2,3,4] returns [2,3,4,1]
**/
pub fn rotate(lst: &[i32]) -> Vec<i32> {
    let mut vec = Vec::new();
    if lst.len() == 0 {
        return vec;
    }
    if lst.len() > 1 {
        for i in 1..lst.len() {
            vec.push(lst[i]);
        }
    }
    vec.push(lst[0]);
    return vec;
}

/**
    Returns true if target is a subtring of s, false otherwise
    You should not use the contains function of the string library in your implementation
    
    Ex: "ace" is a substring of "rustacean"
**/
pub fn substr(s: &String, target: &str) -> bool {

    if target == "" {
        return true;
    }

    for i in 0..(s.len()) {
        if ((s.len()) - i) < target.len() {
            return false;
        } 
        if &s[i..i+(target.len())] == target {
            return true;
        }
    }

    return false;
}

/**
    Takes a string and returns the first longest substring of consecutive equal characters

    EX: longest_sequence of "ababbba" is Some("bbb")
    EX: longest_sequence of "aaabbb" is Some("aaa")
    EX: longest_sequence of "xyz" is Some("x")
    EX: longest_sequence of "" is None
**/

pub fn longest_sequence(s: &str) -> Option<&str> {
    let mut start = 0;
    let mut curr_start = 0;
    let mut count = 0;
    let mut curr_count = 0;
    let mut curr_char = "";

    if s == "" {
        return None;
    }

    for i in 0..(s.len()) {
        if curr_char == &s[i..=i] {
            curr_count = curr_count + 1; 
            if curr_count > count { 
                count = curr_count;
                start = curr_start;
            }
        } else {
            curr_char = &s[i..=i]; 
            curr_start = i; 
            curr_count = 1; 
        }
    }
    return Some (&s[start..=(start + count - 1)]);
}
