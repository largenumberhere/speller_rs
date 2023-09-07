mod inner;
mod simple_hashset;
mod simple_linked_list;
mod manual_destructor;

/// Safety:
///
/// word
/// - must be null terimated,
/// - must be a single contiguous allocation,
/// - must not be mutated while this function is called
#[no_mangle]
extern "C" fn check(word: *const std::ffi::c_char) -> bool{
    if word.is_null(){
        println!("invalid NULL string was given to load function!");
        return false;
    }
    let str = unsafe {  std::ffi::CStr::from_ptr(word) };
    let string = str.to_owned();
    let string = string.into_string();
    let string = match string {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to convert chars to utf8 in check {:?}",e);
            return false;
        }
    };

    inner::check(string)
}

#[no_mangle]
extern "C" fn unload() -> bool{
    inner::unload()
}

/// Safety:
///
/// Dictionary_path
/// - must be null terimated,
/// - must be a single contiguous allocation,
/// - must not be mutated while this function is called
#[no_mangle]
extern "C" fn load(dictionary_path: *const std::ffi::c_char) -> bool{
    if dictionary_path.is_null(){
        println!("invalid NULL string was given to load function!");
        return false;
    }
    let str = unsafe {  std::ffi::CStr::from_ptr(dictionary_path) };
    let string = str.to_owned();
    let string = string.into_string();
    let string = match string {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to convert chars to utf8 in load {:?}",e);
            return false;
        }
    };

    inner::load(string)
}

#[no_mangle]
extern "C" fn size() -> std::ffi::c_uint{
    inner::size() as std::ffi::c_uint
}
