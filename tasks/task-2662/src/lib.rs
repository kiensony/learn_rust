#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn lending_first_byte_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
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
            .chunks_exact(3)
            .map(|raw| {
                let win = Window(raw);
                let s = win.item();
                u32::from(s[0])
            })
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
