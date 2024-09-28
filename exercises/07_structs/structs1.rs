struct ColorRegularStruct {
    red: i32,
    green: i32,
    blue: i32,
}

struct ColorTupleStruct(i32, i32, i32);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        let color = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(color.red, 0);
        assert_eq!(color.green, 255);
        assert_eq!(color.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        let color = ColorTupleStruct(0, 255, 0);
        // color(0, 255, 0);

        assert_eq!(color.0, 0);
        assert_eq!(color.1, 255);
        assert_eq!(color.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
