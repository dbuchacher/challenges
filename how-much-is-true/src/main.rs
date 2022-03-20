/*
    Create a function which returns the number of true values there are in an array. - https://edabit.com/challenge/GLbuMfTtDWwDv2F73
*/

// use a traditional for loop to find the asnwer
fn count_true(a: &[bool]) -> usize {
    let mut counter = 0;

    for v in a {
        if v == &true {
            counter += 1;
        }
    }

    counter
}

// use a functional style to find the answer
fn count_true_functional(a: &[bool]) -> usize {
    a.iter().copied().filter(|&b| b).count()
}

fn main() {
    assert_eq!(count_true(&[true, false, false, true, false]), 2);
    assert_eq!(count_true(&[false, false, false, false]), 0);
    assert_eq!(count_true(&[]), 0);

    assert_eq!(count_true_functional(&[true, false, false, true, false]), 2);
    assert_eq!(count_true_functional(&[false, false, false, false]), 0);
    assert_eq!(count_true_functional(&[]), 0);
}