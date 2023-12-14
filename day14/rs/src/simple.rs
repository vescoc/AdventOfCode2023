pub fn cycle(mut tiles: Vec<u8>, ncols: usize, nrows: usize) -> Vec<u8> {
    // north
    let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];
    for c in 0..ncols {
        let mut state = 0;
        for r in 0..nrows {
            match tiles.get((ncols + 1) * r + c) {
                Some(b'O') => {
                    new_tiles[(ncols + 1) * state + c] = b'O';
                    state += 1;
                }
                Some(b'#') => {
                    new_tiles[(ncols + 1) * r + c] = b'#';
                    state = r + 1;
                }
                _ => {}
            }
        }
    }

    // west
    tiles = new_tiles;
    let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];
    for r in 0..nrows {
        let mut state = 0;
        for c in 0..ncols {
            match tiles.get((ncols + 1) * r + c) {
                Some(b'O') => {
                    new_tiles[(ncols + 1) * r + state] = b'O';
                    state += 1;
                }
                Some(b'#') => {
                    new_tiles[(ncols + 1) * r + c] = b'#';
                    state = c + 1;
                }
                _ => {}
            }
        }
    }

    // south
    tiles = new_tiles;
    let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];
    for c in 0..ncols {
        let mut state = nrows - 1;
        for r in (0..nrows).rev() {
            match tiles.get((ncols + 1) * r + c) {
                Some(b'O') => {
                    new_tiles[(ncols + 1) * state + c] = b'O';
                    state = state.saturating_sub(1);
                }
                Some(b'#') => {
                    new_tiles[(ncols + 1) * r + c] = b'#';
                    state = r.saturating_sub(1);
                }
                _ => {}
            }
        }
    }

    // est
    tiles = new_tiles;
    let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];
    for r in 0..nrows {
        let mut state = ncols - 1;
        for c in (0..ncols).rev() {
            match tiles.get((ncols + 1) * r + c) {
                Some(b'O') => {
                    new_tiles[(ncols + 1) * r + state] = b'O';
                    state = state.saturating_sub(1);
                }
                Some(b'#') => {
                    new_tiles[(ncols + 1) * r + c] = b'#';
                    state = c.saturating_sub(1);
                }
                _ => {}
            }
        }
    }

    new_tiles
}

pub fn load(tiles: &[u8], ncols: usize, nrows: usize) -> usize {
    (0..nrows)
        .flat_map(|r| {
            tiles[(ncols + 1) * r..(ncols + 1) * r + ncols]
                .iter()
                .map(move |&t| if t == b'O' { nrows - r } else { 0 })
        })
        .sum()
}
