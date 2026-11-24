#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn lending_first_byte_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
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
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
