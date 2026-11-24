// rust-track: handwritten task-2697
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2697;

pub fn lending_first_byte_run_ledger(input: &[u8]) -> Vec<(u32, usize)> {
    let observations: Vec<u32> = {
        trait Lending {
            type Item<'a>
            where
                Self: 'a;
            fn item(&self) -> Self::Item<'_>;
        }

        struct Window<'a>(&'a [u8]);
        impl Lending for Window<'_> {
            type Item<'a>
                = &'a [u8]
            where
                Self: 'a;
            fn item(&self) -> Self::Item<'_> {
                self.0
            }
        }
        input
            .chunks(3)
            .map(|raw| {
                let win = Window(raw);
                let s = win.item();
                u32::from(s[0])
            })
            .collect()
    };
    let mut runs: Vec<(u32, usize)> = Vec::new();
    for value in observations {
        if let Some(last) = runs.last_mut().filter(|last| last.0 == value) {
            last.1 += 1;
        } else {
            runs.push((value, 1));
        }
    }
    runs
}
