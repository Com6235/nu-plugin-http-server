def "main checks cargo" [] {
    $'⚙  Checking for (ansi default_bold)cargo(ansi reset)' | print
    try { cargo --version | complete } catch {
        error make {
            msg: "Cargo was not found"
        }
    }
    $"✔  (ansi default_bold)Cargo(ansi reset) was found!" | print
}

def "main checks git" [] {
    $'⚙  Checking for (ansi default_bold)git(ansi reset)' | print
    try { git --version | complete } catch {
        error make {
            msg: "Git was not found"
        }
    }
    $"✔  (ansi default_bold)Git(ansi reset) was found!" | print
}

def "main checks" [ --verbose ] {
    main checks cargo
    main checks git
}

def "make-dir deps" [] {
    if ("./deps" | path exists | not $in) {
        mkdir ./deps
    }
}

def "git-checkout tokio" [] {
    let tagName = "tokio-1.44.1"

    git clone --no-checkout --no-hardlinks --single-branch https://github.com/tokio-rs/tokio.git
    cd ./tokio
    git fetch --tags origin
    git checkout -b local $tagName

    $"✔  Checked-out (ansi default_bold)tokio(ansi reset)" | print
}

def "main checkout tokio" [] {

    $"⚙  Checking-out (ansi default_bold)tokio(ansi reset)..." | print
    make-dir deps
    cd ./deps

    if ("./tokio" | path exists) {
        $'⚠  Found existing (ansi default_bold)tokio(ansi reset) dir' | print
        cd ./tokio
        if ((".git" | path exists) and ("Cargo.toml" | path exists)) {
            $'✔  Probably real (ansi default_bold)tokio(ansi reset) repo. Skipping check-out.' | print
        } else {
            $'⚠  Probably not a (ansi default_bold)tokio(ansi reset) repo. Removing and performing check-out..' | print

            cd ..
            rm --recursive --permanent ./tokio
            
            git-checkout tokio
        }
    } else {
        git-checkout tokio
    }
}

def "main patch tokio" [] {
    $"⚙  Patching (ansi default_bold)tokio(ansi reset)..." | print
    let patches = [ "tokio_clone" ]
    let root = (pwd)

    cd ./deps/tokio
    $patches | each {|x|
        try {
            git apply ($root | path join "patches" | path join $'($x).patch')
            $"✔  Applied (ansi default_bold)($x)(ansi reset) patch" | print
        } catch {
            $"⚠  Couldn't apply ($x) patch. Skipping." | print
        }
    }
    $"✔  Applied all patches for (ansi default_bold)tokio(ansi reset)"
}

def "main" [] {
    main checks
    main checkout tokio
    main patch tokio
}