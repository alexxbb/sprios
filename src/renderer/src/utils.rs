// f32::clamp is unstable,
pub trait Clip {
    type Output;
    fn clip(self, min: Self::Output, max: Self::Output) -> Self::Output;
}

impl Clip for f32
{
    type Output = f32;

    fn clip(self, min: Self::Output, max: Self::Output) -> Self::Output
    {
        debug_assert!(min <= max);
        let mut x = self;
        if x < min {
            x = min;
        }
        if x > max {
            x = max;
        }
        x
    }
}
