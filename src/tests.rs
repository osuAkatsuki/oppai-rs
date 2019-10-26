use libc::{c_double, c_int, size_t};
use std::{env::current_dir, path::PathBuf, process::Command};

const ERROR_MARGIN: f32 = 0.02;

/// Test structures.
/// See https://github.com/Francesco149/oppai-ng/blob/71103a07954b403bc502120a4a752574491ab24b/test/test_suite.c#L15
#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct score {
    mode: c_int,
    id: c_int,
    max_combo: c_int,
    n300: c_int,
    n100: c_int,
    n50: c_int,
    nmiss: c_int,
    mods: c_int,
    pp: c_double,
}

extern "C" {
    fn size() -> size_t;
    fn tests() -> *const score;
}

lazy_static! {
    static ref ARTIFACTS_PATHBUF: PathBuf = {
        let mut r = current_dir().expect("Must find a path");
        r.push("tests");
        r.push("artifacts");
        r
    };
    static ref ARTIFACTS_PATH: String = ARTIFACTS_PATHBUF
        .to_str()
        .expect("It's a valid string!")
        .to_owned();
    static ref SUITE_URL_PATH: String = {
        let mut r = current_dir().expect("Must find a path");
        r.push("oppai");
        r.push("test");
        r.push("suite_url");
        r.to_str().expect("It's valid!").to_owned()
    };
}

fn run_command(mut v: Command) {
    let r = v.spawn().expect("RUN!").wait().expect("Good");
    if !r.success() {
        panic!("Download failed")
    }
}

// Download artifacts if needed.
// We use wget for linux (^3^)
#[cfg(target_os = "linux")]
fn prepare_artifacts() {
    let tests_archive_path: PathBuf = {
        let mut r = ARTIFACTS_PATHBUF.clone();
        r.push("test_suite_2019-02-19.tar.gz");
        r
    };
    if tests_archive_path.exists() {
        let mut rm = Command::new("sh");
        rm.args(&["-c", &format!("rm -rf \"{}/*\"", *ARTIFACTS_PATH)]);
        run_command(rm);
    }

    println!("downloading test artifacts...");

    // Now we download the archive
    let mut download = Command::new("sh");
    download.args(&[
        "-c",
        &format!(
            "curl $(cat {}) -L -o \"{}/test_suite_2019-02-19.tar.gz\"",
            *SUITE_URL_PATH, *ARTIFACTS_PATH
        ),
    ]);
    run_command(download);

    // ...and unpack it
    let mut unpack = Command::new("sh");
    unpack.args(&[
        "-c",
        &format!(
            "tar -zx -C \"{}\" -f \"{}/test_suite_2019-02-19.tar.gz\"",
            *ARTIFACTS_PATH, *ARTIFACTS_PATH
        ),
    ]);
    run_command(unpack);
}

lazy_static! {
    static ref MUST_BE_PREPARED: () = prepare_artifacts();
}

fn run_single_test(score: &score) -> Result<(), Box<dyn std::error::Error>> {
    use std::convert::TryInto;

    // println!("{:?}", score);
    // println!("{}/test_suite/{}.osu", *ARTIFACTS_PATH, score.id);

    let accuracy = if score.mode == 0 {
        (score.n300 * 6 + score.n100 * 2 + score.n50) as f32
            / ((score.n300 + score.n100 + score.n50 + score.nmiss) * 6) as f32
            * 100.0
    } else {
        (score.n300 * 2 + score.n100 * 1) as f32
            / ((score.n300 + score.n100 + score.n50 + score.nmiss) * 2) as f32
            * 100.0
    };

    let oppai = {
        let mut p = crate::Oppai::new(std::path::Path::new(&format!(
            "{}/test_suite/{}.osu",
            *ARTIFACTS_PATH, score.id
        )))?;
        p.combo(crate::Combo::non_fc(
            score.max_combo as u32,
            score.nmiss as u32,
        ))?
        .accuracy(accuracy)?
        .mode(score.mode.try_into()?)?
        .mods(crate::Mods::from_bits(score.mods).ok_or(crate::Error::CannotConvertMode)?);
        p
    };

    let map_max_combo = oppai.max_combo();

    let (pp, stars) = oppai.run();

    let margin = ERROR_MARGIN
        * (score.pp as f32)
        * (if pp < 100.0 {
            3.0
        } else if pp < 200.0 {
            2.0
        } else if pp < 300.0 {
            1.5
        } else {
            1.0
        });

    assert!((pp - (score.pp as f32)).abs() < margin);

    Ok(())
}

#[test]
fn run_tests() {
    *MUST_BE_PREPARED;
    let ptr = unsafe { tests() };
    for i in 0..unsafe { size() } {
        let t = unsafe { *ptr.offset(i as isize) };
        run_single_test(&t).expect(&format!("Case {}", i));
    }
}
