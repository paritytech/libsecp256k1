.PHONY: gen

gen:
	cd gen/ecmult && cargo run > ../../src/ecmult/const.rs.new
	mv src/ecmult/const.rs.new src/ecmult/const.rs
