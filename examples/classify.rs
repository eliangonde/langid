use langid::Model;

fn main() {
    let model = Model::load().unwrap();
    //model.set_langs(Some(vec![...])).unwrap();

    let text = "This text is in English.";

    let classification = model.classify(text).unwrap();
    println!("{:?}", classification);
}
