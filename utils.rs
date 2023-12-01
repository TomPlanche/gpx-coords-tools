///
/// # utils.rs
/// This file contains utility functions.
///
/// ## Author
/// * Tom Planche - <github.com/tomPlanche>
///

// IMPORTS ===================================================================================================  IMPORTS
use std::collections::HashMap;
// END IMPORTS ==========================================================================================   END IMPORTS

// VARIABLES ================================================================================================ VARIABLE
#[allow(dead_code)]
pub type FileCoordsHM = HashMap<String, HashMap<String, Vec<(usize, usize)>>>;

#[allow(dead_code)]
pub enum VecOrHashMap<T, U> {
    Vec(Vec<T>),
    HashMap(HashMap<T, U>),
}

#[allow(dead_code)]
impl<T, U> VecOrHashMap<T, U> {
    pub fn is_empty(&self) -> bool {
        match self {
            VecOrHashMap::Vec(vec) => vec.is_empty(),
            VecOrHashMap::HashMap(hashmap) => hashmap.is_empty(),
        }
    }
}
// END VARIABLES ======================================================================================= END VARIABLES

// FUNCTIONS ================================================================================================ FUNCTIONS
///
/// # get_unique_pairs
/// Returns the unique pairs of a vector.
///
/// ## Arguments
/// * `vectorT<T>` - The vector of T elements
///
/// ## Returns
/// * `Vec<(T, T)>` - The unique pairs of the vector
#[allow(dead_code)]
pub fn get_unique_pairs<T>(vector: &Vec<T>) -> Vec<(&T, &T)>
where T: Clone {
    let mut unique_pairs: Vec<(&T, &T)> = Vec::new();

    for (i, item_1) in vector.iter().enumerate() {
        for item_2 in vector.iter().skip(i + 1) {
            unique_pairs.push((
                item_1,
                item_2
                ));
        }
    }

    unique_pairs
}


#[allow(dead_code)]
pub(crate) fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[test]
fn test_get_unique_pairs() {
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    let unique_pairs: Vec<(&i32, &i32)> = get_unique_pairs(&vector);

    assert_eq!(unique_pairs, vec![
        (&1, &2), (&1, &3), (&1, &4), (&1, &5), (&2, &3), (&2, &4), (&2, &5), (&3, &4), (&3, &5), (&4, &5)
    ]);
}

// END FUNCTIONS =======================================================================================  END FUNCTIONS

//
// * End of file file_utils.rs
//
