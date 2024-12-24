//Split text by spaces. Maximum text block length is defined by len.
pub fn split_text(len: usize, text: &str) -> Vec<String> {
    let split_text = text.split(" ").collect::<Vec<&str>>();
    let mut sub_text = Vec::new();

    let mut last_add = 0;

    for i in 0..split_text.len() {
        if i < last_add {
            continue;
        }

        if split_text[i].len() <= len {
            if i < split_text.len() - 1 {
                let mut total_size = split_text[i].len() + 1;
                let mut end = i+1;
                for j in i+1..split_text.len() {
                    if total_size + split_text[j].len() > len {
                        end = j;
                        break;
                    }

                    if j >= split_text.len() - 1 {
                        end = split_text.len();
                        break;
                    }

                    total_size += split_text[j].len() + 1;
                }

                let mut sub_msg = "".to_string();
                for j in i..end {
                    sub_msg += split_text[j];
                    if j < end - 1 {
                        sub_msg += " ";
                    }
                }

                sub_text.push(sub_msg);
                last_add = end;
            } else {
                sub_text.push(split_text[i].to_string());
                last_add = i;
            }
        } else {
            let mut long_text = split_text[i].to_string();
            while long_text.len() > 0 {

                if long_text.len() > len {
                    sub_text.push(long_text[0..len].to_string());
                    long_text.replace_range(0..len, "");
                } else {
                    sub_text.push(long_text);
                    break;
                }
            }
            last_add = i;
        }
    }

    return sub_text;
}
