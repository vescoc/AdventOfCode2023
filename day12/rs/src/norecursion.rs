use std::collections::HashMap;

pub fn arrangements(line: &[u8], groups: &[usize]) -> u64 {
    let mut memoize = HashMap::with_capacity(1_024);
    let mut stack = vec![(line, groups, vec![])];

    while let Some((line, groups, list)) = stack.pop() {
        let key = (line, groups);

        let r = if let Some(r) = memoize.get(&key) {
            *r
        } else if line.is_empty() && groups.is_empty() {
            1
        } else if groups.iter().sum::<usize>() > line.len() {
            0
        } else if let Some(&head) = groups.iter().next() {
            let mut l = line;
            while !l.is_empty() && l[0] == b'.' {
                l = &l[1..];
            }

            if head > l.len() {
                0
            } else {
                // case 1. prefix in group
                let case1 = if l.iter().take(head).all(|&c| c != b'.')
                    && l.get(head).unwrap_or(&b'.') != &b'#'
                {
                    let (l, g) = (l.get(head + 1..).unwrap_or(&[]), &groups[1..]);
                    if let Some(value) = memoize.get(&(l, g)) {
                        *value
                    } else {
                        let mut list = list.clone();
                        list.push(key);

                        stack.push((l, g, list));
                        0
                    }
                } else {
                    0
                };

                // case 2. prefix not in group
                let case2 = if l[0] == b'?' {
                    let (l, g) = (l.get(1..).unwrap_or(&[]), groups);
                    if let Some(value) = memoize.get(&(l, g)) {
                        *value
                    } else {
                        let mut list = list.clone();
                        list.push(key);

                        stack.push((l, g, list));
                        0
                    }
                } else {
                    0
                };

                case1 + case2
            }
        } else {
            u64::from(!line.contains(&b'#'))
        };

        memoize.insert(key, r);

        if r > 0 {
            for key in list {
                memoize
                    .entry(key)
                    .and_modify(|value| *value += r)
                    .or_insert(r);
            }
        }
    }

    *memoize.get(&(line, groups)).unwrap()
}
