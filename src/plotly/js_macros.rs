#[macro_export]
macro_rules! js_object (
	{ $($key: expr => $value: expr),+ } => {
		{
			let mut ret = js::Object::new();
			$(ret.add($key.to_string(), $value);)+
			ret
		}
	};
);

#[macro_export]
macro_rules! js_object_val (
	{ $($key: expr => $value: expr),+ } => {
		js_object!($($key => $value),+).into()
	};
);

#[macro_export]
macro_rules! js_array_val (
	[$($value: expr),+] => {
		js::Value::Array(js_array![$($value),+])
	};
);

#[macro_export]
macro_rules! js_array (
	[$($value: expr),+] => {
		js::Array::new(vec![$($value),+])
	};
);

#[macro_export]
macro_rules! js_literal (
	($value: expr) => {
		js::Value::Literal($value.to_string())
	};
);

#[macro_export]
macro_rules! js_ident (
	($value: expr) => {
		js::Value::Identifier($value.to_string())
	};
);

#[macro_export]
macro_rules! js_str (
	($value: expr) => {
		js::Value::String($value.to_string())
	};
);
