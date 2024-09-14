proj() {
  array=("goto" "create" "make")

  if [[ "${array[*]}" =~ $1 ]]; then
    returned=$(proj-cmd $@)
    if [[ $returned == x\ * ]]; then
      eval ${returned:2}
    else
      echo $returned
    fi
  else
    proj-cmd $@
  fi

}
