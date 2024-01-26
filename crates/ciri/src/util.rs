use crate::tokens::Token;

pub(crate) fn vec_to_string_list(starting_brace: char, vec: &Vec<Token>) -> String {
    let mut list_string = String::new();
    list_string.push(starting_brace);
    vec.iter().for_each(|tok| {
        list_string.push_str(tok.to_string().as_str());
        list_string.push_str(", ");
    });
    list_string.pop();
    list_string
}

pub(crate) fn vec_to_arr_string(vec: &Vec<Token>) -> String {
    let mut arr_string = vec_to_string_list('[', vec);
    arr_string.push(']');
    arr_string
}

pub(crate) fn vec_to_vec_string(vec: &Vec<Token>) -> String {
    let mut arr_string = vec_to_string_list('{', vec);
    arr_string.push('}');
    arr_string
}