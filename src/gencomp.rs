use std::path::Path;
use clap::{Command, CommandFactory};
use clap_complete::Shell;

fn generate_impl(s: Shell, app: &mut Command, appname: &str, outdir: &Path, file: String) {
    let destfile = outdir.join(file);
    std::fs::create_dir_all(destfile.parent().unwrap()).unwrap();
    if let Ok(mut dest) = std::fs::File::create(destfile) {
        clap_complete::generate(s, app, appname, &mut dest);
    }
}

pub(super) fn generate(outdir: &Path) {
    use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};
    let appname = "squirrelt";
    
    // Config構造体を使ってコマンド情報を取得します
    let mut app = squirrelt::Config::command();
    app.set_bin_name(appname);
    
    generate_impl(Bash, &mut app, appname, outdir, format!("bash/{appname}"));
    generate_impl(Elvish, &mut app, appname, outdir, format!("elvish/{appname}"));
    generate_impl(Fish, &mut app, appname, outdir, format!("fish/{appname}"));
    generate_impl(PowerShell, &mut app, appname, outdir, format!("powershell/{appname}"));
    // Zsh向けの補完ファイル名はアンダースコア(_)で始まるのが慣例です
    generate_impl(Zsh, &mut app, appname, outdir, format!("zsh/_{appname}"));
}