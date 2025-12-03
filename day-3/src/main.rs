use std::fs;


fn main() {
    match fs::read_to_string("./day-3/assets/input.txt") {
        Ok(contents) => {
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

#[cfg(test)]
mod tests {
}
