fn sum_dif_rev(n: usize) -> usize {
    (1..)
        .map(|x| x * 9)
        .filter(|&x| x % 10 != 0)
        .filter(|&x| {
            let r = revbit(x);
            r != x && (r + x) % dif(r, x) == 0
        })
        .nth(n - 1)
        .unwrap()
}

fn dif(m: usize, n: usize) -> usize {
    if m > n {
        m - n
    } else {
        n - m
    }
}

fn revbit(mut n: usize) -> usize {
    let mut m = 0;
    while n != 0 {
        m *= 10;
        m += n % 10;
        n /= 10;
    }
    m
}
// use std::collections::BTreeSet;
//
// fn sum_dif_rev(n: usize) -> usize {
//     let mut result = BTreeSet::new();
//     fn reverse(mut l: usize) -> usize {
//         let radix = 10;
//         let mut reversed = 0;
//         while l != 0 {
//             reversed = reversed * radix + l % radix;
//             l /= radix;
//         }
//         reversed
//     }
//     let mut c = 45;
//     while result.len() <= n {
//         let r = reverse(c);
//         if c % 10 == 0 || result.contains(&c) || (c as i32 - r as i32).abs() == 0 {
//             c += 1;
//             continue;
//         }
//         if (c as i32 + r as i32) % (c as i32 - r as i32).abs() == 0 {
//             result.insert(c);
//             result.insert(r);
//         }
//         c += 1;
//     }
//     println!("result is {:?}", result);
//     *result.iter().nth(n-1).unwrap()
// }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(sum_dif_rev(1), 45);
        assert_eq!(sum_dif_rev(3), 495);
        assert_eq!(sum_dif_rev(4), 594);
    }
}