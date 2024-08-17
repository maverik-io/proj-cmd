proj() {
  returned=$(proj-cmd $@)

  if [[ $returned == x\ * ]]; then
    eval ${returned:2}
  else
    echo $returned
  fi

}

