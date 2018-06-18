use types::{c_int, c_long, SignedValue, Value};

extern "C" {
    pub fn rb_int2inum(num: SignedValue) -> Value;
    pub fn rb_num2int(num: Value) -> c_int;
    pub fn rb_num2long(num: Value) -> c_long;
}
