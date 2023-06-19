fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon_with_id("assets/moon.ico", "1")
            .set("InternalName", "moonish.exe");
        res.compile().unwrap();
    }
}
