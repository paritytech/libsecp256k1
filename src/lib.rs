#[macro_use]
mod field;
#[macro_use]
mod group;
mod scalar;
mod ecmult;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
