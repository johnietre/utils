pub use std::io::{self, prelude::*, stdin};
use std::str::FromStr;

pub fn get_stdin_input<T: FromStr>(prompt: &str) -> io::Result<T> {
    if prompt != "" {
        print!("{prompt}");
    }
    let (stdin, mut line) = (stdin(), String::new());
    loop {
        line.clear();
        stdin.read_line(&mut line)?;
        if let Ok(t) = line.trim().parse() {
            break Ok(t);
        }
    }
}

pub fn get_input<T: FromStr>(prompt: &str, reader: &mut impl BufRead) -> io::Result<T> {
    if prompt != "" {
        print!("{prompt}");
    }
    let mut line = String::new();
    loop {
        line.clear();
        reader.read_line(&mut line)?;
        if let Ok(t) = line.trim().parse() {
            break Ok(t);
        }
    }
}

/// Creates a map containing the arguments.
#[macro_export]
macro_rules! make_map {
    ($map:expr; $($key:expr => $val:expr),+ $(,)?) => {{
        let mut m = $map;
        $(m.insert($key, $val);)+
        m
    }};
    ($($key:expr => $val:expr),+ $(,)?) => {{
        make_map!(make_map!(); $($key => $val,)*)
    }};
    (btree; $($key:expr => $val:expr),+ $(,)?) => {{
        make_map!(make_map!(btree); $($key => $val,)*)
    }};
    () => {
        ::std::collections::HashMap::new()
    };
    (btree) => {
        ::std::collections::BTreeMap::new()
    };
}

/// Creates a set containing the arguments.
#[macro_export]
macro_rules! make_set {
    ($set:expr; $($val:expr),+ $(,)?) => {{
        let mut m = $set;
        $(m.insert($val);)+
        m
    }};
    ($($val:expr),+ $(,)?) => {{
        make_set!(make_set!(); $($val,)*)
    }};
    (btree; $($val:expr),+ $(,)?) => {{
        make_set!(make_set!(btree); $($val,)*)
    }};
    () => {
        ::std::collections::HashSet::new()
    };
    (btree) => {
        ::std::collections::BTreeSet::new()
    };
}

/// Creates a linked list containing the arguments.
#[macro_export]
macro_rules! make_linked_list {
    ($($val:expr),+ $(,)? $(; $ll:expr $(,)?)?) => {{
        let mut ll = make_linked_list!();
        $(ll.push_back($val);)+
        $(
            let mut other = $ll;
            ll.append(&mut other);
        )?
        ll
    }};
    ($front:expr; $($val:expr),+ $(,)? $(; $back:expr $(,)?)?) => {{
        let mut ll = $front;
        $(ll.push_back($val);)+
        $(
            let mut other = $back;
            ll.append(&mut other);
        )?
        ll
    }};
    () => {
        ::std::collections::LinkedList::new()
    };
}

/// Creates a VecDeque containing the arguments.
#[macro_export]
macro_rules! make_vec_deque {
    ($($val:expr),+ $(,)? $(; $dd:expr $(,)?)?) => {{
        let mut dd = make_vec_deque!();
        $(dd.push_back($val);)+
        $(
            let mut other = $dd;
            dd.append(&mut other);
        )?
        dd
    }};
    ($front:expr; $($val:expr),+ $(,)? $(; $back:expr $(,)?)?) => {{
        let mut dd = $front;
        $(dd.push_back($val);)+
        $(
            let mut other = $back;
            dd.append(&mut other);
        )?
        dd
    }};
    () => {
        ::std::collections::VecDeque::new()
    };
}

/// Creates a binary heap containing the arguments.
#[macro_export]
macro_rules! make_heap {
    ($heap:expr; $($val:expr),+ $(,)?) => {{
        let mut m = $heap;
        $(m.push($val);)+
        m
    }};
    ($($val:expr),+ $(,)?) => {{
        make_heap!(make_heap!(); $($val,)*)
    }};
    () => {
        ::std::collections::BinaryHeap::new()
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn make_map() {
        use std::collections::{BTreeMap, HashMap};
        let m = make_map![
            1i32 => 1i32,
            2 => 2, 3=>3,
            4=>4, 5 => 5,
        ];
        let mut want = HashMap::new();
        for i in 1..=5i32 {
            want.insert(i, i);
        }
        assert_eq!(m, want);

        let m = make_map![
            (6..=10i32).zip(6..=10i32).collect::<BTreeMap<_, _>>();
            1=> 1,
            2 => 2, 3 =>3,
            4=>4, 5 => 5,
        ];
        let mut want = BTreeMap::new();
        for i in 1..=10i32 {
            want.insert(i, i);
        }
        assert_eq!(m, want);
    }

    #[test]
    fn make_set() {
        use std::collections::{BTreeSet, HashSet};
        let m = make_set![1i32, 1i32, 2, 2, 3, 3, 4, 4, 5, 5,];
        let mut want = HashSet::new();
        for i in 1..=5i32 {
            want.insert(i);
        }
        assert_eq!(m, want);

        let m = make_set![
            (6..=10i32).collect::<BTreeSet<_>>();
            1, 1,
            2 , 2, 3 ,3,
            4,4, 5 , 5,
        ];
        let mut want = BTreeSet::new();
        for i in 1..=10i32 {
            want.insert(i);
        }
        assert_eq!(m, want);
    }

    #[test]
    fn make_linked_list() {
        use std::collections::LinkedList;
        let ll = make_linked_list![1i32, 2, 3, 4, 5];
        let mut want = LinkedList::new();
        for i in 1..=5i32 {
            want.push_back(i);
        }
        assert_eq!(ll, want);

        let ll = make_linked_list![ll; 6, 7, 8, 9, 10];
        for i in 6..=10 {
            want.push_back(i);
        }
        assert_eq!(ll, want);

        let ll = make_linked_list![-5, -4, -3, -2, -1, 0; ll];
        for i in (-5..=0).rev() {
            want.push_front(i);
        }
        assert_eq!(ll, want);

        let ll = make_linked_list![
            (-5i32..=-1).collect::<LinkedList<_>>();
            0;
            (1..=5).collect(),
        ];
        let want = (-5..=5).collect();
        assert_eq!(ll, want);
    }

    #[test]
    fn make_vec_deque() {
        use std::collections::VecDeque;
        let dd = make_vec_deque![1i32, 2, 3, 4, 5];
        let mut want = VecDeque::new();
        for i in 1..=5i32 {
            want.push_back(i);
        }
        assert_eq!(dd, want);

        let dd = make_vec_deque![dd; 6, 7, 8, 9, 10];
        for i in 6..=10 {
            want.push_back(i);
        }
        assert_eq!(dd, want);

        let dd = make_vec_deque![-5, -4, -3, -2, -1, 0; dd];
        for i in (-5..=0).rev() {
            want.push_front(i);
        }
        assert_eq!(dd, want);

        let dd = make_vec_deque![
            (-5i32..=-1).collect::<VecDeque<_>>();
            0;
            (1..=5).collect(),
        ];
        let want: VecDeque<_> = (-5..=5).collect();
        assert_eq!(dd, want);
    }

    #[test]
    fn make_heap() {
        use std::collections::BinaryHeap;
        let m = make_heap![1i32, 1i32, 2, 2, 3, 3, 4, 4, 5, 5,];
        let mut want = BinaryHeap::new();
        for i in 1..=5i32 {
            want.push(i);
            want.push(i);
        }
        assert_eq!(m.into_sorted_vec(), want.into_sorted_vec());
    }
}
