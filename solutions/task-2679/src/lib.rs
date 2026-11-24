// rust-track: handwritten task-2679
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2679;

pub fn lending_checksum_longest_run(input: &[u8]) -> u64 {
    let samples = observe_lending_checksum(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_lending_checksum(input: &[u8]) -> Vec<u32> {
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
        .chunks(4)
        .map(|raw| {
            let win = Window(raw);
            let s = win.item();
            s.iter().fold(0u32, |a, &x| a ^ u32::from(x))
        })
        .collect()
}
