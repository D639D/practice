#[derive(Debug)]
struct ValuesStruct {
    data: [i32; 6],
}

impl ValuesStruct {
    fn structure_instance(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> ValuesStruct {
        ValuesStruct {
            data: [a, b, c, d, e, f],
        }
    }

    fn search_for_index(&self, index: usize) -> &i32 {
        if index >= self.data.len() {
            panic!(
                "Максимальный индекс: {}, ваш индекс: {}.",
                self.data.len() - 1,
                index
            )
        } else {
            &self.data[index]
        }
    }
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6];
    let data1 = ValuesStruct { data: numbers };
    println!("1. {:?},", ValuesStruct::structure_instance(numbers));
    println!("2. {:?}", data1.search_for_index(6));
}
