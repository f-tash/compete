use std::collections::BTreeMap;
use num_integer::Integer;
use std::ops::RangeBounds;

pub fn integer_factorization(mut n: usize) -> BTreeMap<usize, usize> {
    let mut map = BTreeMap::new();
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            *map.entry(i).or_insert(0) += 1;
            n /= i;
        }
        i += 1;
    }
    if n != 0 && n != 1 {
        *map.entry(n).or_insert(0) += 1;
    }
    map
}
pub trait PartitionPoint<T> {
    fn partition_point<P>(&self, pred: P) -> T
    where
        P: Fn(T) -> bool;
}

impl<T, R> PartitionPoint<T> for R
where
    T: Integer + Clone,
    R: RangeBounds<T>,
{
    fn partition_point<P>(&self, pred: P) -> T
    where
        P: Fn(T) -> bool,
    {
        let one = T::one();
        let mut r = match self.end_bound() {
            std::ops::Bound::Included(r) => r.clone() + one.clone(),
            std::ops::Bound::Excluded(r) => r.clone(),
            std::ops::Bound::Unbounded => panic!("Unbounded ranges are not supported"),
        };
        let mut l = match self.start_bound() {
            std::ops::Bound::Included(l) => l.clone(),
            std::ops::Bound::Excluded(l) => l.clone() + one.clone(),
            std::ops::Bound::Unbounded => panic!("Unbounded ranges are not supported"),
        };

        while l != r {
            let mid = l.clone() + ((r.clone() - l.clone()) / (one.clone() + one.clone()));
            if pred(mid.clone()) {
                l = mid + one.clone();
            } else {
                r = mid;
            }
        }
        l
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_integer_factorization_prime() {
        // 素数の場合
        let mut expected = BTreeMap::new();
        expected.insert(13, 1);
        assert_eq!(integer_factorization(13), expected);
    }

    #[test]
    fn test_integer_factorization_composite() {
        // 合成数の場合
        let mut expected = BTreeMap::new();
        expected.insert(2, 2);
        expected.insert(3, 1);
        assert_eq!(integer_factorization(12), expected);
    }

    #[test]
    fn test_integer_factorization_large_number() {
        // 大きい合成数の場合
        let mut expected = BTreeMap::new();
        expected.insert(2, 3);
        expected.insert(5, 1);
        assert_eq!(integer_factorization(40), expected);
    }

    #[test]
    fn test_integer_factorization_one() {
        // 1の場合 (素因数分解は空のBTreeMap)
        let expected = BTreeMap::new();
        assert_eq!(integer_factorization(1), expected);
    }

    #[test]
    fn test_integer_factorization_zero() {
        // 0の場合（結果が定義されていないが、ここでは空のBTreeMapとして扱う）
        let expected = BTreeMap::new();
        assert_eq!(integer_factorization(0), expected);
    }
}

#[cfg(test)]
mod tests_p {
    use super::*;
    use num_bigint::BigInt;
    use num_traits::FromPrimitive;

    #[test]
    fn test_partition_point_usize() {
        let range = 0..=100;
        let result = range.partition_point(|x| x < 50);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_partition_point_u64() {
        let range = 0u64..=100u64;
        let result = range.partition_point(|x| x * x < 2500);
        assert_eq!(result, 50u64);
    }

    #[test]
    fn test_partition_point_bigint() {
        let start = BigInt::from_u32(0).unwrap();
        let end = BigInt::from_u32(1000).unwrap();
        let target = BigInt::from_u32(500).unwrap();

        let range = start.clone()..=end.clone();
        let result = range.partition_point(|x| x < target.clone());
        assert_eq!(result, target);
    }

    #[test]
    fn test_partition_point_bigint_large() {
        let start = BigInt::from_u8(0).unwrap();
        let end = BigInt::from_u64(1_000_000).unwrap();
        let target = BigInt::from_u64(500_000).unwrap();

        let range = start.clone()..=end.clone();
        let result = range.partition_point(|x| x < target.clone());
        assert_eq!(result, target);
    }
    #[test]
    fn test_lowerbound(){
        let range = 45..100;
        let bound = 50;
        let result = range.partition_point(|y| y < bound );
        assert_eq!(result, 50);
    }
}