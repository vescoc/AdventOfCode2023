use std::collections::HashMap;

type MemoizeKey = (Vec<u8>, Vec<usize>);
type MemoizeValue = u64;

pub fn arrangements(
    line: &[u8],
    groups: &[usize],
) -> MemoizeValue {
    arrangements_internal(line, groups, &mut HashMap::with_capacity(1_024))
}

fn arrangements_internal(
    line: &[u8],
    groups: &[usize],
    memoize: &mut HashMap<MemoizeKey, MemoizeValue>,
) -> MemoizeValue {
    let key = (line.to_owned(), groups.to_owned());
    if let Some(value) = memoize.get(&key) {
        return *value;
    }

    let r = if line.is_empty() && groups.is_empty() {
        1
    } else if groups.iter().sum::<usize>() > line.len() {
        0
    } else if let Some(&head) = groups.iter().next() {
        let mut line = line;
        while !line.is_empty() && line[0] == b'.' {
            line = &line[1..];
        }

        if head > line.len() {
            0
        } else {
            // case 1. prefix in group
            let case1 = if line.iter().take(head).all(|&c| c != b'.')
                && line.get(head).unwrap_or(&b'.') != &b'#'
            {
                arrangements_internal(line.get(head + 1..).unwrap_or(&[]), &groups[1..], memoize)
            } else {
                0
            };

            // case 2. prefix not in group
            let case2 = if line[0] == b'?' {
                arrangements_internal(line.get(1..).unwrap_or(&[]), groups, memoize)
            } else {
                0
            };

            case1 + case2
        }
    } else {
        u64::from(!line.contains(&b'#'))
    };

    memoize.insert(key, r);
    
    r
}
