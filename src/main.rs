#[derive(Debug)]
struct ValuesStruct {
    data: [i32; 6],
}

impl ValuesStruct {
    fn new(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> ValuesStruct {
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

    fn slice(&self, index: usize) -> &[i32] {
        if index > self.data.len() {
            panic!(
                "Максимальная длина: {}, ваш индекс: {}",
                self.data.len(),
                index
            )
        } else {
            &self.data[..index]
        }
    }

    fn trim(&self, left_index: usize, right_index: usize) -> &[i32] {
        if right_index > self.data.len() {
            panic!(
                "Максимальная длина: {}, ваш индекс: {}",
                self.data.len(),
                right_index
            );
        } else if left_index > right_index {
            panic!(
                "Первый индекс не может быть больше второго. Первый индекс: {}, второй индекс: {}",
                left_index, right_index
            )
        } else {
            &self.data[left_index..right_index]
        }
    }

    fn increase_one(&mut self, delta: i32) -> &[i32] {
        for i in 0..self.data.len() {
            self.data[i] += delta;
        }
        &self.data
    }

    fn increase_two(&mut self, delta: i32) -> &[i32] {
        for item in &mut self.data {
            *item += delta
        }
        &self.data
    }

    fn reverse(&mut self) -> &[i32] {
        for i in 0..self.data.len() / 2 {
            let leng = self.data.len() - i - 1;
            self.data.swap(i, leng);
        }
        &self.data
    }

    fn shift(&mut self, k: usize) -> &[i32] {
        self.data.rotate_left(k);
        &self.data
    }

    fn change(&mut self, min: i32) -> &[i32] {
        for item in &mut self.data {
            if *item < min {
                *item = min
            };
        }
        &self.data
    }
}

fn cut(raw: &[i32]) -> &i32 {
    if raw.is_empty() {
        panic!("Входные данные пустые!");
    };
    let mut tmp_index: usize = 0;
    let mut tmp = raw[0];
    for i in 0..raw.len() {
        if raw[i] > tmp {
            tmp = raw[i];
            tmp_index = i;
        }
    }
    &raw[tmp_index]
}

fn even(raw: &mut [i32]) -> &[i32] {
    for i in 0..raw.len() {
        if raw[i] % 2 == 0 {
            raw[i] = 0;
        }
    }
    raw
}

fn equality(one: &[i32], two: &[i32]) -> bool {
    if one.len() != two.len() {
        return false;
    }
    for x in 0..one.len() {
        if one[x] != two[x] {
            return false;
        }
    }
    true
}

fn main() {
    let mut data1 = ValuesStruct::new(1, 2, 3, 4, 5, 6);
    println!("1. {:?},", data1);
    println!("2. {:?}", data1.search_for_index(5));
    println!("3. {:?}", data1.slice(6));
    println!("4. {:?}", data1.trim(1, 5));
    println!("5. {:?}", data1.increase_one(10));
    println!("6. {:?}", data1.increase_two(10));
    println!("7. {:?}", data1.reverse());
    println!("8. {:?}", data1.shift(1));
    println!("9. {:?}", data1.change(22));
    println!("10. {:?}", cut(&[1, 3, 2, 2, 10, 13, 1, 31, 2, 1000, 123]));
    println!("11. {:?}", even(&mut [2, 3, 4, 5, 6, 8, 10]));
    println!("12. {:?}", equality(&[1, 1, 1, 1, 1], &[1, 1, 1, 1, 1]));
}
