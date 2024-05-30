use std::process::Command;

fn main() {
    // Run the Tailwind CSS build script
    Command::new("npx")
        .args(&[
            "tailwind",
            "-i",
            "./input.css",
            "-o",
            "./public/tailwind.css",
            "--watch",
        ])
        .status()
        .expect("Failed to run Tailwind CSS build script");

    // Tell Cargo to rerun this build script if the styles.css file changes
    println!("cargo:rerun-if-changed=input.css");
}
