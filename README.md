# Json2savefile
This repository is used to convert exist json file into [savefile](https://docs.rs/savefile/) format.

## Limitation
Now this toolkit needs you to change the structs manually. But it is easy for experienced people.

## Guide
Please change th struct definition in `main.rs` named `Hitokoto`.

Also, if you do not want top level array, change the
> let structs: Vec<Hitokoto> = from_reader(file).unwrap();
type inference. For example:
> let structs: Hitokoto = from_reader(file).unwrap();

## Thank you for using
