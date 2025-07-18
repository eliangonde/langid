# LangId
This is a rust conversion of the [py3langid](https://github.com/adbar/py3langid) library.

This is only a deployment of the library. For training use the original library.

```rs
use langid::Model;

fn main() {
    let model = Model::load().unwrap();
    //model.set_langs(Some(vec![...])).unwrap();

    let text = "This text is in English.";

    let classification = model.classify(text).unwrap();
    println!("{:?}", classification);
}
```
