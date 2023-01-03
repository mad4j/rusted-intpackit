use num::{traits::NumAssign, ToPrimitive, Unsigned};

pub struct IntegerPack<T>
where
    T: Unsigned + NumAssign + ToPrimitive,
    T: PartialOrd + Copy {
    // resudual packed values
    value: T,
    // max value
    modulo: T,
    // number of packed values
    length: T,
    // next value to be extraced
    index: T,
}

pub fn unpack<T>(value: T, modulo: T, length: T) -> IntegerPack<T>
where
    T: Unsigned + NumAssign + ToPrimitive,
    T: PartialOrd + Copy 
{
    // intialize a new structure
    IntegerPack {
        value,
        modulo,
        length,
        index: T::zero(),
    }
}

impl<T> Iterator for IntegerPack<T>
where
    T: Unsigned + NumAssign + ToPrimitive,
    T: PartialOrd + Copy 
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.length {
            // compute next value
            let v = self.value % self.modulo;

            // update internal state
            self.index += T::one();
            self.value /= self.modulo;

            // return computed value
            Some(v)
        } else {
            // end reached
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let v = self.length - self.index;

        match v.to_usize() {
            Some(v) => (v, Some(v)),
            None => (0, None),
        }
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_values_01() {
        let mut it = unpack(0b11_00_01_10_11_00u16, 4, 7);
        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_values_02() {
        let mut it = unpack(0x89ABCDEFu32, 16, 9);
        assert_eq!(it.next(), Some(0xF));
        assert_eq!(it.next(), Some(0xE));
        assert_eq!(it.next(), Some(0xD));
        assert_eq!(it.next(), Some(0xC));
        assert_eq!(it.next(), Some(0xB));
        assert_eq!(it.next(), Some(0xA));
        assert_eq!(it.next(), Some(0x9));
        assert_eq!(it.next(), Some(0x8));
        assert_eq!(it.next(), Some(0x0));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_values_03() {
        let mut it = unpack((1*1 + 2*3 + 0*9 + 2*27 + 1*81) as u32, 3, 6);
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_meta_01() {
        let ref mut it = unpack(0x89ABCDEFu32, 16, 9);
        assert_eq!(it.count(), 9);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_meta_02() {
        let ref mut it = unpack(0x89ABCDEFu32, 16, 9);

        assert_eq!(it.size_hint(), (9, Some(9)));

        it.next();
        assert_eq!(it.size_hint(), (8, Some(8)));

        it.next();
        assert_eq!(it.size_hint(), (7, Some(7)));

        it.next();
        it.next();
        it.next();
        assert_eq!(it.size_hint(), (4, Some(4)));

        it.next();
        it.next();
        it.next();
        assert_eq!(it.size_hint(), (1, Some(1)));

        it.next();
        assert_eq!(it.size_hint(), (0, Some(0)));

        //how to test (0, None) ??
    }
}
