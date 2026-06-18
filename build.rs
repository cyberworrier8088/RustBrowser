// build.rs

// this for windows exe add icon.
// this not more improtent but it make the exe look professional .
// thankyou

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();

        res.set_icon("assets/iconn.svg");

        res.compile().unwrap();
    }
}


/////////////////////////
// End of file
/////////////////////////