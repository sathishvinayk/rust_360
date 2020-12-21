#[derive(PartialEq, Debug)]
enum MachineWithLengthyLetters<T> {
    Rotor(T),
    Engine
}

#[derive(Debug, PartialEq)]
enum Machine2<'a, T> {
    Rotor(&'a T),
    Engine
}

type Machine<T> = MachineWithLengthyLetters<T>;

#[cfg(test)]
mod testingbaba {
    use super::*;

    #[test]
    fn signed_type() {
        let machine: Machine<i8> = Machine::Rotor(12);

        assert_eq!(machine, Machine::Rotor(12));
        debug_assert_eq!(machine, Machine::Rotor(12));
    }

    #[test]
    fn unsigned_type() {
        let machine: Machine<usize> = Machine::Rotor(12000_0000);

        assert_eq!(machine, Machine::Rotor(12000_00_00));
        debug_assert_eq!(machine, Machine::Rotor(12000_00_00));
    }

    #[test]
    fn pass_integer_reference() {
        let value = Machine2::Rotor(&2);

        assert_eq!(value, Machine2::Rotor(&2));
    }

    #[test]
    fn pass_str_reference() {
        let value = Machine2::Rotor(&"string value");

        assert_eq!(value, Machine2::Rotor(&"string value"));
    }
}

fn main() {

}
