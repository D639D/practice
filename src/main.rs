#[derive(Debug)]
struct ValuesStruct {
    data: [i32; 6],
}

impl ValuesStruct {
    fn structure_instance(data: [i32; 6]) -> ValuesStruct {
        ValuesStruct {
            data,
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
    let data1 = ValuesStruct::structure_instance(numbers);
    println!("1. {:?},", data1);
    println!("2. {:?}", data1.search_for_index(5));
}
