
def proj [ ...args ] {
    let returned = (proj-cmd $args)

    if $returned | str starts-with "x " {
        $returned | str substring 2.. | str to-code | eval
    } else {
        echo $returned
    }
}
