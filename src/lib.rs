use num::Unsigned;

pub struct IntegerPack<T>
where
    T: Unsigned,
{
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
    T: Unsigned,
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
where T: Unsigned 
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // shortcuts to internal status
        let &mut IntegerPack {
            ref mut value,
            ref modulo,
            ref mut length,
            ref mut index,
        } = self;

        if *index < *length {
            // compute next value
            let v = *value % *modulo;

            // update internal state
            *index += 1;
            *value /= modulo;

            Some(v)
        } else {
            // end reached
            None
        }
    }
}

