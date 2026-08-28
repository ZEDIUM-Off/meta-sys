// normalize-stderr-test: "\n$" -> ""
fn too_many_arguments(first: u8, second: u8, third: u8, fourth: u8, fifth: u8, sixth: u8) {
    let _ = (first, second, third, fourth, fifth, sixth);
}
fn too_many_lines() {
    let value = 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let value = value + 1;
    let _ = value;
}

fn helper_01() {}
fn helper_02() {}
fn helper_03() {}
fn helper_04() {}
fn helper_05() {}
fn helper_06() {}
fn helper_07() {}
fn helper_08() {}
fn helper_09() {}
fn helper_10() {}
fn helper_11() {}

fn main() {
    too_many_arguments(1, 2, 3, 4, 5, 6);
    too_many_lines();
}
