fn main() {
    let om_mat: f32 = 907.0;
    let tm_mat: f32 = 1050.0;
    let om_fsc: f32 = 918.0;
    let tm_fsc: f32 = 1100.0;
    let div_1: f32 = om_mat/tm_mat;
    let div_2: f32 = om_fsc/tm_fsc;
    let perc_mat = div_1 * 100.0;
    let perc_fsc = div_2 * 100.0;
    println!("Percentage (Matric) = {}", perc_mat);
    println!("Percentage (F.Sc) = {}", perc_fsc);
}