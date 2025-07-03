use lite_auto_dsl::{import, render};

fn main() {
    let package = "collider";
    let result = import("engine/fyrox-impl/src/scene/collider.rs", package);
    render(
        format!("api/fyrox-lite/src/generated/lite_{}.rs", package),
        result,
    );
}
