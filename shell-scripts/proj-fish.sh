
function proj
    set returned (proj-cmd $argv)
    
    if string match -r '^x\ ' $returned
        eval (string sub -s 3 $returned)
    else
        echo $returned
    end
end
