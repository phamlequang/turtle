run:
	cargo run

test:
	rustc --version && cargo --version
	RUST_BACKTRACE=1 cargo test --all --verbose -- --test-threads=1 --nocapture

loop:
	for i in 1 2 3 4 5 6 7 8 9 10 ; do \
		printf "\n----- run #$$i -----\n" ; \
    	make test ; \
		status=$$? ; \
		if [[ $$status != 0 ]] ; then \
			exit $$status ; \
		fi ; \
	done ;

update:
	cargo update

.PHONY: run test loop update
