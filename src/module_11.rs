pub mod file_io {
    use std::fs::{File, OpenOptions, remove_file};
    use std::io::{BufRead, BufReader, BufWriter, Read, Error, Write};
    use std::path::Path;

    pub fn introduction_to_file_io() -> Result<(), Error> {
        // let mut foo_str = String::new();
        // let file_path = Path::new("foo.txt");
        //
        // // put the contents of the foo.txt file into a string
        // let mut foo_file = File::open(&file_path)?;
        // foo_file.read_to_string(&mut foo_str)?;

        let file_path = Path::new("foo.txt");
        let mut foo_file = File::open(&file_path)?;

        let buffer = BufReader::new(foo_file);
        for line in buffer.lines() {
            println!("Current line: {}", line?);
        }

        Ok(())
    }

    static TEXT: &str = "Lorem ipsum dolor sit amet consectetur adipisicing elit. Maxime mollitia,
molestiae quas vel sint commodi repudiandae consequuntur voluptatum laborum
numquam blanditiis harum quisquam eius sed odit fugiat iusto fuga praesentium
optio, eaque rerum! Provident similique accusantium nemo autem. Veritatis
obcaecati tenetur iure eius earum ut molestias architecto voluptate aliquam
nihil, eveniet aliquid culpa officia aut! Impedit sit sunt quaerat, odit,
tenetur error, harum nesciunt ipsum debitis quas aliquid. Reprehenderit,
quia. Quo neque error repudiandae fuga? Ipsa laudantium molestias eos
sapiente officiis modi at sunt excepturi expedita sint? Sed quibusdam
recusandae alias error harum maxime adipisci amet laborum. Perspiciatis
minima nesciunt dolorem! Officiis iure rerum voluptates a cumque velit";

    fn create_then_remove(path: &str) -> Result<(), Error> {
        File::create(path);
        remove_file(path);

        Ok(())
    }

    fn write_str_buffered(path: &str, to_write: &str) -> Result<(), Error> {
        let writeme = File::create(path)?;
        let mut buffered_writer = BufWriter::new(writeme);

        let temp_str = to_write.to_string();
        for line in temp_str.lines() {
            writeln!(buffered_writer, "{}", line);
        }

        Ok(())
    }

    fn append_str(path: &str, to_append: &str) -> Result<(), Error> {
        let mut writeme = OpenOptions::new().append(true).open(path)?;

        write!(writeme, "\n\n APPENDING... \n\n {}", to_append).map(|_| ())
    }

    fn write_str_glob(path: &str, to_write: &str) -> Result<(), Error> {
        let mut writeme = File::create(path)?;

        write!(writeme, "{}", to_write).map(|_| ())
    }

    fn read_contents_glob(path: &str) -> Result<String, Error> {
        let mut file_text = String::new();
        let mut readme = File::open(path)?;

        readme.read_to_string(&mut file_text).map(|_| file_text)
    }

    fn read_contents_buffered(path: &str) -> Result<String, Error> {
        let mut file_text = String::new();
        let readme = File::open(path)?;

        let buffer = BufReader::new(readme);
        for maybe_line in buffer.lines() {
            file_text.push_str(maybe_line?.as_str());
        }

        Ok(file_text)
    }

    pub fn demo_file_io() {
        // Reading file contents into a string
        match read_contents_glob("readme.txt") {
            Ok(file_contents) => println!("Read contents successfully!\n\n {}", file_contents),
            Err(err) => print!("Error reading file contents: {:?}\n\n", err)
        }

        // Reading buffered file contents into a string line by line
        match read_contents_buffered("readme.txt") {
            Ok(file_contents) => println!("Read contents successfully!\n\n {}", file_contents),
            Err(err) => print!("Error reading file contents: {:?}\n\n", err)
        }

        // Writing contents of a string to a file
        match write_str_glob("writeme.txt", TEXT) {
            Ok(_) => println!("Wrote contents successfully!\n\n"),
            Err(err) => print!("Error writing file contents: {:?}\nn", err)
        }

        // Appending to a file
        match append_str("writeme.txt", TEXT) {
            Ok(_) => println!("Appended contents successfully!\n\n"),
            Err(err) => print!("Error appending file contents: {:?}\nn", err)
        }

        // Buffered writing
        match write_str_buffered("writeme.txt", TEXT) {
            Ok(_) => println!("Wrote bufered contents successfully!\n\n"),
            Err(err) => print!("Error writing buffered file contents: {:?}\n\n", err)
        }

        // Creating and then removing a file to demonstrate deletion
        match create_then_remove("removeme.txt") {
            Ok(_) => println!("Removed file!\n\n"),
            Err(err) => print!("Error Creating/Removing file: {:?}\n\n", err)
        }
    }
}