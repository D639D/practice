#[derive(Debug)]
struct ValuesStruct {
    data: [i32; 6],
}

impl ValuesStruct {
    fn new(data: [i32; 6]) -> ValuesStruct {
        ValuesStruct { data }
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

    fn slice(&self, index: usize) -> &[i32] {
        if index > self.data.len() {
            panic!(
                "Максимальный индекс: {}, ваш индекс: {}",
                self.data.len() - 1,
                index
            )
        } else {
            &self.data[..index]
        }
    }

    fn trim(&self, left_index: usize, right_index: usize) -> &[i32] {
        if right_index > self.data.len() {
            panic!(
                "Максимальный индекс: {}, ваш индекс: {}",
                self.data.len() - 1,
                right_index
            );
        } else if left_index > right_index {
            panic!(
                "Первый индекс не может быть больше второго. Ваш первый индекс: {}, ваш второй индекс: {}",
                left_index,
                right_index
            );
        } else {
            &self.data[left_index..right_index]
        }
    }
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6];
    let data1 = ValuesStruct::new(numbers);
    println!("1. {:?},", data1);
    println!("2. {:?}", data1.search_for_index(5));
    println!("3. {:?}", data1.slice(6));
    println!("4. {:?}", data1.trim(1, 6));
}
