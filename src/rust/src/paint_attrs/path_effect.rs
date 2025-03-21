use savvy::savvy;

/// PathEffect
///
/// `PathEffect` is a struct that offers a reference to `skia_safe::PathEffect`.
/// You can apply a path effect to shapes via [paint()].
/// Currently only single `PathEffect` can be specified; multiple effects are not supported.
///
/// @details
/// The following effects are available:
///
/// * `no_effect()`: does not apply any path effect. This is the default effect for `paint()`.
/// * `trim(start, end)`: trims the `start` and `end` of the path. `start` and `end` are in the range `[0, 1]`.
/// * `discrete(length, deviation, seed)`: applies discrete path effect.
/// * `dash(intervals, phase)`: applies dash path effect.
/// * `corner(radius)`: applies corner path effect.
/// * `path_1d(path, advance, phase, style)`: applies 1D path effect. `style` can be `"translate"`, `"rotate"`, or `"morph"`.
/// * `path_2d(path, mat)`: applies 2D path effect.
/// * `line_2d(width, mat)`: applies 2D line path effect.
///
/// @seealso
/// [Path Effects | React Native Skia](https://shopify.github.io/react-native-skia/docs/path-effects/)
/// @export
#[savvy]
pub struct PathEffect {
    pub label: String,
    pub effect: Option<skia_safe::PathEffect>,
}

#[savvy]
impl PathEffect {
    fn get_label(&self) -> savvy::Result<savvy::Sexp> {
        let label = &self.label;
        let out = savvy::OwnedStringSexp::try_from_scalar(&label)?;
        Ok(out.into())
    }
}
