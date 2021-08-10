use std::{fs::File, io::BufWriter};

use args::Command;
use file2png::{file_name, get_png_bytes, get_starting_point, hash_vec, str_to_vec};
use regress::Regex;

mod args;

fn main() {
    let args: args::Args = argh::from_env();
    match args.command {
        Command::Store(store) => {
            // Data vector
            let mut data = std::fs::read(&store.input).unwrap();

            // Create the file headers
            let mut size_header = str_to_vec(&format!("###SIZE: {} :EZIS###\n", data.len()));

            let mut name_header =
                str_to_vec(&format!("###NAME: {} :EMAN###\n", file_name(&store.input)));

            let sha_string = hash_vec(&data);

            let mut sha_header = str_to_vec(&format!("###SHA256: {} :652AHS###\n", sha_string));

            let mut version_header = str_to_vec(&format!("###VERSION: {} :NOISREV###\n", "0.1.1"));

            let mut util_header = str_to_vec(&format!(
                "###UTIL:  Decode: https://github.com/meguminloli/file2png#readme  :LITU###\n"
            ));

            let mut endline = str_to_vec("\n");

            data.append(&mut endline);
            data.append(&mut size_header);
            data.append(&mut name_header);
            data.append(&mut sha_header);
            data.append(&mut version_header);
            data.append(&mut util_header);

            for comment in store.comments {
                let mut comment_header =
                    str_to_vec(&format!("###COMMENT: {} :TNEMMOC###\n", comment));
                data.append(&mut comment_header);
            }

            let file = File::create(store.output).unwrap();

            // Calculate the size of the resulting file
            let size = data.len();
            let pixbyte = 8 * 3 / 8;
            let pixels = size / pixbyte;
            let resol = ((pixels as f64).sqrt() + 1.) as usize;
            let pixels = resol * resol;
            let container_size = pixels * pixbyte;
            let bytesdiff = container_size - size;

            // In case that's not exact, add a padding of nulls to the end of file
            if bytesdiff > 0 {
                let mut padding = vec![0u8; bytesdiff];
                data.append(&mut padding);
            } else {
                panic!("Error: unexpected container size.");
            }

            data.append(&mut endline);

            let w = BufWriter::new(file);

            // Setup the encoder
            let mut encoder = png::Encoder::new(w, resol as u32, resol as u32);
            encoder.set_color(png::ColorType::RGB);
            encoder.set_depth(png::BitDepth::Eight);

            // Write the content to the output file
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data).unwrap();
        }
        Command::Restore(restore) => {
            // Get the data from the input file
            let data = get_png_bytes(&restore.input);

            let header = get_starting_point(&data);

            // Check if there is an output file
            let output_file = if let Some(output) = restore.output {
                output
            } else {
                let reg = Regex::new(r"###NAME: (.*?) :EMAN###").unwrap();
                let name = match reg.find(&header) {
                    Some(m) => {
                        let name = m.group(1);
                        match name {
                            Some(range) => &header[range.start..range.end],
                            None => "output.txt",
                        }
                    }
                    None => "output.txt",
                };
                name.to_string()
            };

            // Get the length of the initial file
            let size_regex = Regex::new(r"###SIZE: (.*?) :EZIS###").unwrap();
            let len = if let Some(m) = size_regex.find(&header) {
                match m.group(1) {
                    Some(range) => {
                        if let Ok(number) = &header[range.start..range.end].parse::<usize>() {
                            number.clone()
                        } else {
                            data.len()
                        }
                    }
                    None => data.len(),
                }
            } else {
                data.len()
            };

            let mut initial_data = &data[0..len];

            let sha_regex = Regex::new(r"###SHA256: (.*?) :652AHS###").unwrap();
            if let Some(m) = sha_regex.find(&header) {
                match m.group(1) {
                    Some(range) => {
                        if let Ok(hash_sum) = &header[range.start..range.end].parse::<String>() {
                            let result = hash_vec(initial_data);
                            if result != *hash_sum {
                                panic!("The hash isn't the same as the original hash");
                            }
                        }
                    }
                    None => (),
                }
            }
            std::fs::write(output_file, &mut initial_data).unwrap();
        }
        Command::Info(info) => {
            let data = get_png_bytes(&info.input);
            // Workaround because regex systems works only with valid UTF-8 sequences
            let s = get_starting_point(&data);
            // Check for every possible header
            let re = Regex::new(r"###(NAME|SIZE|SHA256|VERSION|COMMENT): (.+?) :.+###").unwrap();
            for m in re.find_iter(&s) {
                let name = m.group(1).unwrap();
                let inf = m.group(2).unwrap();
                println!(
                    "{}: {}",
                    &s[(name.start)..(name.end)],
                    &s[(inf.start)..(inf.end)]
                );
            }
        }
    }
}
