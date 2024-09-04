#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    value: Vec<T>,
}

impl<T: PartialEq + Clone + Ord> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        let mut s = Self { value: Vec::new() };
        for input in _input {
            s.add(input.clone());
        }
        s
    }

    pub fn contains(&self, _element: &T) -> bool {
        // self.value.iter().any(|ele| ele == _element)
        self.value.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        if !self.contains(&_element) {
            self.value.push(_element);
            self.value.sort();
        }
    }

    pub fn is_empty(&self) -> bool {
        // self.value.len() == 0
        self.value.is_empty()
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.value.iter().all(|x| _other.contains(x))
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        !self.value.iter().any(|x| _other.contains(x))
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        Self::new(
            &self
                .value
                .iter()
                .filter(|x| _other.contains(x))
                .cloned()
                .collect::<Vec<T>>(),
        )
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        Self::new(
            &self
                .value
                .iter()
                .filter(|x| !_other.contains(x))
                .cloned()
                .collect::<Vec<T>>(),
        )
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        let ret = &mut self.value.clone();
        ret.extend(_other.value.iter().cloned());
        Self::new(ret)

        // Self::new(
        //     &self
        //         .value
        //         .iter()
        //         .cloned()
        //         .chain(_other.value.iter().cloned())
        //         .collect::<Vec<T>>(),
        // )
    }
}
