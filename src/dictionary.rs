#![allow(dead_code)]

pub struct Dictionary<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K, V> Dictionary<K, V>
where
    K: std::cmp::PartialEq + Copy,
    V: Copy,
{
    pub fn new() -> Dictionary<K, V> {
        return Dictionary {
            keys: Vec::<K>::new(),
            values: Vec::<V>::new(),
        };
    }

    fn get_key_position(&self, key: &K) -> Option<usize> {
        let position = self.keys.iter().position(|item| item == key);

        return position;
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let position = self.get_key_position(&key);

        if (position.is_some()) {
            let unwrapped_position = position.unwrap();

            return Some(&self.values[unwrapped_position]);
        }

        return None;
    }

    pub fn set(&mut self, key: K, value: V) {
        let position = self.get_key_position(&key);

        if (position.is_some()) {
            let unwrapped_position = position.unwrap();

            self.values[unwrapped_position] = value;
        } else {
            self.keys.push(key);
            self.values.push(value);
        }
    }

    pub fn delete(&mut self, key: K) -> bool {
        let position = self.get_key_position(&key);

        if (position.is_some()) {
            let unwrapped_position = position.unwrap();

            self.keys.remove(unwrapped_position);
            self.values.remove(unwrapped_position);

            return true;
        }

        return false;
    }

    pub fn for_each<F>(&self, callback: F)
    where
        F: Fn(&V) -> (),
    {
        for value in self.values.iter() {
            callback(value);
        }
    }

    pub fn map<F>(&self, callback: F) -> Dictionary<K, V>
    where
        F: Fn(&V) -> V,
    {
        let mut new_dict = Dictionary::<K, V>::new();

        for (index, value) in self.values.iter().enumerate() {
            new_dict.set(self.keys[index], callback(value)); // Copies the key
        }

        return new_dict;
    }

    pub fn filter<F>(&self, predicate: F) -> Dictionary<K, V>
    where
        F: Fn(&V) -> bool,
    {
        let mut new_dict = Dictionary::<K, V>::new();

        for (index, value) in self.values.iter().enumerate() {
            if (predicate(value) == true) {
                new_dict.set(self.keys[index], *value); // Copies the key and dereferenced value
            }
        }

        return new_dict;
    }
}
