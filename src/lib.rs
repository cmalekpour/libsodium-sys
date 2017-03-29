extern crate libc;

#[cfg(test)]
mod tests {
	#![allow(non_upper_case_globals)]
	#![allow(non_camel_case_types)]
	#![allow(non_snake_case)]
	#![allow(dead_code)]
	mod binding;
	use self::binding::sodium_init;

    #[test]
    fn it_works() {
    	unsafe {
    		sodium_init();
    	}
    	assert!(true)
    }
}
