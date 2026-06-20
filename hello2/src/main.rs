fn main() {
    let tamil = "வணக்கம்";
    let hindi = "नमस्ते";

    let regions = [tamil, hindi];
    for region in regions.iter() {
        println!("{}", &region);
    }
}
